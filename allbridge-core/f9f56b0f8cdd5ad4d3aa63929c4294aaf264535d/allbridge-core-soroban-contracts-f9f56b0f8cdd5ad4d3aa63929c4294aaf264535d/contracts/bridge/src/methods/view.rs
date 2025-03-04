use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::{
    another_bridge::AnotherBridge, bridge::Bridge, get_gas_oracle_client,
    processed_message::ProcessedMessage,
};

pub fn has_processed_message(env: Env, message: BytesN<32>) -> Result<bool, Error> {
    Ok(ProcessedMessage::is_processed(&env, message))
}

pub fn has_received_message(env: &Env, message: &BytesN<32>) -> Result<bool, Error> {
    Ok(Bridge::get(env)?
        .get_messenger_client(env)
        .has_received_message(message))
}

pub fn get_pool_address(env: Env, token_address: BytesN<32>) -> Result<Address, Error> {
    Bridge::get(&env)?
        .pools
        .get(token_address)
        .ok_or(Error::NotFound)
}

pub fn get_transaction_cost(env: &Env, chain_id: u32) -> Result<u128, Error> {
    let gas_oracle = get_gas_oracle_client(env)?;
    let gas_usage = GasUsage::get_by_chain(env, chain_id)?;

    Ok(gas_oracle.get_gas_cost_in_native_token(&chain_id, &gas_usage))
}

pub fn get_another_bridge(env: Env, chain_id: u32) -> Result<AnotherBridge, Error> {
    AnotherBridge::get(&env, chain_id)
}

pub fn get_config(env: Env) -> Result<Bridge, Error> {
    Bridge::get(&env)
}
