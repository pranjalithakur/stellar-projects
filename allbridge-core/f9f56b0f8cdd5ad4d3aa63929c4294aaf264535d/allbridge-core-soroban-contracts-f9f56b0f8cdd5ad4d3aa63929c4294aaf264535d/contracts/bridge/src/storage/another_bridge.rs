use proc_macros::{extend_ttl_info, data_storage_type, SorobanData};
use shared::{consts::DAY_IN_LEDGERS, soroban_data::SorobanData, Error};
use soroban_sdk::{contracttype, BytesN, Env, Map};

use crate::storage::data_key::DataKey;

const BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const LIFETIME_THRESHOLD: u32 = BUMP_AMOUNT - DAY_IN_LEDGERS;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, SorobanData)]
#[data_storage_type(Persistent)]
#[extend_ttl_info(BUMP_AMOUNT, LIFETIME_THRESHOLD)]
pub struct AnotherBridge {
    pub address: BytesN<32>,
    pub tokens: Map<BytesN<32>, bool>,
}

impl AnotherBridge {
    pub fn get(env: &Env, chain_id: u32) -> Result<Self, Error> {
        AnotherBridge::get_by_key(env, &DataKey::OtherBridge(chain_id))
            .map_err(|_| Error::UnknownAnotherChain)
    }

    pub fn update<F>(env: &Env, chain_id: u32, handler: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        let mut object = Self::get(env, chain_id)?;

        handler(&mut object)?;
        object.save_by_key(env, &DataKey::OtherBridge(chain_id));

        Ok(())
    }

    pub fn update_or_default<F>(env: &Env, chain_id: u32, handler: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>,
    {
        let mut object = Self::get(env, chain_id).unwrap_or(AnotherBridge {
            tokens: Map::new(env),
            address: BytesN::from_array(env, &[0; 32]),
        });
        handler(&mut object)?;
        object.save_by_key(env, &DataKey::OtherBridge(chain_id));

        Ok(())
    }
}
