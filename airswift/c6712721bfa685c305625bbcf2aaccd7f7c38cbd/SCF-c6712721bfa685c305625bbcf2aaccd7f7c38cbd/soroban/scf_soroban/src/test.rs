#![cfg(test)]
use crate::contract::{TokenizedCertificate, TokenizedCertificateClient};

use crate::storage_types::SplitRequest;
use crate::test_util::setup_test_token;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::{
    testutils::Address as _, token::Client as TokenClient, token::StellarAssetClient, vec, Address,
    Env, String,
};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let total_amount: u32 = 1000000;
    let end_time = 1672531200; // 2023-01-01 00:00:00 UTC+0

    client.initialize(&admin, &buyer, &total_amount, &end_time);
    assert_eq!(admin, client.admin());
    // TODO: getters for other fields?
}

#[test]
fn test_mint_original() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));
    assert_eq!(1000000, client.amount(&0));
    assert_eq!(0, client.parent(&0));
    assert_eq!(false, client.is_disabled(&0));
    assert_eq!(vec![&env, String::from_str(&env, "a")], client.vc(&0));
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_mint_original_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));

    client.mint_original(&to, &String::from_str(&env, "a")); // should panic
}

#[test]
fn test_split() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 300000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );

    client.add_vc(&1, &String::from_str(&env, "b"));
    client.add_vc(&3, &String::from_str(&env, "c"));
    client.add_vc(&3, &String::from_str(&env, "d"));

    assert_eq!(300000, client.amount(&1));
    assert_eq!(client.address, client.owner(&1));
    assert_eq!(0, client.parent(&1));
    assert_eq!(vec![&env, String::from_str(&env, "b")], client.vc(&1));

    assert_eq!(500000, client.amount(&2));
    assert_eq!(client.address, client.owner(&2));
    assert_eq!(0, client.parent(&2));
    assert_eq!(vec![&env], client.vc(&2));

    assert_eq!(200000, client.amount(&3));
    assert_eq!(to, client.owner(&3));
    assert_eq!(0, client.parent(&3));
    assert_eq!(
        vec![
            &env,
            String::from_str(&env, "c"),
            String::from_str(&env, "d")
        ],
        client.vc(&3)
    );

    assert_eq!(true, client.is_disabled(&0));
}

#[test]
fn test_split_nested() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 800000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(800000, client.amount(&1));

    // remaining token id 2 is worth 200k and belongs to buyer

    client.split(
        &1,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(500000, client.amount(&3));
    assert_eq!(client.address, client.owner(&3));
    assert_eq!(1, client.parent(&3));

    assert_eq!(300000, client.amount(&4));
    assert_eq!(client.address, client.owner(&4));
    assert_eq!(1, client.parent(&4));
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_split_twice() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
        ],
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #6)")]
fn test_split_exceed() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(1000000, client.amount(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 500001,
                to: to.clone(),
            },
        ],
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_split_empty() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    client.split(&0, &vec![&env]);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let acc1 = Address::generate(&env);
    let acc2 = Address::generate(&env);
    client.mint_original(&acc1, &String::from_str(&env, "a"));
    assert_eq!(acc1, client.owner(&0));

    client.transfer(&acc1, &acc2, &0);
    assert_eq!(acc2, client.owner(&0));
}

#[test]
fn test_burn() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    client.mint_original(&admin, &String::from_str(&env, "a"));
    let res = client.try_owner(&0);
    assert_eq!(res.is_ok(), true);

    client.burn(&0);
    let res2 = client.try_owner(&0);
    assert_eq!(res2.is_ok(), false);
}

#[test]
fn test_pay_off() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    // setup fake external token
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = StellarAssetClient::new(&env, ext_token_addr);
    ext_admin.mint(&buyer, &10000000000000);

    client.set_external_token_provider(&ext_token_addr, &7);
    assert_eq!(client.check_paid(), false);

    client.pay_off(&buyer);
    assert_eq!(client.check_paid(), true);
}

#[test]
fn test_check_expired() {
    let env = Env::default();
    env.ledger().with_mut(|li| li.timestamp = 1640995200); // 2022-01-01 00:00:00 UTC+0
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    assert_eq!(client.check_expired(), false);

    env.ledger().with_mut(|li| li.timestamp = 1672617600); // 2023-01-02 00:00:00 UTC +0
    assert_eq!(client.check_expired(), true);
}

#[test]
fn test_expire_auto_transfer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    let to2 = Address::generate(&env);
    let to3 = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 500000,
                to: to2.clone(),
            },
            SplitRequest {
                amount: 100000,
                to: to2.clone(),
            },
        ],
    );
    client.sign_off(&1);
    client.split(
        &1,
        &vec![
            &env,
            SplitRequest {
                amount: 200000,
                to: to3.clone(),
            },
            SplitRequest {
                amount: 50000,
                to: to3.clone(),
            },
        ],
    );
    client.sign_off(&4);
    assert_eq!(to2, client.owner(&1));
    assert_eq!(client.address, client.owner(&2));
    assert_eq!(to, client.owner(&3));
    assert_eq!(to3, client.owner(&4));
    assert_eq!(client.address, client.owner(&5));
    assert_eq!(to2, client.owner(&6));

    env.ledger().with_mut(|li| li.timestamp = 1672617600); // 2023-01-02 00:00:00 UTC +0
    assert_eq!(client.check_expired(), true);
    assert_eq!(to, client.owner(&2));
    assert_eq!(to2, client.owner(&5));
}

#[test]
fn test_redeem() {
    // setup env with specific timestamp
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    // setup fake external token and pay the contract
    let ext_token_addr = &env.register_stellar_asset_contract(admin.clone());
    let ext_admin = StellarAssetClient::new(&env, ext_token_addr);
    ext_admin.mint(&buyer, &10000000000000);
    let ext_client = TokenClient::new(&env, ext_token_addr);
    ext_client.mock_all_auths_allowing_non_root_auth();

    let supplier = Address::generate(&env);
    client.mint_original(&supplier, &String::from_str(&env, "a"));
    assert_eq!(supplier, client.owner(&0));

    // setup preconditions, and redeem should fail before all preconditions are met
    client.set_external_token_provider(&ext_token_addr, &7);
    assert_eq!(client.try_redeem(&0).is_err(), true);
    client.check_paid();
    assert_eq!(client.try_redeem(&0).is_err(), true);
    env.ledger().with_mut(|li| li.timestamp = 1672617600); // 2023-01-02 00:00:00 UTC +0
    client.check_expired();

    assert_eq!(ext_client.balance(&supplier), 0);

    client.pay_off(&buyer);
    client.redeem(&0);

    // check balance was transferred
    assert_eq!(ext_client.balance(&supplier), 10000000000000);

    // check TC was burned
    assert_eq!(client.try_owner(&0).is_err(), true)
}

#[test]
fn test_sign_off() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    client.mint_original(&to, &String::from_str(&env, "a"));
    assert_eq!(to, client.owner(&0));

    let split_req = SplitRequest {
        amount: 600000,
        to: to.clone(),
    };
    client.split(&0, &vec![&env, split_req.clone()]);
    assert_eq!(client.address, client.owner(&1));
    assert_eq!(to, client.recipient(&1));
    client.sign_off(&1);
    assert_eq!(to, client.owner(&1));
}

#[test]
fn test_get_all_owned() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let buyer = Address::generate(&env);
    let client = setup_test_token(&env, &admin, &buyer);

    let to = Address::generate(&env);
    assert_eq!(vec![&env], client.get_all_owned(&to));

    client.mint_original(&to, &String::from_str(&env, "a"));

    assert_eq!(vec![&env, 0], client.get_all_owned(&to));

    client.split(
        &0,
        &vec![
            &env,
            SplitRequest {
                amount: 100000,
                to: to.clone(),
            },
            SplitRequest {
                amount: 200000,
                to: to.clone(),
            },
        ],
    );
    assert_eq!(vec![&env, 3], client.get_all_owned(&to));

    client.sign_off(&1);
    client.sign_off(&2);
    assert_eq!(vec![&env, 1, 2, 3], client.get_all_owned(&to));
}
