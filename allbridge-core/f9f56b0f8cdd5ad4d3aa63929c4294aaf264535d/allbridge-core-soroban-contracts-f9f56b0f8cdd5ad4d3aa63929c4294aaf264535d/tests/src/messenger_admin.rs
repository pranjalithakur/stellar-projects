use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env,
};

use crate::utils::consts::GOERLI_CHAIN_ID;
use crate::utils::{desoroban_result, expect_auth_error, BridgeEnv};

#[test]
fn set_other_chain_ids() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let other_chains_id = BytesN::random(&env);
    messenger.client.set_other_chain_ids(&other_chains_id);

    assert_eq!(other_chains_id, messenger.other_chain_ids());
}

#[test]
fn set_other_chain_ids_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);

    let call_result = desoroban_result(
        messenger
            .client
            .try_set_other_chain_ids(&BytesN::random(&env)),
    );
    expect_auth_error(&env, call_result)
}

#[test]
fn set_gas_oracle() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let gas_oracle = Address::generate(&env);
    messenger.client.set_gas_oracle(&gas_oracle);

    assert_eq!(gas_oracle, messenger.gas_oracle());
}

#[test]
fn set_gas_oracle_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let call_result = desoroban_result(messenger.client.try_set_gas_oracle(&Address::generate(&env)));

    expect_auth_error(&env, call_result);
}

#[test]
fn set_gas_admin() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let admin = Address::generate(&env);
    messenger.client.set_admin(&admin);

    assert_eq!(admin, messenger.admin());
}

#[test]
fn set_gas_admin_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let call_result = desoroban_result(messenger.client.try_set_admin(&Address::generate(&env)));

    expect_auth_error(&env, call_result);
}

#[test]
fn set_primary_validator() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let validator_address = BytesN::random(&env);
    messenger.client.set_primary_validator(&validator_address);

    assert_eq!(validator_address, messenger.primary_validator_key());
}

#[test]
fn set_primary_validator_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_set_primary_validator(&BytesN::random(&env)),
    );

    expect_auth_error(&env, call_result);
}

#[test]
fn add_secondary_validator() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);

    assert!(messenger.has_secondary_validator_key(&validator_address));
}

#[test]
fn add_secondary_validator_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_add_secondary_validator(&BytesN::random(&env)),
    );

    expect_auth_error(&env, call_result);
}

#[test]
fn remove_secondary_validator() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);
    assert!(messenger.has_secondary_validator_key(&validator_address));

    messenger
        .client
        .remove_secondary_validator(&validator_address);
    assert!(!messenger.has_secondary_validator_key(&validator_address));
}

#[test]
fn remove_secondary_validator_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    let validator_address = BytesN::random(&env);
    messenger.client.add_secondary_validator(&validator_address);
    assert!(messenger.has_secondary_validator_key(&validator_address));

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_remove_secondary_validator(&validator_address),
    );

    expect_auth_error(&env, call_result);
}

#[test]
fn set_gas_usage() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    messenger.client.set_gas_usage(&GOERLI_CHAIN_ID, &100_000);

    assert_eq!(100_000, messenger.client.get_gas_usage(&GOERLI_CHAIN_ID));
}

#[test]
fn set_gas_usage_no_auth() {
    let env = Env::default();
    let BridgeEnv { ref messenger, .. } = BridgeEnv::default(&env);

    env.mock_auths(&[]);
    let call_result = desoroban_result(
        messenger
            .client
            .try_set_gas_usage(&GOERLI_CHAIN_ID, &100_000),
    );
    expect_auth_error(&env, call_result);
}
