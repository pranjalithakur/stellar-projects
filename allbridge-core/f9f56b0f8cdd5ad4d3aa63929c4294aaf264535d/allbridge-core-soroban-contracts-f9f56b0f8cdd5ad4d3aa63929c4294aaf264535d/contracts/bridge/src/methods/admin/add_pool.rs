use bridge_storage::*;
use shared::{
    consts::{CHAIN_PRECISION, ORACLE_PRECISION},
    soroban_data::SimpleSorobanData,
    Error,
};
use soroban_sdk::{Address, Env};
use shared::utils::address_to_bytes;

use crate::storage::bridge::Bridge;

pub fn add_pool(env: Env, pool: &Address, token_address: &Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge::update(&env, |config| {
        let token_bytes = address_to_bytes(&env, token_address)?;
        config.pools.set(token_bytes.clone(), pool.clone());

        let token = soroban_sdk::token::Client::new(&env, token_address);
        let token_decimals = token.decimals();

        let bridging_fee_conversion_factor =
            10u128.pow(ORACLE_PRECISION - token_decimals + CHAIN_PRECISION);
        let from_gas_oracle_factor = 10u128.pow(ORACLE_PRECISION - token_decimals);

        config
            .bridging_fee_conversion_factor
            .set(token_address.clone(), bridging_fee_conversion_factor);
        config
            .from_gas_oracle_factor
            .set(token_address.clone(), from_gas_oracle_factor);

        Ok(())
    })
}
