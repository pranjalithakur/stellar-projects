#![cfg(any(test, feature = "testutils"))]

use crate::contract::{TokenizedCertificate, TokenizedCertificateClient};
use soroban_sdk::{testutils::Ledger, token, Address, Env};

pub fn setup_test_tc_contract<'a>(
    e: &Env,
    admin: &Address,
    ext_token_address: &Address,
    ext_token_decimals: &u32,
) -> TokenizedCertificateClient<'a> {
    let contract_id = e.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(e, &contract_id);

    client.initialize(admin, ext_token_address, ext_token_decimals);
    client
}

pub fn setup_test_token<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let addr = e.register_stellar_asset_contract(admin.clone());
    (
        token::Client::new(e, &addr),
        token::StellarAssetClient::new(e, &addr),
    )
}

pub fn set_ledger_timestamp(e: &Env, timestamp: u64) {
    e.ledger().with_mut(|li| li.timestamp = timestamp);
}
