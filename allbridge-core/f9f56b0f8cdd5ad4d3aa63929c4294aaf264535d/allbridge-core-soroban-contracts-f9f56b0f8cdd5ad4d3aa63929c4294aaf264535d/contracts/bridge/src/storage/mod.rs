use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::Env;

use crate::other_contracts::gas_oracle;

pub mod another_bridge;
pub mod bridge;
pub mod data_key;
pub mod processed_message;
pub mod sent_message;

pub fn get_gas_oracle_client(env: &Env) -> Result<gas_oracle::Client, Error> {
    let gas_oracle_address = GasOracleAddress::get(env)?.as_address();
    Ok(gas_oracle::Client::new(env, &gas_oracle_address))
}
