#![cfg(any(test, feature = "testutils"))]

mod tc_contract {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

use crate::contract::{OfferPool, OfferPoolClient};
use soroban_sdk::{
    testutils::BytesN as _,
    token::{self, TokenClient},
    Address, BytesN, Env,
};

pub fn setup_pool<'a>(e: &Env, admin: &Address) -> (OfferPoolClient<'a>, Address) {
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(e, &contract_id);
    let wasm_hash = install_token_wasm(e);

    client.initialize(admin, &wasm_hash);

    (client, contract_id)
}

pub fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
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

pub fn setup_tc<'a>(
    e: &Env,
    admin: &Address,
    buyer: &Address,
    total_amount: &u32,
    end_time: &u64,
    ext_token_address: &Address,
    ext_token_decimals: &u32,
) -> tc_contract::Client<'a> {
    let wasm_hash = e.deployer().upload_contract_wasm(tc_contract::WASM);
    let addr = e
        .deployer()
        .with_address(admin.clone(), BytesN::<32>::random(&e))
        .deploy(wasm_hash);
    let client = tc_contract::Client::new(e, &addr);
    client.initialize(&admin.clone(), buyer, total_amount, end_time);
    client.set_external_token_provider(ext_token_address, ext_token_decimals);
    client
}
