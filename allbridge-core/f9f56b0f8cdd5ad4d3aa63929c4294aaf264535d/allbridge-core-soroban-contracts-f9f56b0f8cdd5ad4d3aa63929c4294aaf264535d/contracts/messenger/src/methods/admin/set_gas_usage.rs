use bridge_storage::*;
use shared::Error;
use soroban_sdk::Env;

pub fn set_gas_usage(env: Env, chain_id: u32, gas_usage: u128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    GasUsage::set(&env, chain_id, gas_usage);

    Ok(())
}
