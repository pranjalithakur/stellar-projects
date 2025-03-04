use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::approval::{read_approval, read_approval_all, write_approval, write_approval_all};
use crate::balance::{increment_supply, read_supply};
use crate::errors::Error;
use crate::event;
use crate::interface::TokenizedCertificateTrait;
use crate::metadata::{read_external_token, write_external_token};
use crate::order_info::{read_buyer_address, read_total_amount, write_order_info};
use crate::order_state::{read_paid, update_and_read_expired, write_paid};
use crate::owner::{
    add_vc, check_owner, read_all_owned, read_owner, read_recipient, read_vc, write_owner,
    write_recipient, write_vc,
};
use crate::storage_types::{SplitRequest, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::sub_tc::{read_sub_tc, read_sub_tc_disabled, write_sub_tc, write_sub_tc_disabled};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token, vec, Address, Env, String, Vec,
};

#[contract]
pub struct TokenizedCertificate;

#[contractimpl]
impl TokenizedCertificateTrait for TokenizedCertificate {
    fn initialize(
        e: Env,
        admin: Address,
        buyer_address: Address,
        total_amount: u32,
        end_time: u64,
    ) {
        if has_administrator(&e) {
            panic!("already initialized")
        }

        write_administrator(&e, &admin);
        //write_name(&e, &name);
        //write_symbol(&e, &symbol);
        write_order_info(&e, buyer_address, total_amount, end_time);
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

    fn appr(env: Env, owner: Address, operator: Address, id: i128) {
        owner.require_auth();
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        check_owner(&env, &owner, id);

        write_approval(&env, id, Some(operator.clone()));
        event::approve(&env, operator, id);
    }

    fn appr_all(env: Env, owner: Address, operator: Address, approved: bool) {
        owner.require_auth();
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        write_approval_all(&env, owner.clone(), operator.clone(), approved);
        event::approve_all(&env, operator, owner)
    }

    fn get_appr(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval(&env, id)
    }

    fn is_appr(env: Env, owner: Address, operator: Address) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_approval_all(&env, owner, operator)
    }

    fn amount(env: Env, id: i128) -> u32 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let sub_tc = read_sub_tc(&env, id);
        sub_tc.amount
    }

    fn parent(env: Env, id: i128) -> i128 {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let sub_tc = read_sub_tc(&env, id);
        sub_tc.root
    }

    fn owner(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        read_owner(&env, id)
    }

    fn vc(env: Env, id: i128) -> Vec<String> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_vc(&env, id)
    }

    fn get_all_owned(env: Env, address: Address) -> Vec<i128> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        read_all_owned(&env, address)
    }

    fn is_disabled(env: Env, id: i128) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_sub_tc_disabled(&env, id)
    }

    fn transfer(env: Env, from: Address, to: Address, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        check_owner(&env, &from, id);
        from.require_auth();
        write_owner(&env, id, Some(to.clone()));
        event::transfer(&env, from, to, id);
    }

    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env);
        check_owner(&env, &from, id);
        spender.require_auth();

        if read_approval_all(&env, from.clone(), spender.clone())
            || spender == read_approval(&env, id)
        {
            write_approval(&env, id, None);

            write_owner(&env, id, Some(to.clone()));

            event::transfer(&env, from, to, id);
        } else {
            panic_with_error!(&env, Error::NotAuthorized)
        }
    }

    fn mint_original(env: Env, to: Address, vc: String) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        let id = read_supply(&env);
        if id != 0 {
            panic_with_error!(&env, Error::NotEmpty);
        }
        let amount = read_total_amount(&env);
        write_owner(&env, id, Some(to.clone()));
        write_sub_tc(&env, id, id, amount);
        write_vc(&env, id, vec![&env, vc]);
        write_sub_tc_disabled(&env, id, false);
        increment_supply(&env);

        event::mint(&env, to, id)
    }

    fn burn(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        update_and_read_expired(&env);
        let from = read_owner(&env, id);
        write_owner(&env, id, None);

        event::burn(&env, from, id);
    }

    fn split(env: Env, id: i128, splits: Vec<SplitRequest>) -> Vec<i128> {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if read_sub_tc_disabled(&env, id) {
            // if the TC is disabled, it has already been split
            panic_with_error!(&env, Error::NotPermitted);
        }
        if splits.len() == 0 {
            panic_with_error!(&env, Error::InvalidArgs);
        }
        if update_and_read_expired(&env) {
            panic_with_error!(&env, Error::NotPermitted);
        }
        let owner = read_owner(&env, id);
        owner.require_auth();
        let contract_addr = env.current_contract_address();

        let root = read_sub_tc(&env, id);
        let mut sum = 0;
        for req in splits.clone() {
            sum += req.amount;
        }
        if sum > root.amount {
            panic_with_error!(&env, Error::AmountTooMuch);
        }

        let mut remaining = root.amount;
        let mut new_ids = Vec::new(&env);
        for req in splits.clone() {
            let new_id = read_supply(&env);
            write_sub_tc(&env, new_id, id, req.amount);
            write_sub_tc_disabled(&env, new_id, false);
            write_recipient(&env, new_id, &req.to);
            write_owner(&env, new_id, Some(contract_addr.clone()));
            write_vc(&env, new_id, vec![&env]);
            increment_supply(&env);
            new_ids.push_back(new_id);
            remaining -= req.amount;
        }

        // if root amount > 0, create another sub tc to represent the remaining amount belonging to original owner
        if remaining > 0 {
            let new_id = read_supply(&env);
            write_sub_tc(&env, new_id, id, remaining);
            write_sub_tc_disabled(&env, new_id, false);
            write_owner(&env, new_id, Some(owner.clone()));
            write_vc(&env, new_id, vec![&env]);
            increment_supply(&env);
            new_ids.push_back(new_id);
        }

        // disable the original TC
        write_sub_tc_disabled(&env, id, true);

        event::split(&env, owner, id, new_ids.clone());
        new_ids
    }

    fn redeem(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        if !update_and_read_expired(&env) || !read_paid(&env) || read_sub_tc_disabled(&env, id) {
            panic_with_error!(&env, Error::NotPermitted);
        }

        let owner = read_owner(&env, id);
        owner.require_auth();

        // send funds to owner address
        let sub_tc = read_sub_tc(&env, id);
        let ext_token = read_external_token(&env);
        let client = token::Client::new(&env, &ext_token.contract_addr);
        let amount = i128::from(sub_tc.amount) * 10i128.pow(ext_token.decimals);
        client.transfer(&env.current_contract_address(), &owner, &amount);

        // burn the token
        write_owner(&env, id, None);

        event::redeem(&env, owner, id);
    }

    fn set_external_token_provider(env: Env, contract_addr: Address, decimals: u32) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        write_external_token(&env, contract_addr, decimals);
    }

    fn check_paid(env: Env) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_paid(&env)
    }

    fn check_expired(env: Env) -> bool {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        update_and_read_expired(&env)
    }

    fn recipient(env: Env, id: i128) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_recipient(&env, id)
    }

    fn sign_off(env: Env, id: i128) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let expired = update_and_read_expired(&env);
        let owner = read_owner(&env, id);
        if owner != env.current_contract_address() || read_sub_tc_disabled(&env, id) || expired {
            panic_with_error!(&env, Error::NotPermitted);
        }

        let recipient = read_recipient(&env, id);
        recipient.require_auth();

        write_owner(&env, id, Some(recipient.clone()));

        event::transfer(&env, owner, recipient, id);
    }

    fn pay_off(env: Env, from: Address) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        let paid = read_paid(&env);
        if paid {
            panic_with_error!(&env, Error::NotEmpty);
        }
        let ext_token = read_external_token(&env);
        let client = token::Client::new(&env, &ext_token.contract_addr);
        let base_amount = read_total_amount(&env);
        let amount = i128::from(base_amount) * 10i128.pow(ext_token.decimals);

        if from != read_buyer_address(&env) {
            panic_with_error!(&env, Error::NotAuthorized);
        }
        from.require_auth();
        client.transfer(&from, &env.current_contract_address(), &i128::from(amount));
        write_paid(&env, true);
    }

    fn add_vc(env: Env, id: i128, vc: String) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        let admin = read_administrator(&env);
        admin.require_auth();

        add_vc(&env, id, vc);
    }
}
