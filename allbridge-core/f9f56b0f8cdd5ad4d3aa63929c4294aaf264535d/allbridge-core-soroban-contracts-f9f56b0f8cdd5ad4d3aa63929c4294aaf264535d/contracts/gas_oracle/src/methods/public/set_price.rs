use bridge_storage::Admin;
use shared::Error;
use soroban_sdk::Env;

use crate::storage::chain_data::ChainData;

pub fn set_price(
    env: Env,
    chain_id: u32,
    price: Option<u128>,
    gas_price: Option<u128>,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    ChainData::update_gas_price(&env, chain_id, price, gas_price);

    Ok(())
}
