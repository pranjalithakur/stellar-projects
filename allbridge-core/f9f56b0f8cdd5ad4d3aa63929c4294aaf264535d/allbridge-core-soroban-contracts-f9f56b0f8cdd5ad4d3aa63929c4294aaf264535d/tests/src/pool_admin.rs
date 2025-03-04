use soroban_sdk::{testutils::Address as _, Address, Env};

use crate::utils::{
    desoroban_result, expect_auth_error, expect_contract_error, BridgeEnv, BridgeEnvConfig,
};

#[test]
fn stop_deposit() {
    let env = Env::default();
    let BridgeEnv {
        yaro_pool, alice, ..
    } = BridgeEnv::default(&env);

    yaro_pool.client.stop_deposit();
    assert!(!yaro_pool.can_deposit());
    let call_result = yaro_pool.deposit(&alice, 1000.0);

    expect_contract_error(&env, call_result, shared::Error::Forbidden);
}

#[test]
fn stop_deposit_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(&env, desoroban_result(yaro_pool.client.try_stop_deposit()));
}

#[test]
fn start_deposit() {
    let env = Env::default();
    let BridgeEnv {
        yaro_pool, alice, ..
    } = BridgeEnv::default(&env);

    yaro_pool.client.stop_deposit();
    yaro_pool.client.start_deposit();
    assert!(yaro_pool.can_deposit());

    yaro_pool.deposit(&alice, 1000.0).unwrap();
}

#[test]
fn start_deposit_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.stop_deposit();

    env.mock_auths(&[]);
    expect_auth_error(&env, desoroban_result(yaro_pool.client.try_start_deposit()));
}

#[test]
fn stop_withdraw() {
    let env = Env::default();
    let BridgeEnv {
        yaro_pool, alice, ..
    } = BridgeEnv::default(&env);

    yaro_pool.deposit(&alice, 1000.0).unwrap();

    yaro_pool.client.stop_withdraw();
    assert!(!yaro_pool.can_withdraw());

    let call_result = yaro_pool.withdraw(&alice, 1000.0);

    expect_contract_error(&env, call_result, shared::Error::Forbidden);
}

#[test]
fn stop_withdraw_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(&env, desoroban_result(yaro_pool.client.try_stop_withdraw()));
}

#[test]
fn start_withdraw() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.stop_withdraw();
    yaro_pool.client.start_withdraw();
    assert!(yaro_pool.can_withdraw());
}

#[test]
fn start_withdraw_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.stop_withdraw();

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_start_withdraw()),
    );
}

#[test]
fn set_admin() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    let admin = Address::generate(&env);
    yaro_pool.client.set_admin(&admin);

    assert_eq!(yaro_pool.admin(), admin);
}

#[test]
fn set_admin_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_set_admin(&Address::generate(&env))),
    );
}

#[test]
fn set_stop_authority() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    let stop_authority = Address::generate(&env);
    yaro_pool.client.set_stop_authority(&stop_authority);

    assert_eq!(yaro_pool.stop_authority(), stop_authority);
}

#[test]
fn set_stop_authority_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(
            yaro_pool
                .client
                .try_set_stop_authority(&Address::generate(&env)),
        ),
    );
}

#[test]
fn set_bridge() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    let bridge = Address::generate(&env);
    yaro_pool.client.set_bridge(&bridge);

    assert_eq!(yaro_pool.bridge(), bridge);
}

#[test]
fn set_bridge_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_set_bridge(&Address::generate(&env))),
    );
}

#[test]
fn set_fee_share() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.set_fee_share(&1000);
    assert_eq!(yaro_pool.fee_share_bp(), 1000);
}

#[test]
fn set_fee_share_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_set_fee_share(&1_000)),
    );
}

#[test]
fn set_balance_ratio_min_bp() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.set_balance_ratio_min_bp(&100);
    assert_eq!(yaro_pool.balance_ratio_min_bp(), 100);
}

#[test]
fn balance_ratio_min_bp_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_set_balance_ratio_min_bp(&100)),
    );
}

#[test]
fn admin_fee_share_bp() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    yaro_pool.client.set_admin_fee_share(&100);
    assert_eq!(yaro_pool.admin_fee_share_bp(), 100);
}

#[test]
fn admin_fee_share_bp_no_auth() {
    let env = Env::default();
    let BridgeEnv { yaro_pool, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_set_admin_fee_share(&100)),
    );
}

#[test]
fn adjust_total_lp_amount_no_auth() {
    let env = Env::default();
    let bridge_env = BridgeEnv::create(
        &env,
        BridgeEnvConfig {
            yaro_admin_deposit: 1_000_000_000.0,
            yusd_admin_deposit: 1_000_000_000.0,
            ..Default::default()
        },
    );
    let BridgeEnv { ref yaro_pool, .. } = bridge_env;

    env.mock_auths(&[]);

    expect_auth_error(
        &env,
        desoroban_result(yaro_pool.client.try_adjust_total_lp_amount()),
    );
}
