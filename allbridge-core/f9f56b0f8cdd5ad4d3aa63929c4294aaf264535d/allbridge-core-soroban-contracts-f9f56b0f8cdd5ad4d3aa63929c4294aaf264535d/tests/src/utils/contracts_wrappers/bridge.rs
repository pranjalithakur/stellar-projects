use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, U256,
};

use crate::{
    contracts::bridge,
    utils::{desoroban_result, float_to_int_sp, CallResult, contract_id},
};

use super::{Token, User};

pub struct Bridge {
    pub id: soroban_sdk::Address,
    pub client: bridge::Client<'static>,
    pub native_token: Token,
}

impl Bridge {
    pub fn create(
        env: &Env,
        admin: &Address,
        messenger: &Address,
        gas_oracle: &Address,
        native_token: Token,
    ) -> Bridge {
        let id = env.register_contract_wasm(None, bridge::WASM);
        let client = bridge::Client::new(&env, &id);

        client.initialize(&admin, &messenger, &gas_oracle, &native_token.id);

        Bridge {
            id,
            client,
            native_token,
        }
    }

    /// (bridge, token)
    pub fn generate_and_register_bridge(
        &self,
        env: &Env,
        chain_id: u32,
    ) -> (BytesN<32>, BytesN<32>) {
        let other_bridge = BytesN::random(env);
        let other_token = BytesN::random(env);

        self.client.register_bridge(&chain_id, &other_bridge);
        self.client.add_bridge_token(&chain_id, &other_token);

        (other_bridge, other_token)
    }

    pub fn generate_and_set_rebalancer(&self, env: &Env) -> Address {
        let rebalancer = Address::generate(&env);
        self.client.set_rebalancer(&rebalancer);

        rebalancer
    }

    pub fn generate_and_set_stop_authority(&self, env: &Env) -> Address {
        let stop_authority = Address::generate(&env);
        self.client.set_stop_authority(&stop_authority);

        stop_authority
    }

    pub fn receive_tokens(
        &self,
        sender: &Address,
        amount: f64,
        recipient: &User,
        source_chain_id: u32,
        receive_token: &Token,
        nonce: &U256,
        receive_amount_min: f64,
        claimable: bool,
        extra_gas: &Option<u128>,
    ) -> CallResult {
        let amount_sp = float_to_int_sp(amount);
        let receive_amount_min = receive_token.float_to_int(receive_amount_min);

        desoroban_result(self.client.try_receive_tokens(
            &sender,
            &amount_sp,
            &recipient.as_address(),
            &source_chain_id,
            &contract_id(&receive_token.id),
            &nonce,
            &receive_amount_min,
            &claimable,
            &extra_gas,
        ))
    }

    pub fn swap_and_bridge(
        &self,
        sender: &User,
        token: &Token,
        amount: f64,
        gas_amount: f64,
        fee_token_amount: f64,
        destination_chain_id: u32,
        recipient: &BytesN<32>,
        receive_token: &BytesN<32>,
        nonce: &U256,
    ) -> CallResult {
        let amount = token.float_to_int(amount);
        let gas_amount = self.native_token.float_to_int(gas_amount);
        let fee_token_amount = token.float_to_int(fee_token_amount);

        desoroban_result(self.client.try_swap_and_bridge(
            &sender.as_address(),
            &token.id,
            &amount,
            &recipient,
            &destination_chain_id,
            &receive_token,
            &nonce,
            &gas_amount,
            &fee_token_amount,
        ))
    }
}
