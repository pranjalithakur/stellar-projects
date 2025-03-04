use soroban_sdk::{Address, Env};

use crate::{
    contracts::pool::{self, UserDeposit},
    utils::{float_to_int, float_to_int_sp, int_to_float, CallResult},
};
use crate::utils::SYSTEM_PRECISION;

use super::User;

pub struct Pool {
    pub id: soroban_sdk::Address,
    pub client: pool::Client<'static>,
}

impl Pool {
    pub fn create(
        env: &Env,
        admin: &Address,
        bridge: &Address,
        a: u128,
        token: &Address,
        fee_share_bp: u128,
        balance_ratio_min_bp: u128,
        admin_fee: u128,
    ) -> Pool {
        let id = env.register_contract_wasm(None, pool::WASM);
        let client = pool::Client::new(&env, &id);

        client.initialize(
            &admin,
            &bridge,
            &a,
            &token,
            &fee_share_bp,
            &balance_ratio_min_bp,
            &admin_fee,
        );

        Pool { id, client }
    }

    pub fn d(&self) -> u128 {
        self.client.get_pool().d
    }

    pub fn can_deposit(&self) -> bool {
        self.client.get_pool().can_deposit
    }

    pub fn can_withdraw(&self) -> bool {
        self.client.get_pool().can_withdraw
    }

    pub fn total_lp_amount(&self) -> u128 {
        self.client.get_pool().total_lp_amount
    }

    pub fn bridge(&self) -> Address {
        self.client.get_bridge()
    }

    pub fn stop_authority(&self) -> Address {
        self.client.get_stop_authority()
    }

    pub fn fee_share_bp(&self) -> u128 {
        self.client.get_pool().fee_share_bp
    }

    pub fn balance_ratio_min_bp(&self) -> u128 {
        self.client.get_pool().balance_ratio_min_bp
    }

    pub fn admin_fee_share_bp(&self) -> u128 {
        self.client.get_pool().admin_fee_share_bp
    }

    pub fn admin(&self) -> Address {
        self.client.get_admin()
    }

    pub fn user_deposit(&self, user: &User) -> UserDeposit {
        self.user_deposit_by_id(&user.as_address())
    }

    pub fn user_deposit_by_id(&self, id: &Address) -> UserDeposit {
        self.client.get_user_deposit(&id)
    }

    pub fn claim_rewards(&self, user: &User) -> CallResult {
        self.client
            .try_claim_rewards(&user.as_address())
            .map(Result::unwrap)
            .map_err(Result::unwrap)
    }

    pub fn withdraw(&self, user: &User, withdraw_amount: f64) -> CallResult {
        self.client
            .try_withdraw(&user.as_address(), &float_to_int_sp(withdraw_amount))
            .map(Result::unwrap)
            .map_err(Result::unwrap)
    }

    pub fn withdraw_raw(&self, user: &User, withdraw_amount: u128) -> CallResult {
        self.client
            .try_withdraw(&user.as_address(), &withdraw_amount)
            .map(Result::unwrap)
            .map_err(Result::unwrap)
    }

    pub fn float_to_int(&self, amount: f64) -> u128 {
        float_to_int(amount, self.client.get_pool().decimals)
    }

    pub fn int_to_float(&self, amount: u128) -> f64 {
        int_to_float(amount, self.client.get_pool().decimals)
    }

    pub fn deposit_by_id(&self, user: &Address, deposit_amount: f64) -> CallResult {
        self.client
            .try_deposit(&user, &self.float_to_int(deposit_amount))
            .map(Result::unwrap)
            .map_err(Result::unwrap)
    }

    pub fn deposit(&self, user: &User, deposit_amount: f64) -> CallResult {
        self.deposit_by_id(&user.as_address(), deposit_amount)
    }

    pub fn swap_to_v_usd(&self, user: &User, amount: f64) -> u128 {
        self.client
            .swap_to_v_usd(&user.as_address(), &self.float_to_int(amount), &false)
    }

    pub fn swap_from_v_usd(&self, user: &User, amount: f64, claimable: bool) -> u128 {
        self.client
            .swap_from_v_usd(&user.as_address(), &float_to_int(amount, SYSTEM_PRECISION), &0, &false, &claimable)
    }

    pub fn get_claimable_balance(&self, user: &User) -> u128 {
        self.client
            .get_claimable_balance(&user.as_address())
    }
}
