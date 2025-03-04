use crate::reward::reset_reward;
use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::storage_types::{DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};

#[derive(Clone)]
#[contracttype]
pub struct AmmDepositor {
    pub depositor: Address,
    pub balance: i128,
}

pub fn get_amm_depositors(e: &Env, amm_address: Address) -> Option<Vec<AmmDepositor>> {
    let key = DataKey::AmmDepositor(amm_address);
    let result = e
        .storage()
        .persistent()
        .get::<DataKey, Vec<AmmDepositor>>(&key);
    if e.storage().persistent().has(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
    }
    result
}

fn set_amm_depositors(e: &Env, amm_address: Address, depositors: Vec<AmmDepositor>) {
    let key = DataKey::AmmDepositor(amm_address);
    e.storage().persistent().set(&key, &depositors);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

fn clear_amm_depositors(e: &Env, amm_address: &Address) {
    let key = DataKey::AmmDepositor(amm_address.clone());
    e.storage().persistent().remove(&key);
}

pub fn update_amm_depositor_balance(
    e: &Env,
    amm_address: Address,
    depositor_address: Address,
    amount: i128,
) {
    let mut depositors = match get_amm_depositors(e, amm_address.clone()) {
        None => {
            if amount < 0 {
                panic!("tried to withdraw from a pool without any deposits")
            };
            let mut depositors: Vec<AmmDepositor> = Vec::new(e);
            depositors.push_back(AmmDepositor {
                depositor: depositor_address.clone(),
                balance: amount,
            });

            set_amm_depositors(e, amm_address, depositors);
            return;
        }
        Some(depositor) => depositor,
    };

    if amount > 0 {
        let depositor = depositors.iter().find(|d| d.depositor == depositor_address);
        match depositor {
            Some(mut depositor) => depositor.balance += amount,
            None => depositors.push_back(AmmDepositor {
                depositor: depositor_address,
                balance: amount,
            }),
        }
        set_amm_depositors(e, amm_address, depositors);
        return;
    }

    let total_outflow = -amount;
    let total_balance: i128 = depositors.iter().map(|d| d.balance).sum();
    if total_outflow >= total_balance {
        // Clear the list of depositors and return
        clear_amm_depositors(e, &amm_address);
        reset_reward(e, amm_address.clone());
        return;
    }

    // Calculate the new balances and the rounding errors for each depositor
    let mut new_balances: Vec<i128> = Vec::new(e);
    let mut rounding_errors: Vec<i128> = Vec::new(e);
    for depositor in depositors.iter() {
        let new_balance = depositor.balance - depositor.balance * total_outflow / total_balance;
        let rounding_error = depositor.balance * total_outflow % total_balance;
        new_balances.push_back(new_balance);
        rounding_errors.push_back(rounding_error);
    }

    // Distribute the rounding errors to the depositors with the largest balances
    let mut remaining_outflow = total_outflow - new_balances.iter().sum::<i128>();
    while remaining_outflow > 0 {
        let max_error_index = rounding_errors
            .iter()
            .enumerate()
            .max_by_key(|&(_, error)| error)
            .unwrap()
            .0 as u32;
        let new_balance = new_balances
            .get(max_error_index)
            .expect("there was an issue accessing depositor new balance");
        new_balances.set(max_error_index, new_balance - 1);
        rounding_errors.set(max_error_index, 0);
        remaining_outflow -= 1;
    }

    let mut updated_depositors: Vec<AmmDepositor> = Vec::new(e);
    for (index, depositor) in depositors.iter().enumerate() {
        let mut updated_depositor = depositor.clone();
        updated_depositor.balance = new_balances
            .get(index as u32)
            .expect("issue accessing user balacn");
        updated_depositors.push_back(updated_depositor);
    }
    depositors = updated_depositors;

    // Update the list of depositors in storage
    set_amm_depositors(e, amm_address, depositors);
}
pub fn calculate_amm_reward_share(
    total_reward: i128,
    depositor_balance: i128,
    total_balance: i128,
) -> i128 {
    if total_balance == 0 {
        return 0;
    }
    let scale_factor = 1_000_000_i128;
    let scaled_depositor_balance = depositor_balance * scale_factor;
    let participation = scaled_depositor_balance / total_balance;
    (total_reward * participation) / scale_factor
}

#[cfg(test)]
mod test {
    extern crate std;

    use soroban_sdk::{
        testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
        Address, Env, IntoVal, Symbol,
    };

    use crate::amm::calculate_amm_reward_share;
    use crate::test::set_sequence_number;

    #[test]
    fn test_add_amm_address() {
        let e = Env::default();
        e.mock_all_auths();

        let admin = Address::generate(&e);
        let amm_address = Address::generate(&e);
        let token = crate::test::create_token(&e, &admin);

        token.add_amm_address(&amm_address);
        assert_eq!(
            e.auths(),
            std::vec![(
                admin.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        token.address.clone(),
                        Symbol::new(&e, "add_amm_address"),
                        (&amm_address,).into_val(&e),
                    )),
                    sub_invocations: std::vec![],
                }
            )]
        );
    }

    #[test]
    fn test_remove_amm_address() {
        let e = Env::default();
        e.mock_all_auths();

        let admin = Address::generate(&e);
        let amm_address = Address::generate(&e);
        let token = crate::test::create_token(&e, &admin);

        token.remove_amm_address(&amm_address);
        assert_eq!(
            e.auths(),
            std::vec![(
                admin.clone(),
                AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        token.address.clone(),
                        Symbol::new(&e, "remove_amm_address"),
                        (&amm_address,).into_val(&e),
                    )),
                    sub_invocations: std::vec![],
                }
            )]
        );
    }
    #[test]
    #[should_panic(expected = "address is not passed kyc")]
    fn test_mint_not_allowed_for_amm() {
        let e = Env::default();
        e.mock_all_auths();
        let admin = Address::generate(&e);
        let amm_address = Address::generate(&e);
        let token = crate::test::create_token(&e, &admin);
        token.add_amm_address(&amm_address);
        token.mint(&amm_address, &1000);
    }

    #[test]
    #[should_panic(expected = "address is not passed kyc")]
    fn test_burn_not_allowed_for_amm() {
        let e = Env::default();
        e.mock_all_auths();
        let admin = Address::generate(&e);
        let amm_address = Address::generate(&e);
        let user = Address::generate(&e);
        let token = crate::test::create_token(&e, &admin);
        token.pass_kyc(&user);
        token.add_amm_address(&amm_address);
        token.mint(&user, &1000);
        token.transfer(&user, &amm_address, &100);
        token.burn(&amm_address, &100);
    }

    #[test]
    fn test_update_amm_depositor_balance() {
        let e = Env::default();
        e.mock_all_auths();

        let admin = Address::generate(&e);
        let amm_address = Address::generate(&e);
        let user1 = Address::generate(&e);
        let user2 = Address::generate(&e);
        let user3 = Address::generate(&e);
        let token = crate::test::create_token(&e, &admin);
        let blocks_per_reward: u32 = 28_800;
        let reward_rate: u32 = 1_00;
        token.set_reward_tick(&blocks_per_reward);
        token.set_reward_rate(&reward_rate);

        token.pass_kyc(&user1);
        token.pass_kyc(&user2);
        token.pass_kyc(&user3);
        token.add_amm_address(&amm_address);
        token.mint(&user1, &1000);
        token.mint(&user2, &1000);
        token.mint(&user3, &1000);

        token.transfer(&user1, &amm_address, &300);
        set_sequence_number(&e, 0);
        token.transfer(&user2, &amm_address, &400);
        set_sequence_number(&e, blocks_per_reward);
        token.transfer(&user3, &amm_address, &500);
        set_sequence_number(&e, blocks_per_reward * 2);

        token.transfer(&amm_address, &user1, &300);
        set_sequence_number(&e, blocks_per_reward * 3);
        token.transfer(&amm_address, &user2, &400);
        set_sequence_number(&e, blocks_per_reward * 4);
        token.transfer(&amm_address, &user3, &500);
        set_sequence_number(&e, blocks_per_reward * 5);

        token.claim_reward(&user1);
        token.claim_reward(&user2);
        token.claim_reward(&user3);

        assert_eq!(token.balance(&user1), 1008);
        assert_eq!(token.balance(&user2), 1009);
        assert_eq!(token.balance(&user3), 1019);
    }

    #[test]
    fn test_calculate_amm_reward_share() {
        // Test case where reward share is calculated correctly
        let total_reward = 1000;
        let total_balance = 500;
        let reward_share = calculate_amm_reward_share(total_reward, 100, total_balance);
        assert_eq!(reward_share, 200);

        // Test case where reward share is zero due to zero total balance
        let reward_share = calculate_amm_reward_share(total_reward, 100, 0);
        assert_eq!(reward_share, 0);

        // Test case where reward share is zero due to zero depositor balance
        let reward_share = calculate_amm_reward_share(total_reward, 0, total_balance);
        assert_eq!(reward_share, 0);
    }
}
