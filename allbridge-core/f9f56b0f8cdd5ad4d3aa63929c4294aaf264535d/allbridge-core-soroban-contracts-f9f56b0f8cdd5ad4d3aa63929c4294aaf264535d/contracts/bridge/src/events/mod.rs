use proc_macros::Event;
use soroban_sdk::{contracttype, Address, BytesN, U256};

#[derive(Event)]
#[contracttype]
pub struct Swapped {
    pub sender: Address,
    pub recipient: Address,
    pub send_token: BytesN<32>,
    pub receive_token: BytesN<32>,
    pub send_amount: u128,
    pub receive_amount: u128,
}

#[derive(Event)]
#[contracttype]
pub struct TokensSent {
    pub amount: u128,
    pub recipient: BytesN<32>,
    pub destination_chain_id: u32,
    pub receive_token: BytesN<32>,
    pub nonce: U256,
}

#[derive(Event)]
#[contracttype]
pub struct TokensReceived {
    pub amount: u128,
    pub recipient: BytesN<32>,
    pub nonce: U256,
    pub message: BytesN<32>,
    pub claimable: bool
}

#[derive(Event)]
#[contracttype]
pub struct ReceiveFee {
    pub bridge_transaction_cost: u128,
    pub message_transaction_cost: u128,
    pub extra_gas: u128,
}

#[derive(Event)]
#[contracttype]
pub struct BridgingFeeFromTokens {
    pub gas: u128,
    pub fee_token_amount: u128,
}
