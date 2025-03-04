#![cfg(test)]
extern crate std;

use soroban_sdk::testutils::{Ledger, LedgerInfo};
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal, Symbol,
};

use crate::{contract::ExcellarToken, ExcellarTokenClient};

pub fn create_token<'a>(e: &Env, admin: &Address) -> ExcellarTokenClient<'a> {
    let token = ExcellarTokenClient::new(e, &e.register_contract(None, ExcellarToken {}));
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin1);
    token.pass_kyc(&user1);
    token.pass_kyc(&user2);
    token.pass_kyc(&user3);

    token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500);

    token.transfer(&user1, &user2, &600);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);

    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user3), 300);

    token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    // Increase to 500
    token.approve(&user2, &user3, &500, &200);
    assert_eq!(token.allowance(&user2, &user3), 500);
    token.approve(&user2, &user3, &0, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0);
}

#[test]
fn test_mint() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&user1);
    token.pass_kyc(&user2);

    token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);
}

#[test]
fn test_claim_reward() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&user1);
    token.mint(&user1, &1000);
    token.claim_reward(&user1);

    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "claim_reward"),
                    (&user1,).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&user1);
    token.pass_kyc(&user2);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.burn(&user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );

    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 0);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&user1);
    token.pass_kyc(&user2);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&user1);
    token.pass_kyc(&user3);
    token.pass_kyc(&user2);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.initialize(&admin, &10, &"name".into_val(&e), &"symbol".into_val(&e));
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = ExcellarTokenClient::new(&e, &e.register_contract(None, ExcellarToken {}));
    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1),
        &"name".into_val(&e),
        &"symbol".into_val(&e),
    );
}

#[test]
fn test_zero_allowance() {
    // Here we test that transfer_from with a 0 amount does not create an empty allowance
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let spender = Address::generate(&e);
    let from = Address::generate(&e);
    let token = create_token(&e, &admin);
    token.pass_kyc(&spender);

    token.transfer_from(&spender, &from, &spender, &0);
    assert!(token.get_allowance(&from, &spender).is_none());
}

#[test]
#[should_panic(expected = "address is not passed kyc")]
fn test_not_pass_kyc() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let from = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.claim_reward(&from);
}

#[test]
#[should_panic(expected = "address is blacklisted")]
fn test_blacklisted() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let from = Address::generate(&e);
    let to = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.pass_kyc(&to);
    token.blacklist(&to);

    token.transfer(&from, &to, &0);
}

#[test]
fn test_zero_transfer() {
    // Here we test that transfer with a 0 amount does not create an empty balance
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let from = Address::generate(&e);
    let to = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.pass_kyc(&to);
    token.transfer(&from, &to, &0);

    assert_eq!(token.balance(&to), 0);
}

#[test]
fn test_pass_kyc() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.pass_kyc(&user);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("pass_kyc"),
                    (&user,).into_val(&e),
                )),
                sub_invocations: std::vec![],
            }
        )]
    );
}

pub fn set_sequence_number(e: &Env, sequence_number: u32) {
    e.ledger().set(LedgerInfo {
        timestamp: 12345,
        protocol_version: 1,
        sequence_number,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 16,
        max_entry_ttl: 100_000,
    });
}
#[test]
fn test_transfer_with_reward() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);
    let blocks_per_reward: u32 = 28_800;
    let reward_rate: u32 = 1_00;

    token.set_reward_tick(&blocks_per_reward);
    token.set_reward_rate(&reward_rate);

    token.pass_kyc(&user1);
    token.pass_kyc(&user2);

    set_sequence_number(&e, 0);
    token.mint(&user1, &1000);
    token.mint(&user2, &1000);

    set_sequence_number(&e, blocks_per_reward);
    token.transfer(&user1, &user2, &300);
    assert_eq!(token.balance(&user1), 700);
    assert_eq!(token.balance(&user2), 1300);

    set_sequence_number(&e, blocks_per_reward * 2);
    token.transfer(&user1, &user2, &300);
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 1600);

    set_sequence_number(&e, blocks_per_reward * 3);
    token.transfer(&user2, &user1, &600);
    token.claim_reward(&user1);
    token.claim_reward(&user2);
    assert_eq!(token.balance(&user1), 1021);
    assert_eq!(token.balance(&user2), 1039);
}

#[test]
fn test_transfers_burn_with_reward() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);
    let blocks_per_reward: u32 = 28_800;
    let reward_rate: u32 = 1_00;

    token.set_reward_tick(&blocks_per_reward);
    token.set_reward_rate(&reward_rate);
    token.pass_kyc(&user1);
    token.pass_kyc(&user2);

    set_sequence_number(&e, 0);
    token.mint(&user1, &1000);
    token.mint(&user2, &1000);

    set_sequence_number(&e, blocks_per_reward);
    token.transfer(&user1, &user2, &300);
    assert_eq!(token.balance(&user1), 700);
    assert_eq!(token.balance(&user2), 1300);

    set_sequence_number(&e, blocks_per_reward * 2);
    token.burn(&user1, &300);
    token.claim_reward(&user1);
    token.claim_reward(&user2);
    assert_eq!(token.balance(&user1), 417);
    assert_eq!(token.balance(&user2), 1310);
}
