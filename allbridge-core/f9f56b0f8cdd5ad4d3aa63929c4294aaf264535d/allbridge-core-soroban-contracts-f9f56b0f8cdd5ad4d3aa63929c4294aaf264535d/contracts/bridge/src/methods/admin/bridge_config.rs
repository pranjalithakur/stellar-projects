use bridge_storage::*;
use shared::Error;
use soroban_sdk::{BytesN, Env};

use crate::storage::another_bridge::AnotherBridge;

pub fn add_bridge_token(env: Env, chain_id: u32, token_address: &BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    AnotherBridge::update(&env, chain_id, |another_bridge| {
        another_bridge.tokens.set(token_address.clone(), true);
        Ok(())
    })
}

pub fn remove_bridge_token(
    env: Env,
    chain_id: u32,
    token_address: &BytesN<32>,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    AnotherBridge::update(&env, chain_id, |another_bridge| {
        another_bridge.tokens.set(token_address.clone(), false);
        Ok(())
    })
}

pub fn register_bridge(env: Env, chain_id: u32, bridge_address: BytesN<32>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    AnotherBridge::update_or_default(&env, chain_id, |another_bridge| {
        another_bridge.address = bridge_address;

        Ok(())
    })
}
