#![cfg(any(test, feature = "testutils"))]

use soroban_sdk::{
    testutils::{Address as _, BytesN as _, Ledger},
    token, Address, BytesN, Env,
};

use crate::{contract::LiquidityPoolClient, LiquidityPool};

mod tc_contract {
    soroban_sdk::contractimport!(
        file = "../argentina_pledge/target/wasm32-unknown-unknown/release/argentina_pledge.wasm"
    );
}

pub fn setup_pool<'a>(
    e: &Env,
    admin: &Address,
    ext_token_address: &Address,
    ext_token_decimals: &u32,
) -> LiquidityPoolClient<'a> {
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(e, &contract_id);
    let wasm_hash = install_token_wasm(e);

    client.initialize(admin, &wasm_hash, ext_token_address, ext_token_decimals, &0);
    client
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
    ext_token_address: &Address,
    ext_token_decimals: &u32,
) -> tc_contract::Client<'a> {
    let wasm_hash = e.deployer().upload_contract_wasm(tc_contract::WASM);
    let addr = e
        .deployer()
        .with_address(admin.clone(), BytesN::<32>::random(&e))
        .deploy(wasm_hash);
    let client = tc_contract::Client::new(e, &addr);
    client.initialize(&admin.clone(), ext_token_address, ext_token_decimals);
    client
}

pub fn set_ledger_timestamp(e: &Env, timestamp: u64) {
    e.ledger().with_mut(|li| li.timestamp = timestamp);
}
