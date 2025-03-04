#![allow(clippy::too_many_arguments)]

use shared::{soroban_data::SimpleSorobanData, Error, Event};
use soroban_sdk::{Address, BytesN, Env};

use crate::{
    events::Swapped,
    methods::internal::{receive_and_swap_from_v_usd, send_and_swap_to_v_usd},
    storage::bridge::Bridge,
};

pub fn swap(
    env: Env,
    sender: Address,
    amount: u128,
    token: BytesN<32>,
    receive_token: BytesN<32>,
    recipient: Address,
    receive_amount_min: u128
) -> Result<(), Error> {
    Bridge::get(&env)?.assert_can_swap()?;
    sender.require_auth();

    let v_usd_amount = send_and_swap_to_v_usd(&env, &token, &sender, amount)?;
    let receive_amount = receive_and_swap_from_v_usd(
        &env,
        &receive_token,
        &recipient,
        v_usd_amount,
        receive_amount_min,
        false
    )?;

    Swapped {
        sender,
        recipient,
        send_token: token,
        receive_token,
        send_amount: amount,
        receive_amount,
    }
    .publish(&env);

    Ok(())
}
