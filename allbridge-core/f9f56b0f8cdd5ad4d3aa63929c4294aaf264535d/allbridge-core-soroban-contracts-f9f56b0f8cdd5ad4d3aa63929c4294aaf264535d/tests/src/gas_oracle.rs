use shared::{
    consts::{CHAIN_ID, CHAIN_PRECISION, ORACLE_PRECISION},
    Error,
};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

use crate::{
    contracts::gas_oracle,
    utils::{desoroban_result, expect_contract_error},
};

const FROM_ORACLE_TO_CHAIN_SCALING_FACTOR: u128 = 10u128.pow(ORACLE_PRECISION - CHAIN_PRECISION);

#[allow(dead_code)]
struct GasOracleEnv<'a> {
    pub admin: Address,
    pub gas_oracle_id: Address,
    pub gas_oracle_client: gas_oracle::Client<'a>,
}

fn setup_env(env: &Env) -> GasOracleEnv {
    env.mock_all_auths();

    let gas_oracle_id = env.register_contract_wasm(None, gas_oracle::WASM);
    let gas_oracle_client = gas_oracle::Client::new(&env, &gas_oracle_id);

    let admin = Address::generate(&env);

    gas_oracle_client.initialize(&admin);

    GasOracleEnv {
        admin,
        gas_oracle_client,
        gas_oracle_id,
    }
}

#[test]
fn test_initialize() {
    let env = Env::default();

    setup_env(&env);
}

#[test]
fn test_initialize_already_initialized() {
    let env = Env::default();
    let gas_oracle_env = setup_env(&env);

    let try_result = desoroban_result(
        gas_oracle_env
            .gas_oracle_client
            .try_initialize(&gas_oracle_env.admin),
    );

    expect_contract_error(&env, try_result, Error::Initialized);
}

#[test]
fn test_initialize_no_gas_price_data() {
    let env = Env::default();
    let gas_oracle_env = setup_env(&env);

    expect_contract_error(
        &env,
        desoroban_result(gas_oracle_env.gas_oracle_client.try_get_gas_price(&1)),
        Error::NoGasDataForChain,
    );
}

#[test]
fn test_set_price() {
    let env = Env::default();
    let gas_oracle_env = setup_env(&env);

    let chain_id = 1;

    gas_oracle_env
        .gas_oracle_client
        .set_price(&chain_id, &Some(150), &None);
    let new_gas_price = gas_oracle_env.gas_oracle_client.get_gas_price(&chain_id);

    assert_eq!(new_gas_price.price, 150);
    assert_eq!(new_gas_price.gas_price, 0);

    gas_oracle_env
        .gas_oracle_client
        .set_price(&chain_id, &None, &Some(100));
    let new_gas_price = gas_oracle_env.gas_oracle_client.get_gas_price(&chain_id);

    assert_eq!(new_gas_price.price, 150);
    assert_eq!(new_gas_price.gas_price, 100);

    gas_oracle_env
        .gas_oracle_client
        .set_price(&chain_id, &Some(250), &Some(150));
    let new_gas_price = gas_oracle_env.gas_oracle_client.get_gas_price(&chain_id);

    assert_eq!(new_gas_price.price, 250);
    assert_eq!(new_gas_price.gas_price, 150);
}

#[test]
fn test_set_new_admin() {
    let env = Env::default();
    let gas_oracle_env = setup_env(&env);
    let new_admin = Address::generate(&env);

    gas_oracle_env.gas_oracle_client.set_admin(&new_admin);
    assert_eq!(new_admin, gas_oracle_env.gas_oracle_client.get_admin());
}

#[test]
fn test_get_gas_cost_in_native_token() {
    let env = Env::default();

    let gas_oracle_env = setup_env(&env);

    let other_gas_price = 200_000_000;
    let other_price = 1_000_000_000;
    let this_price = 20_000_000;
    let gas_amount = 300_000_000;

    let expected_cost = other_gas_price * gas_amount * other_price
        / this_price
        / FROM_ORACLE_TO_CHAIN_SCALING_FACTOR;

    gas_oracle_env
        .gas_oracle_client
        .set_price(&2, &Some(other_price), &Some(other_gas_price));
    gas_oracle_env
        .gas_oracle_client
        .set_price(&CHAIN_ID, &Some(this_price), &Some(40));

    let cost = gas_oracle_env
        .gas_oracle_client
        .get_gas_cost_in_native_token(&2, &gas_amount);

    assert_eq!(expected_cost, cost);
}
