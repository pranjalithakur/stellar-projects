use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{BytesN, Env};

use crate::storage::config::Config;

pub fn add_secondary_validator(env: Env, validator_address: BytesN<65>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Config::update(&env, |config| {
        config.secondary_validator_keys.set(validator_address, true);
        Ok(())
    })
}

pub fn remove_secondary_validator(env: Env, validator_address: BytesN<65>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Config::update(&env, |config| {
        config.secondary_validator_keys.remove(validator_address);
        Ok(())
    })
}

pub fn set_primary_validator(env: Env, validator_address: BytesN<65>) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Config::update(&env, |config| {
        config.primary_validator_key = validator_address;
        Ok(())
    })
}
