use proc_macros::{
    extend_ttl_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, SorobanData, SorobanSimpleData)]
#[symbol_key("Pool")]
#[data_storage_type(Instance)]
#[extend_ttl_info_instance]
pub struct Pool {
    pub a: u128,
    pub token: Address,
    pub fee_share_bp: u128,
    pub balance_ratio_min_bp: u128,

    pub d: u128,
    pub token_balance: u128,
    pub v_usd_balance: u128,
    pub reserves: u128,
    pub decimals: u32,
    pub total_lp_amount: u128,
    pub admin_fee_share_bp: u128,
    pub acc_reward_per_share_p: u128,
    pub admin_fee_amount: u128,

    pub can_deposit: bool,
    pub can_withdraw: bool,
}

impl Pool {
    pub fn from_init_params(
        a: u128,
        token: Address,
        fee_share_bp: u128,
        balance_ratio_min_bp: u128,
        admin_fee_share_bp: u128,
        decimals: u32,
    ) -> Self {
        Pool {
            a,
            token,
            fee_share_bp,
            balance_ratio_min_bp,
            admin_fee_share_bp,
            decimals,
            can_deposit: true,
            can_withdraw: true,
            d: 0,
            token_balance: 0,
            v_usd_balance: 0,
            reserves: 0,
            total_lp_amount: 0,
            acc_reward_per_share_p: 0,
            admin_fee_amount: 0,
        }
    }
}
