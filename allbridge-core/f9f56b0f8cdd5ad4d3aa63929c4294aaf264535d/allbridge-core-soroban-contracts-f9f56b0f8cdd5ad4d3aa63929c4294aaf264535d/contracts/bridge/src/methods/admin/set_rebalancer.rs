use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::bridge::Bridge;

pub fn set_rebalancer(env: Env, rebalancer: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    Bridge::update(&env, |config| {
        config.rebalancer = rebalancer;

        Ok(())
    })
}
