use crate::admin::{is_amm, is_kyc_passed};
use crate::amm::{calculate_amm_reward_share, get_amm_depositors};
use soroban_sdk::{Address, Env};

use crate::balance::read_balance;
use crate::contract::check_non_negative_amount;
use crate::storage_types::{
    AccumulatedReward, DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD,
    INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD,
};

pub fn read_reward(e: &Env, addr: Address) -> i128 {
    let key = DataKey::RewardCheckpoint(addr);
    if let Some(reward) = e
        .storage()
        .persistent()
        .get::<DataKey, AccumulatedReward>(&key)
    {
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        reward.amount
    } else {
        0
    }
}

fn write_reward(e: &Env, addr: Address, amount: i128) {
    check_non_negative_amount(amount);

    let key = DataKey::RewardCheckpoint(addr);
    let existing_reward: Option<AccumulatedReward> = e.storage().persistent().get(&key);
    match existing_reward {
        Some(reward) => {
            let acc_reward = AccumulatedReward {
                created_ledger_number: reward.created_ledger_number,
                last_ledger_number: e.ledger().sequence(),
                amount: amount + reward.amount,
            };
            e.storage().persistent().set(&key, &acc_reward);
            e.storage().persistent().extend_ttl(
                &key,
                BALANCE_LIFETIME_THRESHOLD,
                BALANCE_BUMP_AMOUNT,
            );
        }
        None => {
            let acc_reward = AccumulatedReward {
                created_ledger_number: e.ledger().sequence(),
                last_ledger_number: e.ledger().sequence(),
                amount,
            };
            e.storage().persistent().set(&key, &acc_reward)
        }
    }
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn reset_reward(e: &Env, addr: Address) {
    let key = DataKey::RewardCheckpoint(addr);
    e.storage().persistent().remove(&key);
}

pub fn set_reward_rate(e: &Env, rate: u32) {
    let key = DataKey::RewardRate;
    let rate = rate.max(0);
    e.storage().persistent().set(&key, &rate);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_reward_rate(e: &Env) -> u32 {
    let key = DataKey::RewardRate;
    if let Some(rate) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
        rate
    } else {
        0
    }
}

pub fn set_reward_tick(e: &Env, tick: u32) {
    let tick = tick.max(0);
    let key = DataKey::RewardTick;
    e.storage().persistent().set(&key, &tick);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_reward_tick(e: &Env) -> u32 {
    let key = DataKey::RewardTick;
    if let Some(tick) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
        tick
    } else {
        // every block
        1
    }
}

pub fn calculate_reward(e: &Env, addr: Address) -> i128 {
    let key = DataKey::RewardCheckpoint(addr.clone());
    let reward_checkpoint: Option<AccumulatedReward> = e.storage().persistent().get(&key);
    let blocks_held = match reward_checkpoint {
        Some(checkpoint) => e.ledger().sequence() - checkpoint.last_ledger_number,
        None => 0,
    };
    let balance = read_balance(e, addr.clone());
    let reward_rate = get_reward_rate(e);
    let reward_tick = get_reward_tick(e);

    _calculate_reward(blocks_held, balance, reward_rate, reward_tick)
}

pub fn _calculate_reward(
    blocks_held: u32,
    balance: i128,
    reward_rate: u32,
    reward_tick: u32,
) -> i128 {
    let basis_points = 10_000_i128;
    let scale_factor = 10_000_i128;

    let reward_rate_fp = (reward_rate as i128 * scale_factor) / basis_points;
    let holding_period_fp = (blocks_held as i128 * scale_factor) / reward_tick as i128;
    let reward_numerator = balance * reward_rate_fp * holding_period_fp;
    // Apply a rounding adjustment before the final division to ensure results close to .5 round up
    // We scale the numerator up further by the scale factor to ensure division is the last operation
    // This maximizes precision before rounding takes effect
    let rounded_numerator = reward_numerator + (scale_factor * scale_factor / 2);

    rounded_numerator / (scale_factor * scale_factor)
}

pub fn checkpoint_reward(e: &Env, address: Address) {
    if !is_kyc_passed(e, address.clone()) && !is_amm(e, address.clone()) {
        return;
    }

    let total_reward = calculate_reward(e, address.clone());
    write_reward(e, address.clone(), total_reward);

    if is_amm(e, address.clone()) {
        if let Some(depositors) = get_amm_depositors(e, address) {
            let total_balance: i128 = depositors.iter().map(|d| d.balance).sum();
            if total_balance > 0 {
                for depositor in depositors.iter() {
                    let reward =
                        calculate_amm_reward_share(total_reward, depositor.balance, total_balance);
                    write_reward(e, depositor.depositor, reward);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    extern crate std;

    use soroban_sdk::testutils::Address as _;
    use soroban_sdk::{Address, Env};

    use crate::reward::{_calculate_reward, set_reward_rate, set_reward_tick};

    fn setup_test_env() -> (Env, Address) {
        let env = Env::default();
        let investor = soroban_sdk::Address::generate(&env);
        let token_admin = soroban_sdk::Address::generate(&env);
        env.mock_all_auths();
        let _token = crate::test::create_token(&env, &token_admin);
        set_reward_tick(&env, 1);
        set_reward_rate(&env, 5_00);
        (env, investor)
    }

    #[test]
    fn test_reward_calculation_per_block_tick() {
        let reward_rate = 5_00;
        let reward_tick = 1;

        let blocks_held = 10;
        let balance = 1000;

        let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);

        assert_eq!(result, 500, "Rounding error in _calculate_reward function");

        let blocks_held = 1_000_000;
        let balance = 1_000_000_000;

        let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
        assert_eq!(
            result, 50_000_000_000_000,
            "Rounding error in _calculate_reward function"
        );
    }

    #[test]
    fn test_reward_calculation_per_day_tick() {
        let reward_rate = 5_00;
        let reward_tick = 28_800;

        let blocks_held = 287;
        let balance = 1_000;

        let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
        assert_eq!(result, 0);

        let blocks_held = 288;
        let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
        assert_eq!(result, 1);

        let balance = 10_000;
        let result = _calculate_reward(blocks_held, balance, reward_rate, reward_tick);
        assert_eq!(result, 5);
    }
}
