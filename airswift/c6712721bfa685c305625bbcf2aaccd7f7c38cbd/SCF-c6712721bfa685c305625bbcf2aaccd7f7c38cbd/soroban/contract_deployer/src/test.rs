#![cfg(test)]
use crate::contract::{Deployer, DeployerClient};
use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    token, vec, Address, BytesN, Env, IntoVal, Symbol,
};

mod tc_contract {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
}

fn install_token_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../scf_soroban/target/wasm32-unknown-unknown/release/scf_soroban.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let deployer_contract = e.register_contract(None, Deployer);
    let deployer_client = DeployerClient::new(&e, &deployer_contract);
    let token = e.register_stellar_asset_contract(admin.clone());

    let wasm_hash = install_token_wasm(&e);
    let salt = BytesN::<32>::random(&e);

    let buyer = Address::generate(&e);
    let new_tc_contract = deployer_client.deploy_contract(
        &admin.clone(),
        &wasm_hash,
        &salt,
        &vec![
            &e,
            Symbol::new(&e, "initialize"),
            Symbol::new(&e, "set_external_token_provider"),
        ],
        &vec![
            &e,
            vec![
                &e,
                admin.into_val(&e),
                buyer.into_val(&e),
                1000000u32.into_val(&e),
                1714693253u64.into_val(&e),
            ],
            vec![&e, token.into_val(&e), 7u32.into_val(&e)],
        ],
    );

    let tc_client = tc_contract::Client::new(&e, &new_tc_contract);
    assert_eq!(tc_client.admin(), admin);
}
