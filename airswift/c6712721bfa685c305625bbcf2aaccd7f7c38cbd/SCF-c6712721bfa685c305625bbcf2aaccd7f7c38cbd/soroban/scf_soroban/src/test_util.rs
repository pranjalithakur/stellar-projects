#![cfg(any(test, feature = "testutils"))]

use crate::contract::{TokenizedCertificate, TokenizedCertificateClient};
use soroban_sdk::{Address, Env};

pub fn setup_test_token<'a>(
    env: &Env,
    admin: &Address,
    buyer: &Address,
) -> TokenizedCertificateClient<'a> {
    let contract_id = env.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(env, &contract_id);

    let total_amount: u32 = 1000000;
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(admin, buyer, &total_amount, &end_time);
    client
}
