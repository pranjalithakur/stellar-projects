use bridge_storage::*;
use proc_macros::{
    extend_ttl_info_instance, data_storage_type, symbol_key, SorobanData, SorobanSimpleData,
};
use shared::{require, soroban_data::SimpleSorobanData, Error};
use soroban_sdk::{contracttype, Address, BytesN, Env, Map};

use crate::other_contracts::{messenger, pool};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, SorobanData, SorobanSimpleData)]
#[symbol_key("Config")]
#[data_storage_type(Instance)]
#[extend_ttl_info_instance]
pub struct Bridge {
    pub messenger: Address,
    pub rebalancer: Address,

    /// precomputed values to divide by to change the precision from the Gas Oracle precision to the token precision
    pub from_gas_oracle_factor: Map<Address, u128>,
    /// precomputed values of the scaling factor required for paying the bridging fee with stable tokens
    pub bridging_fee_conversion_factor: Map<Address, u128>,

    pub pools: Map<BytesN<32>, Address>,
    pub can_swap: bool,
}

impl Bridge {
    pub fn init_from(
        env: &Env,
        admin: Address,
        messenger: Address,
        gas_oracle: Address,
        native_token: Address,
    ) {
        Bridge {
            rebalancer: admin.clone(),
            messenger,

            from_gas_oracle_factor: Map::new(env),
            bridging_fee_conversion_factor: Map::new(env),
            pools: Map::new(env),
            can_swap: true,
        }
        .save(env);
        Admin(admin).save(env);
        GasOracleAddress(gas_oracle).save(env);
        NativeToken(native_token).save(env);
    }

    pub fn assert_can_swap(&self) -> Result<(), Error> {
        require!(self.can_swap, Error::SwapProhibited);

        Ok(())
    }

    pub fn get_pool_client_by_token(
        &self,
        env: &Env,
        token: BytesN<32>,
    ) -> Result<pool::Client<'_>, Error> {
        let pool_address = self.pools.get(token.clone()).ok_or(Error::NoReceivePool)?;
        Ok(pool::Client::new(env, &pool_address))
    }

    pub fn get_messenger_client(&self, env: &Env) -> messenger::Client {
        messenger::Client::new(env, &self.messenger)
    }
}
