use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};
use crate::utils::contract_id;

pub struct User {
    pub tag: &'static str,
    pub address: Address,
}

impl User {
    pub fn generate(env: &Env, tag: &'static str) -> User {
        User {
            tag,
            address: Address::generate(&env),
        }
    }

    pub fn as_address(&self) -> Address {
        self.address.clone()
    }

    pub fn contract_id(&self) -> BytesN<32> {
        contract_id(&self.address)
    }
}
