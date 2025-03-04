use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::bridge::Bridge;

pub fn set_messenger(env: Env, messenger: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge::update(&env, |config| {
        config.messenger = messenger;

        Ok(())
    })
}
