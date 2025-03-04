use shared::consts::DAY_IN_LEDGERS;
use soroban_sdk::{BytesN, Env};

use crate::storage::data_key::DataKey;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SentMessage;

impl SentMessage {
    const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
    const LIFETIME_THRESHOLD: u32 = Self::BUMP_AMOUNT - DAY_IN_LEDGERS;

    pub fn extend_ttl(env: &Env, key: &DataKey) {
        env.storage()
            .persistent()
            .extend_ttl(key, Self::LIFETIME_THRESHOLD, Self::BUMP_AMOUNT);
    }

    #[inline]
    pub fn set_processed(env: &Env, message: BytesN<32>) {
        let key = DataKey::SentMessage(message);
        env.storage().persistent().set(&key, &true);
        Self::extend_ttl(env, &key);
    }

    #[inline]
    pub fn is_processed(env: &Env, message: BytesN<32>) -> bool {
        let key = DataKey::SentMessage(message);
        if let Some(result) = env.storage().persistent().get(&key) {
            Self::extend_ttl(env, &key);
            result
        } else {
            false
        }
    }
}
