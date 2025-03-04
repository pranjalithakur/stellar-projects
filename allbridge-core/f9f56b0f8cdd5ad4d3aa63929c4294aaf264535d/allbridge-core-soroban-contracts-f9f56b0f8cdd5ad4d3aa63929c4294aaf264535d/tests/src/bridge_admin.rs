use shared::consts::{CHAIN_PRECISION, ORACLE_PRECISION};
use shared::Error;
use soroban_sdk::testutils::{Address as _, BytesN as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, BytesN, Env, IntoVal};

use crate::utils::consts::GOERLI_CHAIN_ID;
use crate::utils::{
    contract_id, desoroban_result, expect_auth_error, expect_contract_error, BridgeEnv, Pool,
};

#[test]
fn add_pool() {
    let env = Env::default();
    let BridgeEnv {
        bridge,
        native_token,
        admin,
        ..
    } = BridgeEnv::default(&env);

    let init_bridge_config = bridge.client.get_config();

    let pool = Pool::create(&env, &admin, &bridge.id, 20, &native_token.id, 30, 0, 1);

    bridge.client.add_pool(&pool.id, &native_token.id);

    let bridge_config = bridge.client.get_config();

    assert_eq!(
        bridge_config.pools.len(),
        init_bridge_config.pools.len() + 1
    );

    let decimals = native_token.client.decimals();

    let bridging_fee_conversion_factor = 10u128.pow(ORACLE_PRECISION - decimals + CHAIN_PRECISION);
    let from_gas_oracle_factor = 10u128.pow(ORACLE_PRECISION - decimals);

    let pool_id_on_contract = bridge_config
        .pools
        .get(contract_id(&native_token.id))
        .unwrap();

    let from_gas_oracle_factor_on_contract = bridge_config
        .from_gas_oracle_factor
        .get(native_token.id.clone())
        .unwrap();

    let bridging_fee_conversion_factor_on_contract = bridge_config
        .bridging_fee_conversion_factor
        .get(native_token.id.clone())
        .unwrap();

    assert_eq!(pool_id_on_contract, pool.id);
    assert_eq!(from_gas_oracle_factor_on_contract, from_gas_oracle_factor);
    assert_eq!(
        bridging_fee_conversion_factor_on_contract,
        bridging_fee_conversion_factor
    );
}

#[test]
fn set_gas_oracle_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let new_gas_oracle = Address::generate(&env);
    env.mock_auths(&[]);

    expect_auth_error(
        &env,
        desoroban_result(bridge.client.try_set_gas_oracle(&new_gas_oracle)),
    );
}

#[test]
fn set_gas_oracle() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let new_gas_oracle = Address::generate(&env);
    bridge.client.set_gas_oracle(&new_gas_oracle);

    let get_gas_oracle = bridge.client.get_gas_oracle();
    assert_eq!(get_gas_oracle, new_gas_oracle);
}

#[test]
fn set_rebalancer_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let rebalancer = Address::generate(&env);
    env.mock_auths(&[]);

    expect_auth_error(
        &env,
        desoroban_result(bridge.client.try_set_rebalancer(&rebalancer)),
    );
}

#[test]
fn set_rebalancer() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let rebalancer = Address::generate(&env);
    bridge.client.set_rebalancer(&rebalancer);

    assert_eq!(bridge.client.get_config().rebalancer, rebalancer);
}

#[test]
fn set_messenger() {
    let env = Env::default();
    let BridgeEnv {
        bridge, ref admin, ..
    } = BridgeEnv::default(&env);
    env.mock_auths(&[]);

    let messenger = Address::generate(&env);

    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &bridge.id,
            fn_name: "set_messenger",
            args: (&messenger,).into_val(&env),
            sub_invokes: &[],
        },
    }]);

    bridge.client.set_messenger(&messenger);

    assert_eq!(bridge.client.get_config().messenger, messenger);
}

#[test]
fn set_messenger_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);

    expect_auth_error(
        &env,
        desoroban_result(bridge.client.try_set_messenger(&Address::generate(&env))),
    );
}

#[test]
fn set_stop_authority() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    assert_eq!(bridge.client.get_stop_authority(), stop_authority);
}

#[test]
fn set_stop_authorityno_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let stop_authority = Address::generate(&env);

    env.mock_auths(&[]);

    expect_auth_error(
        &env,
        desoroban_result(bridge.client.try_set_stop_authority(&stop_authority)),
    );
}

#[test]
fn sucessful_stop_swap() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);
    let BridgeEnv { ref bridge, .. } = bridge_env;

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    let swap_and_bridge_call_result = bridge_env.do_swap_and_bridge(
        &env,
        &bridge_env.alice,
        &bridge_env.yaro_token,
        10.0,
        30_00.0,
        0.0,
    );
    expect_contract_error(&env, swap_and_bridge_call_result, Error::SwapProhibited);

    let receive_tokens_call_result = bridge_env.do_receive_tokens(
        &env,
        10.0,
        0,
        1.5,
        &bridge_env.alice,
        &bridge_env.yaro_token,
    );
    expect_contract_error(&env, receive_tokens_call_result, Error::SwapProhibited);

    let swap_call_result = bridge_env.do_swap(
        &env,
        &bridge_env.alice,
        &bridge_env.alice,
        &bridge_env.yaro_token,
        &bridge_env.yusd_token,
        10.0,
        1.0,
    );
    expect_contract_error(&env, swap_call_result, Error::SwapProhibited);
}

#[test]
fn stop_swap_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    env.mock_auths(&[]);
    expect_auth_error(&env, desoroban_result(bridge.client.try_stop_swap()));
}

#[test]
fn sucessful_swap_restart() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);
    let BridgeEnv { ref bridge, .. } = bridge_env;

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    bridge.client.start_swap();

    let bridge_config = bridge.client.get_config();
    assert!(bridge_config.can_swap);

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            10.0,
            30_00.0,
            0.0,
        )
        .unwrap();

    bridge_env
        .do_receive_tokens(
            &env,
            10.0,
            0,
            1.5,
            &bridge_env.alice,
            &bridge_env.yaro_token,
        )
        .unwrap();

    bridge_env
        .do_swap(
            &env,
            &bridge_env.alice,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            &bridge_env.yusd_token,
            10.0,
            1.0,
        )
        .unwrap();
}

#[test]
fn swap_restart_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let stop_authority = Address::generate(&env);
    bridge.client.set_stop_authority(&stop_authority);

    bridge.client.stop_swap();

    let bridge_config = bridge.client.get_config();
    assert!(!bridge_config.can_swap);

    env.mock_auths(&[]);
    expect_auth_error(&env, desoroban_result(bridge.client.try_start_swap()));
}

#[test]
fn register_bridge() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let other_bridge = BytesN::random(&env);
    let chain_id = 5;

    bridge.client.register_bridge(&chain_id, &other_bridge);

    let another_bridge = bridge.client.get_another_bridge(&chain_id);
    println!("{:?}", another_bridge);
    assert_eq!(another_bridge.address, other_bridge);
    assert_eq!(another_bridge.tokens.len(), 0);
}

#[test]
fn register_bridge_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let error = desoroban_result(bridge.client.try_register_bridge(&5, &BytesN::random(&env)));

    expect_auth_error(&env, error);
}

#[test]
fn change_bridge_address() {
    let env = Env::default();
    let BridgeEnv {
        bridge,
        goerli_bridge,
        ..
    } = BridgeEnv::default(&env);

    assert_eq!(
        bridge.client.get_another_bridge(&GOERLI_CHAIN_ID).address,
        goerli_bridge
    );

    let bridge_address = BytesN::random(&env);

    bridge
        .client
        .register_bridge(&GOERLI_CHAIN_ID, &bridge_address);

    assert_eq!(
        bridge.client.get_another_bridge(&GOERLI_CHAIN_ID).address,
        bridge_address
    );
}

#[test]
fn add_bridge_token() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let token = BytesN::random(&env);

    bridge.client.add_bridge_token(&GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(another_bridge.tokens.get(token).unwrap());
}

#[test]
fn add_bridge_token_no_auth() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let error = desoroban_result(
        bridge
            .client
            .try_add_bridge_token(&GOERLI_CHAIN_ID, &BytesN::random(&env)),
    );

    expect_auth_error(&env, error);
}

#[test]
fn add_bridge_for_unregistered_bridge() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let call_result = desoroban_result(
        bridge
            .client
            .try_add_bridge_token(&10, &BytesN::random(&env)),
    );

    expect_contract_error(&env, call_result, Error::UnknownAnotherChain);
}

#[test]
fn remove_bridge_token() {
    let env = Env::default();
    let BridgeEnv { bridge, .. } = BridgeEnv::default(&env);

    let token = BytesN::random(&env);

    bridge.client.add_bridge_token(&GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(another_bridge.tokens.get(token.clone()).unwrap());

    bridge.client.remove_bridge_token(&GOERLI_CHAIN_ID, &token);

    let another_bridge = bridge.client.get_another_bridge(&GOERLI_CHAIN_ID);

    assert_eq!(another_bridge.tokens.len(), 2);
    assert!(!another_bridge.tokens.get(token).unwrap());
}

#[test]
fn withdraw_gas_tokens() {
    let env = Env::default();
    let BridgeEnv {
        admin,
        native_token,
        bridge,
        ..
    } = BridgeEnv::default(&env);

    let user = Address::generate(&env);
    let gas_amount = 10000000u128;
    let half_gas_amount = gas_amount / 2;

    native_token.asset_client.mint(&user, &(gas_amount as i128));
    native_token
        .client
        .transfer(&user, &bridge.id, &(gas_amount as i128));

    let init_admin_token_balance = native_token.balance_of(&admin);
    let init_bridge_token_balance = native_token.balance_of(&bridge.id);

    bridge.client.withdraw_gas_tokens(&admin, &half_gas_amount);

    let admin_token_balance = native_token.balance_of(&admin);
    let bridge_token_balance = native_token.balance_of(&bridge.id);

    assert_eq!(
        admin_token_balance,
        init_admin_token_balance + half_gas_amount
    );
    assert_eq!(
        bridge_token_balance,
        init_bridge_token_balance - half_gas_amount
    );
}
