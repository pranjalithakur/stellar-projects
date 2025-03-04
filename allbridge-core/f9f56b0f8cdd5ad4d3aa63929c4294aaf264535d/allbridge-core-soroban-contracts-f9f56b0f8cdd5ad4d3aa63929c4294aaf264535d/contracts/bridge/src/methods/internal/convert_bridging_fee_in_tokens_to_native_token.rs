use shared::{consts::CHAIN_ID, soroban_data::SimpleSorobanData, Error, Event};
use soroban_sdk::{token, Address, Env};

use crate::{
    events::BridgingFeeFromTokens,
    storage::{bridge::Bridge, get_gas_oracle_client},
};

pub fn convert_bridging_fee_in_tokens_to_native_token(
    env: &Env,
    user: &Address,
    token_address: &Address,
    fee_token_amount: u128,
) -> Result<u128, Error> {
    if fee_token_amount == 0 {
        return Ok(0);
    }

    let config = Bridge::get(env)?;

    let contract = env.current_contract_address();

    let token = token::Client::new(env, token_address);
    let gas_oracle = get_gas_oracle_client(env)?;

    token.transfer(user, &contract, &(fee_token_amount as i128));

    let bridging_fee_conversion_scaling_factor = config
        .bridging_fee_conversion_factor
        .get(token_address.clone())
        .ok_or(Error::Uninitialized)?;

    let fee =
        bridging_fee_conversion_scaling_factor * fee_token_amount / gas_oracle.get_price(&CHAIN_ID);

    BridgingFeeFromTokens {
        gas: fee,
        fee_token_amount,
    }
    .publish(env);

    Ok(fee)
}
