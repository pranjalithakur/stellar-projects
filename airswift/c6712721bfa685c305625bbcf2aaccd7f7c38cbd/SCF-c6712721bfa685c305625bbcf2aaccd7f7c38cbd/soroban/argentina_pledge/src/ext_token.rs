use soroban_sdk::Env;

use crate::storage_types::{DataKey, ExtTokenInfo};

pub fn write_ext_token(e: &Env, ext_token: ExtTokenInfo) {
    let key = DataKey::ExtToken;
    e.storage().instance().set(&key, &ext_token)
}

pub fn read_ext_token(e: &Env) -> ExtTokenInfo {
    let key = DataKey::ExtToken;
    e.storage()
        .instance()
        .get::<DataKey, ExtTokenInfo>(&key)
        .unwrap()
}
