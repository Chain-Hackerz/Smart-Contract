use core::time;
use std::thread;

use crate::contract::{execute, instantiate, query};
use crate::msg::*;
use crate::state::*;

use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{
    coin, coins, from_binary, to_binary, Addr, BankMsg, CosmosMsg, SubMsg, Uint128, WasmMsg, Response, StdResult,
};

#[test]
fn test_initialization() {
    let creator = mock_info("creator", &[]);
    let mut deps = mock_dependencies();

    let inst_msg = InstantiateMsg {
    };

    let initialization_check = instantiate(deps.as_mut(), mock_env(), creator, inst_msg).unwrap();
    let actual = Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", mock_info("creator", &[]).sender);
    
    assert_eq!(actual.messages, initialization_check.messages);

    //check if state matches sender
    let res_query_config = query(deps.as_ref(), mock_env(), QueryMsg::GetInput {}).unwrap();
    
    let config: InputMsg = from_binary(&res_query_config).unwrap();
    let actual:InputMsg = InputMsg{ inputs: Vec::new() };

    assert_eq!(actual, config);
}

#[test]
fn test_action_fun_time(){
    let creator = mock_info("creator", &[]);
    let mut deps = mock_dependencies();

    let inst_msg = InstantiateMsg {
    };

    let initialization_check = instantiate(deps.as_mut(), mock_env(), creator.clone(), inst_msg).unwrap();
    let actual = Response::new()
    .add_attribute("method", "instantiate")
    .add_attribute("owner", mock_info("creator", &[]).sender);
    
    assert_eq!(actual.messages, initialization_check.messages);

    //check if state matches sender
    let res_query_config = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetInput {}
    ).unwrap();
    
    let config: InputMsg = from_binary(&res_query_config).unwrap();
    let actual:InputMsg = InputMsg{ inputs: Vec::new() };

    assert_eq!(actual, config);
    let vec = vec![
        Action::new(Input::key_down{button: 10}),
        {
            thread::sleep(time::Duration::from_millis(10));
            Action::new(Input::key_up{button: 10})}
        ];
    let executeMsg = ExecuteMsg::append(
        vec.clone()
        );

    execute(
        deps.as_mut(),
        mock_env(),
        creator, 
        executeMsg
    ).unwrap();

    let res_query_config = query(
        deps.as_ref(),
        mock_env(),
        QueryMsg::GetInput {}
    ).unwrap();

    let config: InputMsg = from_binary(&res_query_config).unwrap();
    let actual:InputMsg = InputMsg{ inputs: vec };

    assert_eq!(actual, config);
}