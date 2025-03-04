use soroban_sdk::{contracttype, BytesN};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    SentMessage(BytesN<32>),
    ReceivedMessage(BytesN<32>),
}
