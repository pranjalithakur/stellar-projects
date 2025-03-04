use bridge_storage::*;
use shared::{
    require, soroban_data::SimpleSorobanData, utils::hash_with_sender_address, Error, Event,
};
use soroban_sdk::{Address, BytesN, Env};

use crate::{
    events::MessageSent,
    methods::view::get_transaction_cost,
    storage::{config::Config, message::Message},
};

pub fn send_message(env: Env, message: BytesN<32>, sender: Address) -> Result<(), Error> {
    sender.require_auth();
    let config = Config::get(&env)?;

    let from_chain_id = message.get(0).ok_or(Error::WrongByteLength)? as u32;
    let to_chain_id = message.get(1).ok_or(Error::WrongByteLength)? as u32;

    config.assert_chain_id(from_chain_id)?;
    config.assert_other_chain_id(to_chain_id)?;

    let message_with_sender = hash_with_sender_address(&env, &message, &sender)?;

    require!(
        !Message::has_sent_message(&env, message_with_sender.clone()),
        Error::HasMessage
    );

    Message::set_sent_message(&env, message_with_sender.clone());
    let native_token = NativeToken::get_client(&env)?;

    let transaction_cost = get_transaction_cost(&env, to_chain_id)?;
    native_token.transfer(
        &sender,
        &env.current_contract_address(),
        &(transaction_cost as i128),
    );

    MessageSent {
        message: message_with_sender,
    }
    .publish(&env);

    Ok(())
}
