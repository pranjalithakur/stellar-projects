use shared::{soroban_data::SimpleSorobanData, Error, Event};
use soroban_sdk::{BytesN, Env};

use crate::{
    events::MessageReceived,
    storage::{config::Config, message::Message},
};

pub fn receive_message(
    env: Env,
    message: BytesN<32>,
    primary_signature: BytesN<64>,
    primary_recovery_id: u32,
    secondary_signature: BytesN<64>,
    secondary_recovery_id: u32,
) -> Result<(), Error> {
    let config = Config::get(&env)?;
    let message_hash = env.crypto().keccak256(message.as_ref());
    let primary_validator =
        env.crypto()
            .secp256k1_recover(&message_hash, &primary_signature, primary_recovery_id);
    config.assert_primary_validator(primary_validator)?;

    let secondary_validator =
        env.crypto()
            .secp256k1_recover(&message_hash, &secondary_signature, secondary_recovery_id);
    config.assert_secondary_validator(secondary_validator)?;

    Message::set_received_message(&env, message.clone());
    MessageReceived { message }.publish(&env);

    Ok(())
}
