#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::state::Action;

use crate::error::ContractError;
use crate::msg::{ ExecuteMsg, InstantiateMsg, QueryMsg, InputMsg};

use crate::state::{Data, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:first-project";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let vec:Vec<Action> = Vec::new();
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &Data{input: vec})?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::append(input_keys) => {
            STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {

                for i in 0..input_keys.len() {
                    state.input.push(input_keys[i].clone());
                }

                Ok(state)
            })?;
            Ok(Response::new().add_attribute("method", "append")
                   .add_attribute("status", "good job"),)
        },
        ExecuteMsg::stats => {
            let keys = STATE.load(deps.storage)?.input;

            let mut iter = keys.iter();
            let mut t = iter.next().unwrap().time_stamp;

            let delta_time:Vec<i64> = iter.map(|val| -> i64 {
                let deltaT:i64 = val.time_stamp - &t;
                t = val.time_stamp;
                return deltaT;
            }).collect();

            let mut sum:i64 = 0;

            if delta_time.iter().any(|val| { sum = &sum + val; val <= &0 }) {
                return Err(ContractError::CustomError{val: "rejected".to_string()});
            }

            return Ok(
                Response::new()
                    .add_attribute("method", "verify")
                    .add_attribute("status", "good job")
                    .add_attribute("delta_time", format!("\"{:?}\"", delta_time)),
            );
        },
    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
       QueryMsg::GetInput {} => to_binary(&query_inputs(deps)?),
    }
}

fn query_inputs(deps: Deps) -> StdResult<InputMsg> {
    let state = STATE.load(deps.storage)?;
    Ok(InputMsg{inputs: state.input})
}