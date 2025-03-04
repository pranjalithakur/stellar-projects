use bridge_storage::*;
use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env, Map};

use crate::storage::config::Config;

#[allow(clippy::too_many_arguments)]
pub fn initialize(
    env: Env,
    admin: Address,
    chain_id: u32,
    native_token_address: Address,
    other_chain_ids: BytesN<32>,
    gas_oracle_address: Address,
    primary_validator_key: BytesN<65>,
    secondary_validator_keys: Map<BytesN<65>, bool>,
) -> Result<(), Error> {
    require!(!Config::has(&env), Error::Initialized);

    let config = Config {
        chain_id,
        other_chain_ids,
        primary_validator_key,
        secondary_validator_keys,
    };

    Admin(admin).save(&env);
    GasOracleAddress(gas_oracle_address).save(&env);
    NativeToken(native_token_address).save(&env);

    config.save(&env);

    Ok(())
}
