use shared::{soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{Address, BytesN, Env};

use crate::storage::bridge::Bridge;

pub fn send_and_swap_to_v_usd(
    env: &Env,
    token: &BytesN<32>,
    user: &Address,
    amount: u128,
) -> Result<u128, Error> {
    let config = Bridge::get(env)?;

    config.pools.get(token.clone()).ok_or(Error::NoPool)?;

    let pool = config.get_pool_client_by_token(env, token.clone())?;

    Ok(pool.swap_to_v_usd(user, &amount, &config.rebalancer.eq(user)))
}
