use soroban_sdk::{contracttype, BytesN};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    OtherBridge(u32),
    SentMessage(BytesN<32>),
    ReceivedMessage(BytesN<32>),
}
