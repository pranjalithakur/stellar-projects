use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::bridge::Bridge;

pub fn receive_and_swap_from_v_usd(
    env: &Env,
    token: &BytesN<32>,
    recipient: &Address,
    v_usd_amount: u128,
    receive_amount_min: u128,
    claimable: bool
) -> Result<u128, Error> {
    let config = Bridge::get(env)?;
    let pool = config.get_pool_client_by_token(env, token.clone())?;

    Ok(pool.swap_from_v_usd(
        recipient,
        &v_usd_amount,
        &receive_amount_min,
        &config.rebalancer.eq(recipient),
        &claimable
    ))
}
