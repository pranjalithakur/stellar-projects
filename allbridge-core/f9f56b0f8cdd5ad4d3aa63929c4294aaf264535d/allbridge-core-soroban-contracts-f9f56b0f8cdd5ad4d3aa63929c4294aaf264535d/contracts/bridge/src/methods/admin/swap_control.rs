use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::bridge::Bridge;

pub fn stop_swap(env: Env) -> Result<(), Error> {
    StopAuthority::require_exist_auth(&env)?;

    Bridge::update(&env, |config| {
        config.can_swap = false;

        Ok(())
    })
}

pub fn start_swap(env: Env) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge::update(&env, |config| {
        config.can_swap = true;

        Ok(())
    })
}

pub fn set_stop_authority(env: Env, stop_authority: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    StopAuthority(stop_authority).save(&env);

    Ok(())
}
