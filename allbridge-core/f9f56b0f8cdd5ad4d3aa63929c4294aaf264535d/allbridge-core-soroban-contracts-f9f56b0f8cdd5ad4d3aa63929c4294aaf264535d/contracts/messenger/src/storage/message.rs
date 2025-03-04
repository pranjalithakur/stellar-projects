use shared::consts::DAY_IN_LEDGERS;
use soroban_sdk::{BytesN, Env};

use crate::storage::data_key::DataKey;

pub struct Message;

const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

impl Message {
    pub fn has_sent_message(env: &Env, message: BytesN<32>) -> bool {
        let key = DataKey::SentMessage(message);
        let result = env.storage().persistent().get::<_, u32>(&key).is_some();
        if result {
            Self::extend_ttl(env, &key);
        }
        result
    }

    pub fn set_sent_message(env: &Env, message: BytesN<32>) {
        let key = DataKey::SentMessage(message);
        let sequence = env.ledger().sequence();
        env.storage().persistent().set(&key, &sequence);
        Self::extend_ttl(env, &key);
    }

    pub fn get_sent_message_sequence(env: &Env, message: BytesN<32>) -> u32 {
        let key = DataKey::SentMessage(message);
        let result = env.storage().persistent().get::<_, u32>(&key);
        if result.is_some() {
            Self::extend_ttl(env, &key);
        }
        result.unwrap_or(0)
    }

    #[allow(dead_code)]
    pub fn has_received_message(env: &Env, message: BytesN<32>) -> bool {
        let key = DataKey::ReceivedMessage(message);
        let result = env.storage().persistent().get::<_, bool>(&key).is_some();
        if result {
            Self::extend_ttl(env, &key);
        }
        result
    }

    pub fn set_received_message(env: &Env, message: BytesN<32>) {
        let key = DataKey::ReceivedMessage(message);
        env.storage().persistent().set(&key, &true);
        Self::extend_ttl(env, &key);
    }

    fn extend_ttl(env: &Env, key: &DataKey) {
        env.storage()
            .persistent()
            .extend_ttl(key, LIFETIME_THRESHOLD, BUMP_AMOUNT);
    }
}
