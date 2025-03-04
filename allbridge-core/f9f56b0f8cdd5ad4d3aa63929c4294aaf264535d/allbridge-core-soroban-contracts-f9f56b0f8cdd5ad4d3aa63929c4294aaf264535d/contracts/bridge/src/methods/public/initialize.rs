use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

use crate::storage::bridge::Bridge;

pub fn initialize(
    env: Env,
    admin: Address,
    messenger: Address,
    gas_oracle: Address,
    native_token: Address,
) -> Result<(), Error> {
    require!(!Bridge::has(&env), Error::Initialized);

    Bridge::init_from(&env, admin, messenger, gas_oracle, native_token);

    Ok(())
}
