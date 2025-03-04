use soroban_sdk::{contract, contractimpl, panic_with_error, token, Address, Env, String, Vec};

use crate::{
    admin::{has_admin, read_admin, write_admin},
    approval::{read_approval, read_approval_all, write_approval, write_approval_all},
    balance::{increment_supply, read_supply},
    errors::Error,
    event,
    ext_token::{read_ext_token, write_ext_token},
    interface::TokenizedCertificateTrait,
    owner::{check_owner, read_owner, write_owner},
    storage_types::{ExtTokenInfo, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD},
    token_data::{
        read_amount, read_file_hashes, read_redeem_time, write_amount, write_file_hashes,
        write_redeem_time,
    },
};

#[contract]
pub struct TokenizedCertificate;

#[contractimpl]
impl TokenizedCertificateTrait for TokenizedCertificate {
    fn initialize(e: Env, admin: Address, ext_token_address: Address, ext_token_decimals: u32) {
        if has_admin(&e) {
            panic!("already initialized")
        }
        write_admin(&e, &admin);
        if ext_token_decimals > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }
        write_ext_token(
            &e,
            ExtTokenInfo {
                address: ext_token_address,
                decimals: ext_token_decimals,
            },
        )
    }

    fn set_admin(e: Env, new_admin: Address) {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_admin(&e, &new_admin);
        event::set_admin(&e, admin, new_admin)
    }

    fn mint(e: Env, amount: u32, redeem_time: u64, file_hashes: Vec<String>) -> i128 {
        let admin = read_admin(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let id = read_supply(&e);
        let to = e.current_contract_address();
        write_amount(&e, id, amount);
        write_redeem_time(&e, id, redeem_time);
        write_file_hashes(&e, id, file_hashes);
        write_owner(&e, id, Some(to.clone()));
        increment_supply(&e);

        event::mint(&e, to, id);
        id
    }

    fn transfer(e: Env, from: Address, to: Address, id: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&e, &from, id);

        write_owner(&e, id, Some(to.clone()));
        event::transfer(&e, from, to, id);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, id: i128) {
        spender.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&e, &from, id);

        if read_approval_all(&e, from.clone(), spender.clone()) || spender == read_approval(&e, id)
        {
            write_approval(&e, id, None);

            write_owner(&e, id, Some(to.clone()));

            event::transfer(&e, from, to, id);
        } else {
            panic_with_error!(&e, Error::NotAuthorized)
        }
    }

    fn appr(e: Env, owner: Address, operator: Address, id: i128) {
        owner.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&e, &owner, id);

        write_approval(&e, id, Some(operator.clone()));
        event::approve(&e, operator, id);
    }

    fn appr_all(e: Env, owner: Address, operator: Address, approved: bool) {
        owner.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        write_approval_all(&e, owner.clone(), operator.clone(), approved);
        event::approve_all(&e, operator, owner)
    }

    fn get_appr(e: Env, id: i128) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval(&e, id)
    }

    fn is_appr(e: Env, owner: Address, operator: Address) -> bool {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval_all(&e, owner, operator)
    }

    fn pledge(e: Env, from: Address, id: i128) {
        from.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&e, &e.current_contract_address(), id);

        // Transfer USDC from "from" to the contract address
        let ext_token = read_ext_token(&e);
        let client = token::Client::new(&e, &ext_token.address);
        let base_amount = read_amount(&e, id);
        let amount = i128::from(base_amount) * 10i128.pow(ext_token.decimals);
        client.transfer(&from, &e.current_contract_address(), &i128::from(amount));

        // Transfer TC from the contract address to "from"
        write_owner(&e, id, Some(from.clone()));
        event::transfer(&e, e.current_contract_address(), from, id);
    }

    fn redeem(e: Env, to: Address, id: i128) {
        to.require_auth();
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&e, &to, id);
        if e.ledger().timestamp() < read_redeem_time(&e, id) {
            panic_with_error!(&e, Error::NotRedeemable);
        }

        // Transfer USDC from the contract address to "to"
        let ext_token = read_ext_token(&e);
        let client = token::Client::new(&e, &ext_token.address);
        let base_amount = read_amount(&e, id);
        let amount = i128::from(base_amount) * 10i128.pow(ext_token.decimals);
        client.transfer(&e.current_contract_address(), &to, &i128::from(amount));

        // Burn TC
        write_owner(&e, id, None);
        event::burn(&e, to, id);
    }

    fn get_amount(e: Env, id: i128) -> u32 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_amount(&e, id)
    }

    fn get_owner(e: Env, id: i128) -> Address {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_owner(&e, id)
    }

    fn get_file_hashes(e: Env, id: i128) -> Vec<String> {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_file_hashes(&e, id)
    }

    fn get_ext_token(e: Env) -> (Address, u32) {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let ext_token = read_ext_token(&e);
        (ext_token.address, ext_token.decimals)
    }

    fn get_redeem_time(e: Env, id: i128) -> u64 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_redeem_time(&e, id)
    }
}
