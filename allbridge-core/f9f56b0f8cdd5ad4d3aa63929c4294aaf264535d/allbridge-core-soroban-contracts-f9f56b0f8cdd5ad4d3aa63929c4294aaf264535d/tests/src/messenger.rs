use shared::{
    consts::{CHAIN_ID, CHAIN_PRECISION, ORACLE_PRECISION},
    utils::{hash_message, hash_with_sender, hash_with_sender_address},
    Error,
};
use soroban_sdk::{
    map,
    testutils::{Address as _, BytesN as _},
    Address, BytesN, Env, U256,
};

use crate::utils::{
    consts::{GOERLI_CHAIN_ID, GOERLI_GAS_PRICE, GOERLI_PRICE, THIS_PRICE},
    expect_contract_error, gen_nonce, message_hash_vec_to_byte, sign_message, vec_to_bytes,
    BridgeEnv, Messenger, MessengerConfig, GAS_AMOUNT, contract_id
};

const FROM_ORACLE_TO_CHAIN_SCALING_FACTOR: u128 = 10u128.pow(ORACLE_PRECISION - CHAIN_PRECISION);

#[test]
fn messenger_init() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let native_token_address = env.register_stellar_asset_contract(admin.clone());
    let gas_oracle_id = Address::generate(&env);

    let init_config = MessengerConfig {
        admin: admin.clone(),
        native_token: native_token_address.clone(),
        gas_oracle: gas_oracle_id.clone(),
        secondary_validator_keys: map![&env, (BytesN::random(&env), true)],
        ..MessengerConfig::default_config(&env)
    };

    let messenger = Messenger::create(&env, init_config.clone());

    assert_eq!(messenger.client.get_config(), init_config.into());
}

#[test]
fn messenger_send_message() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let mut message = BytesN::random(&env);
    message.set(0, CHAIN_ID as u8);
    message.set(1, GOERLI_CHAIN_ID as u8);
    let hash_with_sender =
        hash_with_sender_address(&env, &message, &bridge_env.alice.as_address()).unwrap();

    bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message)
        .unwrap();

    let expected_fee = (GOERLI_GAS_PRICE * GAS_AMOUNT * GOERLI_PRICE
        / THIS_PRICE
        / FROM_ORACLE_TO_CHAIN_SCALING_FACTOR) as i128;

    assert!(bridge_env
        .messenger
        .client
        .has_sent_message(&hash_with_sender));
    assert_eq!(
        bridge_env
            .native_token
            .client
            .balance(&bridge_env.messenger.id),
        expected_fee
    );
}

#[test]
fn send_message_to_unsupported_chain() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let mut message = BytesN::random(&env);
    message.set(0, 8);
    message.set(1, GOERLI_CHAIN_ID as u8);

    let try_result = bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message);

    expect_contract_error(&env, try_result, Error::InvalidChainId);
}

#[test]
fn send_message_with_wrong_chain_id() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let mut message = BytesN::random(&env);
    message.set(0, CHAIN_ID as u8);
    message.set(1, 8);

    let try_result = bridge_env
        .messenger
        .send_message(&bridge_env.alice, &message);

    expect_contract_error(&env, try_result, Error::InvalidOtherChainId);
}

#[test]
fn messenger_receive_message() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);

    let primary_validator_slice = hex::decode("04ba5734d8f7091719471e7f7ed6b9df170dc70cc661ca05e688601ad984f068b0d67351e5f06073092499336ab0839ef8a521afd334e53807205fa2f08eec74f4").unwrap();
    let secondary_validator_slice = hex::decode("049d9031e97dd78ff8c15aa86939de9b1e791066a0224e331bc962a2099a7b1f0464b8bbafe1535f2301c72c2cb3535b172da30b02686ab0393d348614f157fbdb").unwrap();
    let primary_signature_slice = hex::decode("e8d012d6892859ec0fb6a44d4693dd64d84854f804bfe89aad293e5f05754f9b3dbcb2ff6580db858a99b33daa0064a8851cda2ad532c27bc6fb2f0e55aaa200").unwrap();
    let secondary_signature_slice =    hex::decode("fb8fbfa594f889da925f57ef766871776568482d87f7364246d78459641d63ab655902e80af59d2ad772a3b277d51aaec3df354f6048b7edc9967109e808d616").unwrap();
    let message_slice =
        hex::decode("000354657374206d657373616765000000000000000000000000000000000000").unwrap();

    let primary_validator = vec_to_bytes::<65>(&env, primary_validator_slice);
    let secondary_validator = vec_to_bytes::<65>(&env, secondary_validator_slice);
    let message = vec_to_bytes::<32>(&env, message_slice);
    let primary_signature = vec_to_bytes::<64>(&env, primary_signature_slice);
    let secondary_signature = vec_to_bytes::<64>(&env, secondary_signature_slice);

    let messenger = Messenger::create(
        &env,
        MessengerConfig {
            admin: admin.clone(),
            primary_validator_key: primary_validator,
            secondary_validator_keys: map![&env, (secondary_validator, true)],
            ..MessengerConfig::default_config(&env)
        },
    );

    messenger
        .client
        .receive_message(&message, &primary_signature, &1, &secondary_signature, &1);

    assert!(messenger.client.has_received_message(&message));
}

#[test]
pub fn messenger_receive_message_full() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);

    let user = Address::generate(&env);
    let yaro_token = Address::generate(&env);
    let goerli_bridge = Address::generate(&env);

    let message_hash = hash_message(
        &env,
        100_000_000,
        &contract_id(&user),
        GOERLI_CHAIN_ID,
        CHAIN_ID,
        &contract_id(&yaro_token),
        &U256::from_u32(&env, 8247),
    );
    let message_hash_with_sender =
        hash_with_sender(&env, &message_hash, &contract_id(&goerli_bridge));
    let message_hash = message_hash_with_sender.to_array().to_vec();

    let primary_signature = sign_message(&env, &message_hash, &bridge_env.primary_validator_wallet);
    let secondary_signature =
        sign_message(&env, &message_hash, &bridge_env.secondary_validator_wallet);

    bridge_env
        .messenger
        .receive_message(
            &env,
            &message_hash_vec_to_byte(&env, &message_hash),
            &primary_signature,
            &secondary_signature,
        )
        .unwrap();
}

#[test]
fn send_message_twice() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);
    let BridgeEnv {
        ref alice,
        ref yaro_token,
        ref messenger,
        ..
    } = bridge_env;

    let message = bridge_env
        .messenger
        .hash_and_send_message(
            &env,
            &alice,
            100_000,
            &alice.as_address(),
            &yaro_token,
            &gen_nonce(&env),
        )
        .unwrap();

    let try_result = messenger.send_message(&alice, &message);

    expect_contract_error(&env, try_result, Error::HasMessage);
}

#[test]
fn confirm_message_with_broken_validator() {
    let env = Env::default();
    let mut bridge_env = BridgeEnv::default(&env);

    bridge_env.override_primary_validator(
        "a07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77",
    );

    let try_result = bridge_env.hash_and_receive_message(
        &env,
        100_000,
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, try_result, Error::InvalidPrimarySignature);
}

#[test]
fn confirm_message_with_broken_secondary_validator() {
    let env = Env::default();
    let mut bridge_env = BridgeEnv::default(&env);

    bridge_env.override_secondary_validator(
        "a07d0e5f33e159bd7b471d0e79e4211205f4e89949247ec01ba7559b71acee77",
    );

    let try_result = bridge_env.hash_and_receive_message(
        &env,
        100_000,
        &bridge_env.alice.as_address(),
        &bridge_env.yaro_token,
        &gen_nonce(&env),
    );

    expect_contract_error(&env, try_result, Error::InvalidSecondarySignature);
}

#[test]
fn withdraw_gas_tokens() {
    let env = Env::default();
    let bridge_env = BridgeEnv::default(&env);
    let BridgeEnv {
        ref alice,
        ref yaro_token,
        ref messenger,
        ref admin,
        ref native_token,
        ..
    } = bridge_env;

    bridge_env
        .messenger
        .hash_and_send_message(
            &env,
            &alice,
            100_000,
            &alice.as_address(),
            &yaro_token,
            &gen_nonce(&env),
        )
        .unwrap();

    let messenger_balance = native_token.balance_of(&messenger.id);

    messenger
        .client
        .withdraw_gas_tokens(&admin, &messenger_balance);

    let messenger_balance = native_token.balance_of(&messenger.id);

    assert_eq!(messenger_balance, 0);
}
