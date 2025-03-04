use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::error::Error;
use crate::event;
use crate::interface::OfferPoolTrait;
use crate::offer::{change_offer, check_offer, read_offer, write_offer};
use crate::pool_token::{
    add_pool_token, create_contract, get_pool_token, read_ext_token, read_pool_tokens,
    read_wasm_hash, write_ext_token, write_wasm_hash,
};
use crate::storage_types::{Offer, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, vec, Address, BytesN, Env, IntoVal, Map, Val,
};

mod tc {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

#[contract]
pub struct OfferPool;

#[contractimpl]
impl OfferPoolTrait for OfferPool {
    fn initialize(e: Env, admin: Address, token_wasm_hash: BytesN<32>) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        write_wasm_hash(&e, token_wasm_hash);
    }

    fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_administrator(&env)
    }

    fn set_admin(env: Env, new_admin: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_administrator(&env, &new_admin);
        event::set_admin(&env, admin, new_admin);
    }

    fn add_pool_token(e: Env, ext_token: Address) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&e);
        admin.require_auth();

        let tokens = read_pool_tokens(&e);
        if tokens.contains_key(ext_token.clone()) {
            panic_with_error!(&e, Error::TokenExists);
        }

        let ext_client = token::Client::new(&e, &ext_token);

        let token_wasm_hash = read_wasm_hash(&e);
        let pool_token = create_contract(&e, token_wasm_hash, &ext_token);

        e.invoke_contract::<Val>(
            &pool_token,
            &"initialize".into_val(&e),
            vec![
                &e,
                e.current_contract_address().into_val(&e),
                ext_client.decimals().into_val(&e),
                "Pool Token".into_val(&e),
                "SPT".into_val(&e),
            ],
        );

        add_pool_token(&e, &ext_token, &pool_token);
        write_ext_token(&e, &pool_token, &ext_token);
        pool_token
    }

    fn get_pool_tokens(e: Env) -> Map<Address, Address> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_pool_tokens(&e)
    }

    fn deposit(e: Env, from: Address, ext_token: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let pool_token = get_pool_token(&e, &ext_token);

        // Transfer select ext token from "from" to the contract address
        let client = token::Client::new(&e, &ext_token);
        client.transfer(&from, &e.current_contract_address(), &amount);

        // Mint the equal amount number of liquidity tokens to 'from'
        token::StellarAssetClient::new(&e, &pool_token).mint(&from, &amount);
        event::deposit(&e, from, ext_token, pool_token, amount);
    }

    fn withdraw(e: Env, from: Address, pool_token: Address, amount: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let ext_token = read_ext_token(&e, &pool_token);

        // Burn the specified number of liquidity tokens from "from"
        token::Client::new(&e, &pool_token).burn(&from, &amount);

        // Transfer ext token from the contract address to "from"
        token::Client::new(&e, &ext_token).transfer(&e.current_contract_address(), &from, &amount);
        event::withdraw(&e, from, ext_token, pool_token, amount);
    }

    fn create_offer(
        e: Env,
        from: Address,
        offer_id: i128,
        pool_token: Address,
        amount: i128,
        tc_contract: Address,
        tc_id: i128,
    ) {
        if check_offer(&e, offer_id) {
            panic_with_error!(&e, Error::OfferExist);
        } else {
            e.storage()
                .instance()
                .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
            // Transfer the offer amount to the contract address until the offer is accepted or expired.
            let token_client = token::Client::new(&e, &pool_token);
            from.require_auth();
            token_client.transfer(&from, &e.current_contract_address(), &amount);
            write_offer(
                &e,
                offer_id,
                from.clone(),
                pool_token,
                amount,
                tc_contract,
                tc_id,
            );
            event::create_offer(&e, from, offer_id, amount);
        }
    }

    // Cancels an offer and returns the offered amount to the owner. Callable by the admin or offer owner.
    fn expire_offer(e: Env, from: Address, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        match read_offer(&e, offer_id) {
            Some(offer) => {
                if offer.status != 0 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                // check that 'from' matches either the admin or the offer owner
                let admin = read_administrator(&e);
                let offer_from = offer.from;
                if (from != admin) && (from != offer_from) {
                    panic_with_error!(&e, Error::NotAuthorized);
                }

                // transfer the offer amount from the contract address back to the offer owner
                from.require_auth();
                let amount = offer.amount;
                let token_client = token::Client::new(&e, &offer.pool_token);

                token_client.transfer(&e.current_contract_address(), &offer_from, &amount);
                change_offer(&e, offer_id, 1);
                event::expire_offer(&e, from, offer_id);
            }
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    fn get_offer(e: Env, offer_id: i128) -> Offer {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let offer = read_offer(&e, offer_id);
        match offer {
            Some(x) => return x,
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    // On accepting an offer, the offered amount in tokens is transferred from to contract address to 'to' and the TC is transferred to the offer creator.
    fn accept_offer(e: Env, to: Address, offer_id: i128) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        match read_offer(&e, offer_id) {
            Some(offer) => {
                if offer.status != 0 {
                    panic_with_error!(&e, Error::OfferChanged);
                }
                let from = offer.from;
                let amount = offer.amount;
                let tc_contract = offer.tc_contract;
                let tc_id = offer.tc_id;

                let token_client = token::Client::new(&e, &offer.pool_token);
                let tc_client = tc::Client::new(&e, &tc_contract);

                to.require_auth();
                tc_client.transfer(&to, &from, &tc_id);

                token_client.transfer(&e.current_contract_address(), &to, &amount);

                change_offer(&e, offer_id, 2);
                event::accept_offer(&e, to, offer_id);
            }
            None => panic_with_error!(&e, Error::OfferEmpty),
        }
    }

    fn get_ext_token(e: Env, pool_token: Address) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_ext_token(&e, &pool_token)
    }
}
