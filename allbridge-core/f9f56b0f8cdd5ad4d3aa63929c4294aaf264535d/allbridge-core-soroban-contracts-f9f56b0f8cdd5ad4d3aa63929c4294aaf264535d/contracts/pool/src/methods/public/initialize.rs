use bridge_storage::*;
use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{token, Address, Env};

use crate::storage::{bridge_address::Bridge, pool::Pool};

#[allow(clippy::too_many_arguments)]
pub fn initialize(
    env: Env,
    admin: Address,
    bridge: Address,
    a: u128,
    token: Address,
    fee_share_bp: u128,
    balance_ratio_min_bp: u128,
    admin_fee_share_bp: u128,
) -> Result<(), Error> {
    require!(!Pool::has(&env), Error::Initialized);

    let token_client = token::Client::new(&env, &token);
    let decimals = token_client.decimals();

    Pool::from_init_params(
        a,
        token,
        fee_share_bp,
        balance_ratio_min_bp,
        admin_fee_share_bp,
        decimals,
    )
    .save(&env);
    Admin(admin).save(&env);
    Bridge(bridge).save(&env);

    Ok(())
}
