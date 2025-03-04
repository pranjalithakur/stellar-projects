use crate::errors::Error;
use crate::storage_types::{
    DataKey, ExternalToken, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD,
};
use soroban_sdk::{panic_with_error, Address, Env};

pub fn read_external_token(env: &Env) -> ExternalToken {
    let key = DataKey::ExternalToken;
    match env
        .storage()
        .persistent()
        .get::<DataKey, ExternalToken>(&key)
    {
        Some(data) => {
            env.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
            data
        }
        None => panic_with_error!(env, Error::InvalidContract),
    }
}

pub fn write_external_token(env: &Env, addr: Address, decimals: u32) {
    let key = DataKey::ExternalToken;
    let ext_token = ExternalToken {
        contract_addr: addr,
        decimals: decimals,
    };
    env.storage().persistent().set(&key, &ext_token);
    env.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
