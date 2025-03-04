use shared::{consts::CHAIN_ID, Error};

use soroban_sdk::{
    testutils::{Address as _, BytesN as _},
    xdr::ScError,
    Address, BytesN, Env,
};

use crate::utils::{consts::GOERLI_CHAIN_ID, expect_sc_error, contract_id};
use crate::utils::{expect_contract_error, float_to_int_sp, gen_nonce, BridgeEnv, BridgeEnvConfig};

#[test]
fn swap_and_bridge() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            1000.0,
            30_00.0,
            0.0,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_fee_share_gt_zero() {
    let env = Env::default();
    let bridge_env = BridgeEnv::create(
        &env,
        BridgeEnvConfig {
            yaro_fee_share_bp: 0.05,
            ..Default::default()
        },
    );

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            1000.0,
            30_00.0,
            0.0,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_near_zero() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            0.0001,
            30_00.0,
            0.0,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_near_zero_unbalanced_pool() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_receive_tokens(
            &env,
            200_000.0,
            0,
            200_000.0,
            &bridge_env.alice,
            &bridge_env.yaro_token,
        )
        .unwrap();

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            0.001,
            30_00.0,
            0.0,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_fee_fully_in_token() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            1000.0,
            0.0,
            5.0,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_fee_partially_in_token() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_swap_and_bridge(
            &env,
            &bridge_env.alice,
            &bridge_env.yaro_token,
            100.0,
            5.0,
            0.1,
        )
        .unwrap();
}

#[test]
fn swap_and_bridge_bridge_to_the_zero_address() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::<32>::from_array(&env, &[0; 32]),
        &bridge_env.goerli_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::BridgeToTheZeroAddress)
}

#[test]
fn swap_and_bridge_bridge_swap_prohibited() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env.bridge.client.stop_swap();

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::random(&env),
        &bridge_env.goerli_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::SwapProhibited);
}

#[test]
fn swap_and_bridge_invalid_other_chain_id() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        CHAIN_ID,
        &BytesN::random(&env),
        &bridge_env.goerli_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::InvalidOtherChainId);
}

#[test]
fn swap_and_bridge_unknown_chain() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        10,
        &BytesN::random(&env),
        &bridge_env.goerli_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::UnknownAnotherChain);
}

#[test]
fn swap_and_bridge_unknown_token() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        5_000.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::random(&env),
        &BytesN::random(&env),
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::UnknownAnotherToken);
}

#[test]
fn swap_and_bridge_amount_too_low_for_fee() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let call_result = bridge_env.bridge.swap_and_bridge(
        &bridge_env.alice,
        &bridge_env.yaro_token,
        1_000.0,
        0.0,
        0.0,
        GOERLI_CHAIN_ID,
        &BytesN::random(&env),
        &bridge_env.goerli_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, call_result, Error::AmountTooLowForFee);
}

#[test]
pub fn receive_tokens() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_receive_tokens(
            &env,
            100.0,
            0,
            1.5,
            &bridge_env.alice,
            &bridge_env.yaro_token,
        )
        .unwrap();
}

#[test]
pub fn receive_tokens_fee_share_gt_zero() {
    let env = Env::default();
    let bridge_env = BridgeEnv::create(
        &env,
        BridgeEnvConfig {
            yaro_fee_share_bp: 0.05,
            ..Default::default()
        },
    );

    bridge_env
        .do_receive_tokens(
            &env,
            100.0,
            0,
            100.0,
            &bridge_env.alice,
            &bridge_env.yaro_token,
        )
        .unwrap();
}

#[test]
pub fn receive_tokens_zero_amount() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_receive_tokens(&env, 0.0, 0, 0.0, &bridge_env.alice, &bridge_env.yaro_token)
        .unwrap();
}

#[test]
pub fn receive_tokens_extra_gas() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env
        .do_receive_tokens(
            &env,
            100.0,
            1000,
            1.5,
            &bridge_env.alice,
            &bridge_env.yusd_token,
        )
        .unwrap();
}

#[test]
pub fn receive_tokens_extra_gas_not_enough_native_token_on_bridge() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let nonce = gen_nonce(&env);

    bridge_env.native_token.client.transfer(
        &bridge_env.bridge.id,
        &Address::generate(&env),
        &(bridge_env.native_token.balance_of(&bridge_env.bridge.id) as i128),
    );

    bridge_env
        .hash_and_receive_message(
            &env,
            float_to_int_sp(1_000.0),
            &bridge_env.alice.as_address(),
            &bridge_env.yaro_token,
            &nonce,
        )
        .unwrap();

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        1_000.0,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        0.0,
        false,
        &Some(1000),
    );

    expect_sc_error(&env, call_result, ScError::Contract(10));
}

#[test]
pub fn receive_tokens_no_message() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let amount = 1_000.0;

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &gen_nonce(&env),
        amount - 10.0,
        false,
        &Some(0u128),
    );

    expect_contract_error(&env, call_result, Error::NoMessage);
}

#[test]
pub fn receive_tokens_swap_prohibited() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    bridge_env.bridge.client.stop_swap();

    let amount = 1_000.0;

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &gen_nonce(&env),
        amount - 10.0,
        false,
        &Some(0u128),
    );

    expect_contract_error(&env, call_result, Error::SwapProhibited);
}

#[test]
pub fn receive_tokens_source_not_registered() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let amount = 1_000.0;

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        10,
        &bridge_env.yaro_token,
        &gen_nonce(&env),
        amount - 10.0,
        false,
        &Some(0u128),
    );

    expect_contract_error(&env, call_result, Error::SourceNotRegistered);
}

#[test]
pub fn receive_tokens_insufficient_received_amount() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let amount = 1_000.0;
    let nonce = gen_nonce(&env);

    bridge_env
        .hash_and_receive_message(
            &env,
            float_to_int_sp(amount),
            &bridge_env.alice.as_address(),
            &bridge_env.yaro_token,
            &nonce,
        )
        .unwrap();

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        amount + 10.0,
        false,
        &Some(0u128),
    );

    expect_contract_error(&env, call_result, Error::InsufficientReceivedAmount);
}

#[test]
pub fn receive_tokens_message_processed() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let nonce = gen_nonce(&env);
    let amount = 1_000.0;
    let amount_sp = float_to_int_sp(amount);

    bridge_env
        .hash_and_receive_message(
            &env,
            float_to_int_sp(amount),
            &bridge_env.alice.as_address(),
            &bridge_env.yaro_token,
            &nonce,
        )
        .unwrap();

    bridge_env.bridge.client.receive_tokens(
        &bridge_env.bridge.id,
        &amount_sp,
        &bridge_env.alice.as_address(),
        &GOERLI_CHAIN_ID,
        &contract_id(&bridge_env.yaro_token.id),
        &nonce,
        &0,
        &false,
        &Some(0u128),
    );

    let call_result = bridge_env.bridge.receive_tokens(
        &bridge_env.bridge.id,
        amount,
        &bridge_env.alice,
        GOERLI_CHAIN_ID,
        &bridge_env.yaro_token,
        &nonce,
        0.0,
        false,
        &Some(0u128),
    );

    expect_contract_error(&env, call_result, Error::MessageProcessed);
}

#[test]
pub fn swap() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

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
