use proc_macros::Event;
use soroban_sdk::{contracttype, Address, BytesN, Vec};

#[derive(Event)]
#[contracttype]
pub struct MessageSent {
    pub message: BytesN<32>,
}

#[derive(Event)]
#[contracttype]
pub struct MessageReceived {
    pub message: BytesN<32>,
}

#[derive(Event)]
#[contracttype]
pub struct SecondaryValidatorsSet {
    pub old_validators: Vec<Address>,
    pub new_validators: Vec<Address>,
}
