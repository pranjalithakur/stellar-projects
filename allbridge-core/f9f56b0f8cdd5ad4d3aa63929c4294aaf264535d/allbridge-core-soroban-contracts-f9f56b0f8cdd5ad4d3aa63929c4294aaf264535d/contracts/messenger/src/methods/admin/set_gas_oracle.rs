use bridge_storage::*;
use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, Env};

pub fn set_gas_oracle(env: Env, new_address: Address) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    GasOracleAddress(new_address).save(&env);
    Ok(())
}
