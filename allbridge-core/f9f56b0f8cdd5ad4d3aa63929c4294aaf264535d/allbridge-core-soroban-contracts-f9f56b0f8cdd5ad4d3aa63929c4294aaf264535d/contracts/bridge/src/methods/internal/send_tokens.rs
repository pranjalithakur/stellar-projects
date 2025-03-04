#![allow(clippy::too_many_arguments)]

use bridge_storage::*;
use shared::{
    consts::CHAIN_ID, require, soroban_data::SimpleSorobanData, utils::hash_message, Error, Event,
};
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, BytesN, Env, IntoVal, Symbol, U256,
};

use crate::{
    events::{ReceiveFee, TokensSent},
    methods::view::get_transaction_cost,
    storage::{another_bridge::AnotherBridge, bridge::Bridge, sent_message::SentMessage},
};

pub fn send_tokens(
    env: &Env,
    amount: u128,
    recipient: &BytesN<32>,
    destination_chain_id: u32,
    receive_token: &BytesN<32>,
    nonce: &U256,
    gas_amount: u128,
    fee_token_amount_in_native: u128,
    sender: &Address,
) -> Result<(), Error> {
    let config = Bridge::get(env)?;

    require!(destination_chain_id != CHAIN_ID, Error::InvalidOtherChainId);

    AnotherBridge::get(env, destination_chain_id)?
        .tokens
        .get(receive_token.clone())
        .ok_or(Error::UnknownAnotherToken)?;

    let message = hash_message(
        env,
        amount,
        recipient,
        CHAIN_ID,
        destination_chain_id,
        receive_token,
        nonce,
    );
    let already_sent = SentMessage::is_processed(env, message.clone());

    require!(!already_sent, Error::TokensAlreadySent);

    SentMessage::set_processed(env, message.clone());

    let messenger = config.get_messenger_client(env);

    let bridge_tx_cost = get_transaction_cost(env, destination_chain_id)?;
    let message_tx_cost = messenger.get_transaction_cost(&destination_chain_id);

    env.authorize_as_current_contract(vec![
        env,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: NativeToken::get(env)?.as_address(),
                fn_name: Symbol::new(env, "transfer"),
                args: (
                    env.current_contract_address(),
                    config.messenger.clone(),
                    message_tx_cost as i128,
                )
                    .into_val(env),
            },
            sub_invocations: vec![env],
        }),
    ]);

    messenger.send_message(&message, &env.current_contract_address());

    require!(
        bridge_tx_cost + message_tx_cost <= gas_amount + fee_token_amount_in_native,
        Error::AmountTooLowForFee
    );

    if gas_amount > 0 {
        let native_token = NativeToken::get_client(env)?;
        native_token.transfer(
            sender,
            &env.current_contract_address(),
            &(gas_amount as i128),
        );
    }

    let total_extra_gas =
        (fee_token_amount_in_native + gas_amount).saturating_sub(bridge_tx_cost + message_tx_cost);

    ReceiveFee {
        message_transaction_cost: message_tx_cost,
        bridge_transaction_cost: bridge_tx_cost,
        extra_gas: total_extra_gas,
    }
    .publish(env);

    TokensSent {
        amount,
        recipient: recipient.clone(),
        destination_chain_id,
        receive_token: receive_token.clone(),
        nonce: nonce.clone(),
    }
    .publish(env);

    Ok(())
}
