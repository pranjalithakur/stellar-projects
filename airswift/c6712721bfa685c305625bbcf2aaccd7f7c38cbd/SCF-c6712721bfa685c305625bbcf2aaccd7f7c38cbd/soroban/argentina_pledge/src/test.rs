#![cfg(test)]
use soroban_sdk::{testutils::Address as _, vec, Address, Env, Error, String};

use crate::contract::{TokenizedCertificate, TokenizedCertificateClient};
use crate::errors::Error as ContractError;
use crate::test_util::{set_ledger_timestamp, setup_test_tc_contract, setup_test_token};

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, TokenizedCertificate);
    let client = TokenizedCertificateClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(&admin, &token_client.address, &0);

    assert_eq!(client.get_ext_token(), (token_client.address, 0));
}

#[test]
fn test_mint() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(
        &1000000,
        &1641024000,
        &vec![
            &e,
            String::from_str(&e, "a"),
            String::from_str(&e, "b"),
            String::from_str(&e, "c"),
        ],
    );

    assert_eq!(tc_client.get_amount(&0), 1000000);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);
    assert_eq!(
        tc_client.get_file_hashes(&0),
        vec![
            &e,
            String::from_str(&e, "a"),
            String::from_str(&e, "b"),
            String::from_str(&e, "c"),
        ],
    );

    assert_eq!(
        tc_client.try_get_owner(&1),
        Err(Ok(Error::from_contract_error(
            ContractError::NotFound as u32
        )))
    );
}

#[test]
fn test_pledge() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user);
}

#[test]
fn test_pledge_insufficient_balance() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10);
    let res = tc_client.try_pledge(&user.clone(), &0);
    assert_eq!(res.is_ok(), false);
    assert_eq!(tc_client.get_owner(&0), tc_client.address); // ensure that the owner hasn't changed
}

#[test]
fn test_transfer() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user);

    // transfer to another user
    let user2 = Address::generate(&e);
    tc_client.transfer(&user.clone(), &user2.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user2);
}

#[test]
fn test_transfer_not_owned() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);

    // try to transfer while the contract still owns TC #0
    let user = Address::generate(&e);
    let user2 = Address::generate(&e);
    let res = tc_client.try_transfer(&user.clone(), &user2.clone(), &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotOwned as u32
        )))
    );
    assert_eq!(tc_client.get_owner(&0), tc_client.address);
}

#[test]
fn test_appr_transfer_from() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    assert_eq!(tc_client.get_owner(&0), tc_client.address);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user);

    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    // initial transfer attempt from user2 to user3 should fail since user2 is not approved to transfer from user
    let res = tc_client.try_transfer(&user2.clone(), &user3.clone(), &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotOwned as u32
        )))
    );

    // transfer from user2 to user3 should succeed this time
    tc_client.appr(&user.clone(), &user2.clone(), &0);
    tc_client.transfer_from(&user2.clone(), &user.clone(), &user3.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user3);
}

#[test]
fn test_appr_all_transfer_from() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);
    tc_client.mint(&2000000, &1641024000, &vec![&e]);
    tc_client.mint(&3000000, &1641024000, &vec![&e]);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);
    tc_client.pledge(&user.clone(), &1);
    tc_client.pledge(&user.clone(), &2);
    assert_eq!(tc_client.get_owner(&0), user);
    assert_eq!(tc_client.get_owner(&1), user);
    assert_eq!(tc_client.get_owner(&2), user);

    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    // initial transfer attempt from user2 to user3 should fail since user2 is not approved to transfer from user
    let res = tc_client.try_transfer(&user2.clone(), &user3.clone(), &1);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotOwned as u32
        )))
    );

    // transfer from user2 to user3 should succeed this time
    tc_client.appr_all(&user.clone(), &user2.clone(), &true);
    tc_client.transfer_from(&user2.clone(), &user.clone(), &user3.clone(), &0);
    tc_client.transfer_from(&user2.clone(), &user.clone(), &user3.clone(), &1);
    assert_eq!(tc_client.get_owner(&0), user3);
    assert_eq!(tc_client.get_owner(&1), user3);

    // set approval_all to false again, attempting to transfer TC #2 should fail
    tc_client.appr_all(&user.clone(), &user2.clone(), &false);
    let res = tc_client.try_transfer(&user2.clone(), &user3.clone(), &2);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotOwned as u32
        )))
    );
}

#[test]
fn test_redeem_too_early() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);

    let user2 = Address::generate(&e);
    tc_client.transfer(&user.clone(), &user2.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), user2);

    let res = tc_client.try_redeem(&user2.clone(), &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotRedeemable as u32
        )))
    );
}

#[test]
fn test_redeem() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);

    let user = Address::generate(&e);
    token_admin_client.mint(&user, &10000000);
    tc_client.pledge(&user, &0);

    let user2 = Address::generate(&e);
    tc_client.transfer(&user, &user2, &0);
    assert_eq!(tc_client.get_owner(&0), user2);

    set_ledger_timestamp(&e, 1641024001);
    let test = e.ledger().timestamp();
    tc_client.redeem(&user2.clone(), &0);
    assert_eq!(token_client.balance(&user2.clone()), 1000000);
    assert_eq!(tc_client.try_get_owner(&0).is_ok(), false);
}

#[test]
fn test_redeem_not_owned() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let tc_client = setup_test_tc_contract(&e, &admin, &token_client.address, &0);
    e.mock_all_auths();

    tc_client.mint(&1000000, &1641024000, &vec![&e]);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &10000000);
    tc_client.pledge(&user.clone(), &0);

    let user2 = Address::generate(&e);

    let res = tc_client.try_redeem(&user2.clone(), &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotOwned as u32
        )))
    );
}
