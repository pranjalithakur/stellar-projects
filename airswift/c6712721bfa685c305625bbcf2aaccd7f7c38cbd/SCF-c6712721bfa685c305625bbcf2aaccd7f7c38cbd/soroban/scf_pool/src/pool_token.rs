use soroban_sdk::{panic_with_error, xdr::ToXdr, Address, Bytes, BytesN, Env, Map};

use crate::{error::Error, storage_types::DataKey as ContractDataKey};

soroban_sdk::contractimport!(
    file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
);

pub fn write_wasm_hash(e: &Env, token_wasm_hash: BytesN<32>) {
    let key = ContractDataKey::PoolTokenWasmHash;
    e.storage().instance().set(&key, &token_wasm_hash);
}

pub fn read_wasm_hash(e: &Env) -> BytesN<32> {
    let key = ContractDataKey::PoolTokenWasmHash;
    e.storage()
        .instance()
        .get::<ContractDataKey, BytesN<32>>(&key)
        .unwrap()
}

pub fn create_contract(e: &Env, token_wasm_hash: BytesN<32>, token_address: &Address) -> Address {
    let mut salt = Bytes::new(e);
    salt.append(&token_address.to_xdr(e));
    let salt = e.crypto().sha256(&salt);
    e.deployer()
        .with_current_contract(salt)
        .deploy(token_wasm_hash)
}

pub fn add_pool_token(e: &Env, ext_token: &Address, pool_token: &Address) {
    let key = ContractDataKey::PoolTokens;
    match e
        .storage()
        .instance()
        .get::<ContractDataKey, Map<Address, Address>>(&key)
    {
        Some(mut tokens) => {
            tokens.set(ext_token.clone(), pool_token.clone());
            e.storage().instance().set(&key, &tokens);
        }
        None => {
            let mut tokens = Map::new(&e);
            tokens.set(ext_token.clone(), pool_token.clone());
            e.storage().instance().set(&key, &tokens);
        }
    }
}

pub fn read_pool_tokens(e: &Env) -> Map<Address, Address> {
    let key = ContractDataKey::PoolTokens;
    match e
        .storage()
        .instance()
        .get::<ContractDataKey, Map<Address, Address>>(&key)
    {
        Some(tokens) => tokens,
        None => Map::new(e),
    }
}

pub fn get_pool_token(e: &Env, ext_token: &Address) -> Address {
    let tokens = read_pool_tokens(e);
    match tokens.get(ext_token.clone()) {
        Some(token) => token,
        None => panic_with_error!(&e, Error::TokenNotSupported),
    }
}

pub fn write_ext_token(e: &Env, pool_token: &Address, ext_token: &Address) {
    let key = ContractDataKey::ExtToken(pool_token.clone());
    e.storage().instance().set(&key, &ext_token);
}

pub fn read_ext_token(e: &Env, pool_token: &Address) -> Address {
    let key = ContractDataKey::ExtToken(pool_token.clone());
    match e.storage().instance().get::<ContractDataKey, Address>(&key) {
        Some(token) => token,
        None => panic_with_error!(&e, Error::TokenNotSupported),
    }
}
