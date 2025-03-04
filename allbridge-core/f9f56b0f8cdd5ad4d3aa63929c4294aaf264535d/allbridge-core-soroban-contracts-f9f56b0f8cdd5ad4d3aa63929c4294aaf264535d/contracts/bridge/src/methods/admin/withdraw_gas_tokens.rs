use bridge_storage::*;
use shared::Error;
use soroban_sdk::{Address, Env};

pub fn withdraw_gas_tokens(env: Env, sender: Address, amount: u128) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let contract = env.current_contract_address();

    NativeToken::get_client(&env)?.transfer(&contract, &sender, &(amount as i128));

    Ok(())
}
