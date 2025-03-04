#![allow(clippy::too_many_arguments)]

use bridge_storage::*;
use shared::{
    consts::CHAIN_ID,
    require,
    soroban_data::SimpleSorobanData,
    utils::{address_to_bytes, hash_message, hash_with_sender},
    Error, Event,
};
use soroban_sdk::{Address, BytesN, Env, U256};

use crate::{
    events::TokensReceived,
    methods::{internal::receive_and_swap_from_v_usd, view::has_received_message},
    storage::{another_bridge::AnotherBridge, bridge::Bridge, processed_message::ProcessedMessage},
};

pub fn receive_tokens(
    env: Env,
    sender: Address,
    amount: u128,
    recipient_address: Address,
    source_chain_id: u32,
    receive_token: BytesN<32>,
    nonce: U256,
    receive_amount_min: u128,
    claimable: bool,
    extra_gas: Option<u128>,
) -> Result<(), Error> {
    sender.require_auth();

    let config = Bridge::get(&env)?;

    config.assert_can_swap()?;

    let another_bridge =
        AnotherBridge::get(&env, source_chain_id).or(Err(Error::SourceNotRegistered))?;

    let recipient_bytes = address_to_bytes(&env, &recipient_address)?;

    let message = hash_message(
        &env,
        amount,
        &recipient_bytes,
        source_chain_id,
        CHAIN_ID,
        &receive_token,
        &nonce,
    );

    let message_with_sender = hash_with_sender(&env, &message, &another_bridge.address);
    let is_processed_message = ProcessedMessage::is_processed(&env, message_with_sender.clone());

    require!(!is_processed_message, Error::MessageProcessed);

    ProcessedMessage::set_processed(&env, message_with_sender.clone());

    require!(
        has_received_message(&env, &message_with_sender)?,
        Error::NoMessage
    );

    let receive_amount = receive_and_swap_from_v_usd(
        &env,
        &receive_token,
        &recipient_address,
        amount,
        receive_amount_min,
        claimable
    )?;

    // pass extra gas from the sender to the recipient
    if let Some(extra_gas) = extra_gas {
        NativeToken::get_client(&env)?.transfer(&sender, &recipient_address, &(extra_gas as i128));
    }

    TokensReceived {
        amount: receive_amount,
        recipient: recipient_bytes,
        nonce,
        message: message_with_sender,
        claimable
    }
    .publish(&env);

    Ok(())
}
