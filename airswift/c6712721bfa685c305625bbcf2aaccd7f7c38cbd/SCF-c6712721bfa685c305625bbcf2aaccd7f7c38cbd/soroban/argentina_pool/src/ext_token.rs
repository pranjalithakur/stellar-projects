use soroban_sdk::Env;

use crate::storage_types::{DataKey, TokenInfo};

pub fn write_ext_token(e: &Env, ext_token: TokenInfo) {
    let key = DataKey::ExtToken;
    e.storage().instance().set(&key, &ext_token)
}

pub fn read_ext_token(e: &Env) -> TokenInfo {
    let key = DataKey::ExtToken;
    e.storage()
        .instance()
        .get::<DataKey, TokenInfo>(&key)
        .unwrap()
}
