use soroban_sdk::Env;

use crate::{
    contracts::pool::{Deposit, Withdraw},
    utils::{
        assert_rel_eq, float_to_int_sp, format_diff, get_event_by_name, BridgeEnv, BridgeEnvConfig,
    },
};

#[test]
fn deposit() {
    let env = Env::default();
    let BridgeEnv {
        yaro_pool,
        alice,
        yaro_token,
        ..
    } = BridgeEnv::default(&env);

    let deposit_amount = 100.0;

    let balance_before = yaro_token.balance_of(&alice.as_address());
    yaro_pool.deposit(&alice, deposit_amount).unwrap();
    let balance_after = yaro_token.balance_of(&alice.as_address());

    assert_eq!(
        yaro_pool.user_deposit(&alice).lp_amount,
        float_to_int_sp(deposit_amount)
    );
    assert_eq!(
        balance_before - balance_after,
        yaro_token.float_to_int(deposit_amount)
    );

    let deposit_event = get_event_by_name::<Deposit>(&env, "Deposit");

    assert!(deposit_event.is_some());
    assert_eq!(
        deposit_event.unwrap(),
        Deposit {
            user: alice.as_address(),
            amount: float_to_int_sp(deposit_amount)
        }
    );
}

#[test]
fn withdraw() {
    let env = Env::default();
    let BridgeEnv {
        yaro_pool,
        alice,
        yaro_token,
        ..
    } = BridgeEnv::default(&env);

    let withdraw_amount = 100.0;
    yaro_pool.deposit(&alice, withdraw_amount).unwrap();

    let balance_before = yaro_token.balance_of(&alice.as_address());
    yaro_pool.withdraw(&alice, withdraw_amount).unwrap();
    let balance_after = yaro_token.balance_of(&alice.as_address());

    assert_eq!(yaro_pool.user_deposit(&alice).lp_amount, 0);
    assert_eq!(
        balance_after - balance_before,
        yaro_token.float_to_int(withdraw_amount)
    );

    let withdraw_event = get_event_by_name::<Withdraw>(&env, "Withdraw");

    assert!(withdraw_event.is_some());
    assert_eq!(
        withdraw_event.unwrap(),
        Withdraw {
            user: alice.as_address(),
            amount: float_to_int_sp(withdraw_amount)
        }
    );
}

#[test]
fn zero_diff() {
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

    let total_lp_amount_before = yaro_pool.total_lp_amount();
    yaro_pool.client.adjust_total_lp_amount();
    let total_lp_amount_after = yaro_pool.total_lp_amount();

    println!(
        "Total lp amount change: {}",
        &format_diff(total_lp_amount_before, total_lp_amount_after)
    );

    assert_rel_eq(total_lp_amount_before, total_lp_amount_after, 5);
    assert_eq!(yaro_pool.d(), total_lp_amount_after);
}

#[test]
fn success() {
    let env = Env::default();
    let bridge_env = BridgeEnv::create(
        &env,
        BridgeEnvConfig {
            yaro_admin_deposit: 1_000_000_000.0,
            yusd_admin_deposit: 1_000_000_000.0,
            ..Default::default()
        },
    );
    let BridgeEnv {
        ref yaro_pool,
        ref bob,
        ref yusd_pool,
        ref admin,
        ..
    } = bridge_env;

    let init_owner_lp_amount = yaro_pool.user_deposit_by_id(&admin);

    let vusd_amount = yaro_pool.swap_to_v_usd(&bob, 50_000_000.0);
    yusd_pool
        .client
        .swap_from_v_usd(&bob.as_address(), &vusd_amount, &0, &false, &false);

    yaro_pool.deposit(&bob, 50_000_000.0).unwrap();
    yaro_pool
        .withdraw_raw(&bob, yaro_pool.user_deposit(&bob).lp_amount)
        .unwrap();

    let total_lp_amount_before = yaro_pool.total_lp_amount();
    assert!(total_lp_amount_before < yaro_pool.d());

    yaro_pool.client.adjust_total_lp_amount();

    assert_eq!(yaro_pool.total_lp_amount(), yaro_pool.d());
    assert_eq!(
        yaro_pool.user_deposit_by_id(&admin).lp_amount - init_owner_lp_amount.lp_amount,
        yaro_pool.d() - total_lp_amount_before
    );
}


#[test]
fn claim_balance() {
    let env = Env::default();
    let bridge_env = BridgeEnv::create(
        &env,
        BridgeEnvConfig {
            yaro_admin_deposit: 1_000_000_000.0,
            yusd_admin_deposit: 1_000_000_000.0,
            ..Default::default()
        },
    );

    let BridgeEnv {
        ref yaro_pool,
        ref bob,
        ref yaro_token,
        ..
    } = bridge_env;
    yaro_token.balance_of(&bob.as_address());

    let bob_balance_before_swap = yaro_token.balance_of(&bob.as_address());
    let claimable_balance_before_swap = yaro_pool.get_claimable_balance(&bob);
    assert_eq!(claimable_balance_before_swap, 0);

    let vusd_amount = 50.0;
    let amount = yaro_pool
        .swap_from_v_usd(&bob, vusd_amount, true);
    let claimable_balance_after_swap = yaro_pool.get_claimable_balance(&bob);

    assert_eq!(claimable_balance_after_swap, amount);

    let bob_balance_after_swap = yaro_token.balance_of(&bob.as_address());

    assert_eq!(bob_balance_before_swap, bob_balance_after_swap);

    yaro_pool.client.claim_balance(&bob.address);
    let claimable_balance_after_claim = yaro_pool.get_claimable_balance(&bob);
    assert_eq!(claimable_balance_after_claim, 0);

    let bob_balance_after_claim = yaro_token.balance_of(&bob.as_address());

    assert_eq!(bob_balance_before_swap + amount, bob_balance_after_claim);
}

