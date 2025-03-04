use shared::{
    consts::{CHAIN_ID, CHAIN_PRECISION, ORACLE_PRECISION, ORACLE_SCALING_FACTOR},
    Error,
};
use soroban_sdk::Env;

use crate::storage::chain_data::ChainData;

const FROM_ORACLE_TO_CHAIN_SCALING_FACTOR: u128 = 10u128.pow(ORACLE_PRECISION - CHAIN_PRECISION);

pub fn crossrate(env: Env, other_chain_id: u32) -> Result<u128, Error> {
    let this_gas_price = ChainData::get(&env, CHAIN_ID)?;
    let other_gas_price = ChainData::get(&env, other_chain_id)?;

    Ok(other_gas_price.price * ORACLE_SCALING_FACTOR / this_gas_price.price)
}

pub fn get_transaction_gas_cost_in_usd(
    env: Env,
    other_chain_id: u32,
    gas_amount: u128,
) -> Result<u128, Error> {
    let other_gas_price = ChainData::get(&env, other_chain_id)?;

    Ok((other_gas_price.gas_price * gas_amount * other_gas_price.price) / ORACLE_SCALING_FACTOR)
}

pub fn get_gas_cost_in_native_token(
    env: Env,
    other_chain_id: u32,
    gas_amount: u128,
) -> Result<u128, Error> {
    let this_gas_price = ChainData::get(&env, CHAIN_ID)?;
    let other_gas_price = ChainData::get(&env, other_chain_id)?;

    Ok(
        (other_gas_price.gas_price * gas_amount * other_gas_price.price)
            / this_gas_price.price
            / FROM_ORACLE_TO_CHAIN_SCALING_FACTOR,
    )
}

pub fn get_price(env: Env, chain_id: u32) -> Result<u128, Error> {
    ChainData::get(&env, chain_id).map(|chain_data| chain_data.price)
}

pub fn get_gas_price(env: Env, chain_id: u32) -> Result<ChainData, Error> {
    ChainData::get(&env, chain_id)
}
