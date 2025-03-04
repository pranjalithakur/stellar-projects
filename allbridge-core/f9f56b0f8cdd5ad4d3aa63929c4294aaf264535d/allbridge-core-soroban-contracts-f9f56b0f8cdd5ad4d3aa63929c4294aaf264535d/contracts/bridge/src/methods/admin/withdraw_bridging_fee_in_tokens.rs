use bridge_storage::*;
use shared::Error;
use soroban_sdk::{Address, Env};

pub fn withdraw_bridging_fee_in_tokens(
    env: Env,
    sender: Address,
    token_address: Address,
) -> Result<(), Error> {
    Admin::require_exist_auth(&env)?;
    let contract = env.current_contract_address();

    let token = soroban_sdk::token::Client::new(&env, &token_address);
    let to_withdraw = token.balance(&contract);

    if to_withdraw > 0 {
        token.transfer(&contract, &sender, &to_withdraw);
    }

    Ok(())
}
