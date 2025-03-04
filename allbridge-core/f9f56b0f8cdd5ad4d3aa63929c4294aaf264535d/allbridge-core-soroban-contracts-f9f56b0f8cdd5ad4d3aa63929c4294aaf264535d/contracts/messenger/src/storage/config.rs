use proc_macros::{
    extend_ttl_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use shared::{require, Error};
use soroban_sdk::{contracttype, BytesN, Map};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, SorobanData, SorobanSimpleData)]
#[symbol_key("Config")]
#[data_storage_type(Instance)]
#[extend_ttl_info_instance]
pub struct Config {
    pub chain_id: u32,
    pub other_chain_ids: BytesN<32>,
    pub primary_validator_key: BytesN<65>,
    pub secondary_validator_keys: Map<BytesN<65>, bool>,
}

impl Config {
    pub fn assert_chain_id(&self, chain_id: u32) -> Result<(), Error> {
        require!(chain_id == self.chain_id, Error::InvalidChainId);

        Ok(())
    }

    pub fn assert_other_chain_id(&self, chain_id: u32) -> Result<(), Error> {
        require!(chain_id < 32, Error::InvalidOtherChainId);

        let is_supported = self.other_chain_ids.get(chain_id).unwrap_or(0);

        require!(is_supported == 1, Error::InvalidOtherChainId);

        Ok(())
    }

    pub fn assert_primary_validator(&self, public_key: BytesN<65>) -> Result<(), Error> {
        require!(
            self.primary_validator_key == public_key,
            Error::InvalidPrimarySignature
        );

        Ok(())
    }

    pub fn assert_secondary_validator(&self, public_key: BytesN<65>) -> Result<(), Error> {
        self.secondary_validator_keys
            .get(public_key)
            .ok_or(Error::InvalidSecondarySignature)?;

        Ok(())
    }
}
