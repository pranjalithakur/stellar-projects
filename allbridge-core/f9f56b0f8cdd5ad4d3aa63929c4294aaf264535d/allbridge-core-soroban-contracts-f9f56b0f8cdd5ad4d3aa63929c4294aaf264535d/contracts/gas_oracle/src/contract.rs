use bridge_storage::view::get_admin;
use shared::{utils::extend_ttl_instance, Error};
use soroban_sdk::{contract, contractimpl, Address, Env};

use crate::{
    methods::{
        public::{initialize, set_admin, set_price},
        view::{
            crossrate, get_gas_cost_in_native_token, get_gas_price, get_price,
            get_transaction_gas_cost_in_usd,
        },
    },
    storage::chain_data::ChainData,
};

#[contract]
pub struct GasOracleContract;

#[contractimpl]
impl GasOracleContract {
    pub fn initialize(env: Env, admin: Address) -> Result<(), Error> {
        initialize(env, admin)
    }

    pub fn set_price(
        env: Env,
        chain_id: u32,
        price: Option<u128>,
        gas_price: Option<u128>,
    ) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_price(env, chain_id, price, gas_price)
    }

    pub fn set_admin(env: Env, new_admin: Address) -> Result<(), Error> {
        extend_ttl_instance(&env);

        set_admin(env, new_admin)
    }

    // view

    pub fn get_gas_price(env: Env, chain_id: u32) -> Result<ChainData, Error> {
        get_gas_price(env, chain_id)
    }

    pub fn get_price(env: Env, chain_id: u32) -> Result<u128, Error> {
        get_price(env, chain_id)
    }

    pub fn get_gas_cost_in_native_token(
        env: Env,
        other_chain_id: u32,
        gas_amount: u128,
    ) -> Result<u128, Error> {
        get_gas_cost_in_native_token(env, other_chain_id, gas_amount)
    }

    pub fn get_transaction_gas_cost_in_usd(
        env: Env,
        other_chain_id: u32,
        gas_amount: u128,
    ) -> Result<u128, Error> {
        get_transaction_gas_cost_in_usd(env, other_chain_id, gas_amount)
    }

    pub fn crossrate(env: Env, other_chain_id: u32) -> Result<u128, Error> {
        crossrate(env, other_chain_id)
    }

    pub fn get_admin(env: Env) -> Result<Address, Error> {
        get_admin(env)
    }
}
