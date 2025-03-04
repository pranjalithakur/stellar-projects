use crate::gas_oracle;
use shared::{soroban_data::SimpleSorobanData, Error};

use bridge_storage::*;
use soroban_sdk::Env;

pub mod config;
mod data_key;
pub mod message;

pub fn get_gas_oracle_client(env: &Env) -> Result<gas_oracle::Client, Error> {
    let gas_oracle_address = GasOracleAddress::get(env)?.as_address();
    Ok(gas_oracle::Client::new(env, &gas_oracle_address))
}
