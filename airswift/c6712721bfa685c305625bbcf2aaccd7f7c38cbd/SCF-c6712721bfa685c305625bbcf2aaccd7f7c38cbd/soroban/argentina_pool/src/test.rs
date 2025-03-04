#![cfg(test)]
use soroban_sdk::{testutils::Address as _, token, Address, Env, Error, String, Vec};

use crate::{
    contract::LiquidityPoolClient,
    errors::Error as ContractError,
    loan::LoanStatus,
    test_util::{install_token_wasm, setup_pool, setup_tc, setup_test_token},
    LiquidityPool,
};

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );

    assert_eq!(
        client.get_ext_token(),
        (token_client.address.clone(), token_client.decimals())
    );
    assert_eq!(client.get_pool_rate(), 2);
    assert_eq!(client.try_get_liquidity_token().is_ok(), true);
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, LiquidityPool);
    let client = LiquidityPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );

    client.initialize(
        &admin,
        &token_wasm_hash,
        &token_client.address,
        &token_client.decimals(),
        &2,
    );
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    assert_eq!(token_client.balance(&user.clone()), 400000);
    assert_eq!(
        token::Client::new(&e, &client.get_liquidity_token()).balance(&user.clone()),
        600000
    );
}

#[test]
fn test_deposit_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    let res = client.try_deposit(&user.clone(), &1);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}

#[test]
fn test_withdraw() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &600000);
    client.withdraw(&user.clone(), &100000);

    assert_eq!(token_client.balance(&user.clone()), 500000);
    assert_eq!(
        token::Client::new(&e, &client.get_liquidity_token()).balance(&user.clone()),
        500000
    );
}

#[test]
fn test_withdraw_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    let res = client.try_withdraw(&user.clone(), &1);
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);

    // trying to create a loan before depositing for liquidity tokens should fail
    let res = client.try_create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(res.is_err(), true);

    // successful call
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(client.get_loan_rate(&loan_id), 0);
    assert_eq!(client.get_loan_creditor(&loan_id), creditor.clone());
    assert_eq!(client.get_loan_tc(&loan_id), (tc_client.address.clone(), 0));
    assert_eq!(client.get_loan_amount(&loan_id), 1000000);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Pending as u32);

    // Calling with duplicate loan id should fail
    let res = client.try_create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotEmpty as u32
        )))
    );
}

#[test]
fn test_cancel_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    let liquidity_token_client = token::Client::new(&e, &client.get_liquidity_token());

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Pending as u32);
    assert_eq!(liquidity_token_client.balance(&creditor.clone()), 0);
    client.cancel_loan_offer(&loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Closed as u32);
    assert_eq!(
        liquidity_token_client.balance(&creditor.clone()),
        10000000000000
    );
}

#[test]
fn test_accept_loan_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    let liquidity_token_client = token::Client::new(&e, &client.get_liquidity_token());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    client.accept_loan_offer(&borrower.clone(), &loan_id);
    assert_eq!(client.get_loan_borrower(&loan_id), borrower.clone());
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Active as u32);
    assert_eq!(tc_client.get_owner(&0), creditor.clone());
    assert_eq!(
        liquidity_token_client.balance(&borrower.clone()),
        10000000000000
    );
    assert_eq!(liquidity_token_client.balance(&creditor.clone()), 0);

    // it should not be possible to cancel a loan offer once it has been accepted
    let res = client.try_cancel_loan_offer(&loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
}

#[test]
fn test_payoff_loan() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    let liquidity_token_client = token::Client::new(&e, &client.get_liquidity_token());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.deposit(&creditor.clone(), &10000000000000);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);

    // it should not be possible to pay off the loan before the offer is accepted
    let res = client.try_payoff_loan(&borrower.clone(), &loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    client.payoff_loan(&borrower.clone(), &loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Paid as u32);
    assert_eq!(liquidity_token_client.balance(&borrower.clone()), 0);
    assert_eq!(
        liquidity_token_client.balance(&client.address),
        10000000000000
    );
}

#[test]
fn test_payoff_loan_with_interest() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    let liquidity_token_client = token::Client::new(&e, &client.get_liquidity_token());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10000000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.deposit(&creditor.clone(), &10000000000000);
    client.set_rate(&2);
    assert_eq!(client.get_pool_rate(), 2);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);
    assert_eq!(client.get_loan_rate(&loan_id), 2);
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    token_admin_client.mint(&borrower.clone(), &200000000000);
    client.deposit(&borrower.clone(), &200000000000);
    assert_eq!(
        liquidity_token_client.balance(&borrower.clone()),
        10200000000000
    );
    assert_eq!(client.get_payoff_amount(&loan_id), 10200000000000);
    client.payoff_loan(&borrower.clone(), &loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Paid as u32);
    assert_eq!(liquidity_token_client.balance(&borrower.clone()), 0);
    assert_eq!(
        liquidity_token_client.balance(&client.address),
        10200000000000
    );
}

#[test]
fn test_close_loan() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let client = setup_pool(&e, &admin, &token_client.address, &token_client.decimals());
    let tc_client = setup_tc(&e, &admin, &token_client.address, &token_client.decimals());
    let liquidity_token_client = token::Client::new(&e, &client.get_liquidity_token());
    e.budget().reset_default();

    let borrower = Address::generate(&e);
    let creditor = Address::generate(&e);
    let loan_id = 123i128;
    token_admin_client.mint(&borrower.clone(), &10500000000000);
    token_admin_client.mint(&creditor.clone(), &10000000000000);
    tc_client.mint(&1000000, &1641024000, &Vec::<String>::new(&e));
    tc_client.pledge(&borrower.clone(), &0);
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
    client.deposit(&creditor.clone(), &10000000000000);
    client.set_rate(&5);
    client.create_loan_offer(&creditor.clone(), &loan_id, &tc_client.address, &0);

    e.budget().reset_default();
    // it should not be possible to close the loan before the offer is accepted
    let res = client.try_close_loan(&borrower.clone(), &loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
    client.accept_loan_offer(&borrower.clone(), &loan_id);

    // it should also not be possible to close the loan before the offer is paid off
    let res = client.try_close_loan(&borrower.clone(), &loan_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::InvalidStatus as u32
        )))
    );
    client.deposit(&borrower.clone(), &500000000000);
    client.payoff_loan(&borrower.clone(), &loan_id);
    client.close_loan(&borrower.clone(), &loan_id);
    assert_eq!(client.get_loan_status(&loan_id), LoanStatus::Closed as u32);
    assert_eq!(
        liquidity_token_client.balance(&creditor.clone()),
        10500000000000
    );
    assert_eq!(tc_client.get_owner(&0), borrower.clone());
}
