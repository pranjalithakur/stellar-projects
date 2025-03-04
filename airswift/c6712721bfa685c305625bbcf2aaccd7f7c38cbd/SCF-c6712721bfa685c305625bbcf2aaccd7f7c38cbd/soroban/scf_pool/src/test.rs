#![cfg(test)]
use crate::contract::{OfferPool, OfferPoolClient};
use crate::error::Error as ContractError;
use crate::test_util::{install_token_wasm, setup_pool, setup_tc, setup_test_token};
use soroban_sdk::events::Topics;
use soroban_sdk::map;
use soroban_sdk::{
    log, symbol_short, testutils::Address as _, testutils::Events, token, vec, Address, Env, Error,
    IntoVal, Map, String,
};

extern crate std;

#[test]
fn test_get_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);

    let offer_id = 1;
    let res = pool_client.try_get_offer(&offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    );
}

#[test]
fn test_initialize() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(&admin, &token_wasm_hash);

    assert_eq!(client.get_pool_tokens(), Map::new(&e))
}

#[test]
#[should_panic(expected = "already initialized")]
fn test_initialize_twice() {
    let e = Env::default();
    let contract_id = e.register_contract(None, OfferPool);
    let client = OfferPoolClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    let token_wasm_hash = install_token_wasm(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    client.initialize(&admin, &token_wasm_hash);
    client.initialize(&admin, &token_wasm_hash);
}

#[test]
fn test_add_pool_tokens() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client_1, _) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);

    let liquidity_token_1 = client.add_pool_token(&token_client_1.address);
    assert_eq!(
        client.get_pool_tokens(),
        map![
            &e,
            (token_client_1.address.clone(), liquidity_token_1.clone())
        ]
    );
    assert_eq!(
        client.get_ext_token(&liquidity_token_1.clone()),
        token_client_1.address.clone()
    );

    let (token_client_2, _) = setup_test_token(&e, &admin);
    let liquidity_token_2 = client.add_pool_token(&token_client_2.address);
    assert_eq!(
        client.get_pool_tokens(),
        map![
            &e,
            (token_client_1.address.clone(), liquidity_token_1.clone()),
            (token_client_2.address.clone(), liquidity_token_2.clone())
        ]
    );
    assert_eq!(
        client.get_ext_token(&liquidity_token_2.clone()),
        token_client_2.address.clone()
    );
}

#[test]
fn test_add_pool_token_duplicate() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);

    let liquidity_token = client.add_pool_token(&token_client.address);

    assert_eq!(
        client.get_pool_tokens(),
        map![&e, (token_client.address.clone(), liquidity_token.clone())]
    );
    assert_eq!(
        client.get_ext_token(&liquidity_token),
        token_client.address.clone()
    );

    // The second call to add_pool_token with the same ext_token should fail
    let res = client.try_add_pool_token(&token_client.address);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TokenExists as u32
        )))
    )
}

#[test]
fn test_deposit() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = client.add_pool_token(&token_client.address);
    let liquidity_token_client = token::Client::new(&e, &liquidity_token);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &token_client.address, &600000);
    assert_eq!(token_client.balance(&user.clone()), 400000);
    assert_eq!(liquidity_token_client.balance(&user.clone()), 600000);

    // Match the latest event in the event stream
    match e.events().all().last() {
        // If there is an event
        Some((contract_address, topics, data)) => {
            // Assert that the contract address matches the expected contract ID
            assert_eq!(contract_address, contract_id.clone());

            // Assert that the topics match the expected values (deposit event for a specific user)
            assert_eq!(
                topics,
                (
                    symbol_short!("deposit"),
                    user,
                    token_client.address,
                    liquidity_token
                )
                    .into_val(&e)
            );

            // Decode the event data and assert that it matches the expected value (600000)
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, 600000);
        }
        // If there are no events, panic with a descriptive message
        None => panic!("The event is not published"),
    }
}

#[test]
fn test_deposit_unsupported_token() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);

    let user = Address::generate(&e);
    let res = client.try_deposit(&user.clone(), &token_client.address, &1);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TokenNotSupported as u32
        )))
    );
}

#[test]
fn test_deposit_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);
    client.add_pool_token(&token_client.address);

    let user = Address::generate(&e);
    let res = client.try_deposit(&user.clone(), &token_client.address, &1);
    assert_eq!(res, Err(Ok(Error::from_contract_error(10))));
}

#[test]
fn test_withdraw() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = client.add_pool_token(&token_client.address);
    let liquidity_token_client = token::Client::new(&e, &liquidity_token);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &token_client.address, &600000);
    client.withdraw(&user.clone(), &liquidity_token.clone(), &100000);

    assert_eq!(token_client.balance(&user.clone()), 500000);
    assert_eq!(liquidity_token_client.balance(&user.clone()), 500000);

    //Test for the event
    //Get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            // Test the event contract address
            assert_eq!(contract_address, contract_id.clone());

            // Test the event topics
            assert_eq!(
                topics,
                (
                    symbol_short!("withdraw"),
                    user,
                    token_client.address,
                    liquidity_token
                )
                    .into_val(&e)
            );

            // Test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, 100000);
        }
        None => panic!("The event is not published"),
    }
}

#[test]
fn test_withdraw_unsupported_token() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);

    let liquidity_token = client.add_pool_token(&token_client.address);
    let other_token = e.register_stellar_asset_contract(admin.clone());

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    client.deposit(&user.clone(), &token_client.address, &600000);
    let res = client.try_withdraw(&user.clone(), &other_token, &100000);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::TokenNotSupported as u32
        )))
    );
}

#[test]
fn test_withdraw_invalid_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (client, _) = setup_pool(&e, &admin);
    let liquidity_token = client.add_pool_token(&token_client.address);

    let user = Address::generate(&e);
    token_admin_client.mint(&user.clone(), &1000000);
    let res = client.try_withdraw(&user.clone(), &liquidity_token.clone(), &1);
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let liquidity_token_client = token::Client::new(&e, &liquidity_token);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // offerer deposits ext token to receive liquidity tokens in exchange
    pool_client.deposit(&offerer.clone(), &token_client.address, &1000000);

    let offer_id = 1;

    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &600000,
        &tc_client.address,
        &1,
    );

    //Test for the event
    //Get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            // Test the event contract address
            assert_eq!(contract_address, contract_id.clone());

            // Test the event topics
            assert_eq!(
                topics,
                (symbol_short!("create"), offerer.clone(), 600000i128).into_val(&e)
            );

            // Test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, 1);
        }
        None => panic!("The event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    //test offer information
    assert_eq!(offer.from, offerer);
    assert_eq!(offer.amount, 600000);
    assert_eq!(offer.tc_contract, tc_client.address);
    assert_eq!(offer.tc_id, 1);
    assert_eq!(offer.status, 0);
    assert_eq!(liquidity_token_client.balance(&offerer.clone()), 400000);
}

#[test]
fn test_create_offer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &1000000);

    // offerer deposits ext token to receive liquidity tokens in exchange
    pool_client.deposit(&offerer.clone(), &token_client.address, &1000000);

    let offer_id = 1;

    // create_offer should fail because the offerer did not deposit enough ext_token
    let res = pool_client.try_create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &2000000,
        &tc_client.address,
        &1,
    );
    assert_eq!(res.is_err(), true);
}

#[test]
fn test_create_offer_duplicate() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let liquidity_token_client = token::Client::new(&e, &liquidity_token);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    // mint ext token to offerer
    token_admin_client.mint(&offerer, &2000000);

    // offerer deposits ext token to receive liquidity tokens in exchange
    pool_client.deposit(&offerer.clone(), &token_client.address, &2000000);

    let offer_id = 1;

    let res = pool_client.try_create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    assert_eq!(res.is_ok(), true);
    assert_eq!(liquidity_token_client.balance(&offerer.clone()), 1000000);
    let res = pool_client.try_create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferExist as u32
        )))
    );
}

#[test]
fn test_accept_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let liquidity_token_client = token::Client::new(&e, &liquidity_token.clone());
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );

    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    // setup TC
    tc_client.mint_original(&buyer, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.accept_offer(&buyer, &offer_id);

    //Test for the event
    //Get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            // Test the event contract address
            assert_eq!(contract_address, contract_id.clone());

            // Test the event topics
            assert_eq!(
                topics,
                (symbol_short!("accept"), buyer.clone()).into_val(&e)
            );

            // Test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, offer_id);
        }
        None => panic!("The event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);
    assert_eq!(liquidity_token_client.balance(&buyer), 1000000)
}

#[test]
fn test_accept_offer_nonexistent_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);

    let res = pool_client.try_accept_offer(&admin, &123);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    )
}

#[test]
fn test_accept_offer_nonexistent_tc() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    // try to accept an offer for a TC that was never minted
    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &0,
    );
    let res = pool_client.try_accept_offer(&admin, &offer_id);
    assert_eq!(res, Err(Ok(Error::from_contract_error(1))))
}

#[test]
fn test_accept_offer_not_tc_owner() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // other_user is not the owner of the TC
    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &0,
    );
    let res = pool_client.try_accept_offer(&other_user, &offer_id);
    assert_eq!(res, Err(Ok(Error::from_contract_error(5))));
}

#[test]
fn test_expire_accepted_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and accept the offer
    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &0,
    );
    pool_client.accept_offer(&tc_holder, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 2);

    // try to expire an accepted offer
    let res = pool_client.try_expire_offer(&admin, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferChanged as u32
        )))
    );
}

#[test]
fn test_expire_offer_nonexistent() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let (token_client, _) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);

    let res = pool_client.try_expire_offer(&admin, &123);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferEmpty as u32
        )))
    )
}

#[test]
fn test_expire_offer_as_admin() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    pool_client.expire_offer(&admin, &offer_id);

    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);
}

#[test]
fn test_expire_offer_as_owner() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    pool_client.expire_offer(&offerer, &offer_id);

    //test for the event
    //get the latest event
    match e.events().all().last() {
        Some((contract_address, topics, data)) => {
            //test the event contract address
            assert_eq!(contract_address, contract_id.clone());
            //test the event topics
            assert_eq!(
                topics,
                (symbol_short!("expire"), offerer.clone()).into_val(&e)
            );
            //test the event data
            let data_decoded: i128 = data.into_val(&e);
            assert_eq!(data_decoded, offer_id);
        }
        None => panic!("the event is not published"),
    }

    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);
}

#[test]
fn test_expire_offer_not_owned() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, contract_id) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    let offer_id = 1;
    let other_user = Address::generate(&e);
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    let res = pool_client.try_expire_offer(&other_user, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::NotAuthorized as u32
        )))
    )
}

#[test]
fn test_accept_expired_offer() {
    let e = Env::default();
    e.mock_all_auths();
    let admin = Address::generate(&e);
    let buyer = Address::generate(&e);
    let offerer = Address::generate(&e);
    let (token_client, token_admin_client) = setup_test_token(&e, &admin);
    let (pool_client, _) = setup_pool(&e, &admin);
    let liquidity_token = pool_client.add_pool_token(&token_client.address);
    let tc_client = setup_tc(
        &e,
        &admin,
        &buyer,
        &1000000,
        &1712793295,
        &token_client.address,
        &token_client.decimals(),
    );
    token_admin_client.mint(&offerer, &1000000);

    pool_client.deposit(&offerer, &token_client.address, &1000000);

    // setup TC
    let tc_holder = Address::generate(&e);
    tc_client.mint_original(&tc_holder, &String::from_str(&e, ""));

    // create and expire the offer
    let offer_id = 1;
    pool_client.create_offer(
        &offerer,
        &offer_id,
        &liquidity_token.clone(),
        &1000000,
        &tc_client.address,
        &1,
    );
    pool_client.expire_offer(&admin, &offer_id);
    let offer = pool_client.get_offer(&offer_id);
    assert_eq!(offer.status, 1);

    // try to accept an expired offer
    let res = pool_client.try_accept_offer(&tc_holder, &offer_id);
    assert_eq!(
        res,
        Err(Ok(Error::from_contract_error(
            ContractError::OfferChanged as u32
        )))
    );
}
