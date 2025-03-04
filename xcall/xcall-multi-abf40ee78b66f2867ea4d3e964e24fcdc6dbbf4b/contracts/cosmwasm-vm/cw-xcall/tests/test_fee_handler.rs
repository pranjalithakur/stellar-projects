use std::str::FromStr;

use cosmwasm_std::{
    from_json,
    testing::{mock_dependencies, mock_env, mock_info},
    to_json_binary, Addr, Coin, ContractResult, SystemResult, WasmQuery,
};
use cw_xcall::{msg::QueryMsg, state::CwCallService};
pub mod account;
use account::*;
use cw_xcall_lib::network_address::NetId;

#[test]
fn set_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let address = "xyz".to_string();

    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);

    contract
        .set_protocol_feehandler(deps.as_mut(), &info, address.clone())
        .unwrap();

    let result = contract.get_protocol_feehandler(deps.as_ref());
    assert_eq!(address, result);
}

#[test]
#[should_panic(expected = "OnlyAdmin")]
fn test_invalid_input() {
    let mut deps = mock_dependencies();
    let info = mock_info("user", &[Coin::new(1000, "ucosm")]);
    let address = "xyz".to_string();
    let cw_callservice = CwCallService::new();

    cw_callservice
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let result = cw_callservice.query_admin(deps.as_ref().storage).unwrap();

    assert_eq!(result, admin_one().to_string());

    cw_callservice
        .set_protocol_feehandler(deps.as_mut(), &info, address)
        .unwrap();
}

#[test]
fn get_protocol_fee_handler() {
    let mut deps = mock_dependencies();
    let address = "xyz".to_string();

    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();
    contract
        .fee_handler()
        .save(&mut deps.storage, &address)
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_feehandler(deps.as_mut(), &info, address)
        .unwrap();
    let result = contract.get_protocol_feehandler(deps.as_ref());
    let env = mock_env();

    assert_eq!("xyz", result);
    let result = contract
        .query(deps.as_ref(), env, QueryMsg::GetProtocolFeeHandler {})
        .unwrap();
    let result: String = from_json(result).unwrap();
    assert_eq!("xyz".to_string(), result);
}

#[test]
fn set_protocol_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let mut contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();

    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "uconst")]);
    let result = contract.set_protocol_fee(deps.as_mut(), info.clone(), value);
    assert_eq!(result.unwrap().attributes.len(), 1);
    let env = mock_env();
    let result = contract
        .execute(
            deps.as_mut(),
            env,
            info,
            cw_xcall_lib::xcall_msg::ExecuteMsg::SetProtocolFee { value },
        )
        .unwrap();
    println!("{result:?}");
    assert_eq!(result.attributes.len(), 1);
}

#[test]
fn get_protocol_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_fee(deps.as_mut(), info, value)
        .unwrap();
    let result = contract.get_protocol_fee(deps.as_ref().storage);
    assert_eq!("123", result.to_string());
    let env = mock_env();
    let result = contract
        .query(deps.as_ref(), env, QueryMsg::GetProtocolFee {})
        .unwrap();
    let result: u128 = from_json(result).unwrap();
    assert_eq!("123", result.to_string());
}

#[test]
fn get_fee() {
    let mut deps = mock_dependencies();
    let value = 123;
    let contract = CwCallService::new();

    contract
        .set_admin(
            deps.as_mut().storage,
            Addr::unchecked(admin_one().to_string()),
        )
        .unwrap();
    let env = mock_env();
    let info = mock_info(&admin_one().to_string(), &[Coin::new(1000, "ucosm")]);
    contract
        .set_protocol_fee(deps.as_mut(), info.clone(), value)
        .unwrap();
    contract
        .set_default_connection(
            deps.as_mut(),
            info,
            NetId::from_str("icon").unwrap(),
            Addr::unchecked("connectionaddress"),
        )
        .unwrap();
    deps.querier.update_wasm(|r| match r {
        WasmQuery::Smart {
            contract_addr: _,
            msg: _,
        } => SystemResult::Ok(ContractResult::Ok(to_json_binary(&100_u128).unwrap())),
        _ => todo!(),
    });
    let result = contract
        .get_fee(
            deps.as_ref(),
            NetId::from_str("icon").unwrap(),
            true,
            vec![],
        )
        .unwrap();
    assert_eq!("223", result.to_string());
    let result = contract
        .query(
            deps.as_ref(),
            env,
            QueryMsg::GetFee {
                nid: NetId::from_str("icon").unwrap(),
                rollback: true,
                sources: None,
            },
        )
        .unwrap();
    let result: u128 = from_json(result).unwrap();
    assert_eq!("223", result.to_string());
}
