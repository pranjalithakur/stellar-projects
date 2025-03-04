#![allow(unused)]
use soroban_sdk::{xdr::ToXdr, Address, Bytes, BytesN, Env};

use crate::storage_types::{DataKey as ContractDataKey, TokenInfo};

soroban_sdk::contractimport!(
    file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);

pub fn create_contract(e: &Env, token_wasm_hash: BytesN<32>) -> Address {
    let mut salt = Bytes::from_array(e, &token_wasm_hash.to_array());
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash)
}

pub fn write_pool_token(e: &Env, pool_token: TokenInfo) {
    let key = ContractDataKey::PoolToken;
    e.storage().instance().set(&key, &pool_token)
}

pub fn read_pool_token(e: &Env) -> TokenInfo {
    let key = ContractDataKey::PoolToken;
    e.storage()
        .instance()
        .get::<ContractDataKey, TokenInfo>(&key)
        .unwrap()
}
