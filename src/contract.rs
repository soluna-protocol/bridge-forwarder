#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, CosmosMsg, 
  WasmMsg, WasmQuery, QueryRequest, Uint128,
};
use cw20::Cw20ExecuteMsg;
use terraswap::asset::{
  Asset,
  AssetInfo,
};

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TimeResponse};
use crate::state::{Config, CONFIG, TimeInfo, TIME};
use crate::pool_msg;
use crate::pool_resp;
use crate::bridge_msg;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
      pause: false,
      owner: deps.api.addr_canonicalize(info.sender.as_str())?,
      receiver: deps.api.addr_canonicalize(msg.receiver.as_str())?,
      bank: deps.api.addr_canonicalize(msg.bank.as_str())?,
      bridge:  deps.api.addr_canonicalize(msg.bridge.as_str())?,
      target: msg.target
    };

    CONFIG.save(deps.storage, &config)?;

    let time = TimeInfo {
      last_updated_time: env.block.time.seconds()
    };

    TIME.save(deps.storage, &time)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Earn {} => try_earn(deps),
        ExecuteMsg::Time {} => try_time(deps, env),
        ExecuteMsg::UpdateConfig { pause, owner, receiver, bank, bridge, target }
          => try_update_config(deps, info, pause, owner, receiver, bank, bridge, target),
        ExecuteMsg::Bridge { }
          => try_bridge(deps, info),
        // ExecuteMsg::ApproveBridge { amount } => try_approve( deps, info, amount),
    }
}

pub fn try_earn(deps: DepsMut) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if config.pause {
      return Err(ContractError::Paused {});
    }

    // let token_balance = token::balance_of(
    //   deps.as_ref(),
    //   deps.api.addr_humanize(&config.)
    // )

    // Ok(Response::new()
    //     .add_message(CosmosMsg::Bank(BankMsg::IncreaseAllowance {
    //       spender: 
    //       amount
    //     }))
    // .add_attribute("method", "try_do"))
    Ok(Response::new()
      .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps
          .api
          .addr_humanize(&config.bank)
          .unwrap()
          .to_string(),
        msg: to_binary(&pool_msg::ExecuteMsg::Earn {})?,
        funds: vec![],
      })))
}

pub fn try_time(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
  let mut time: TimeInfo = TIME.load(deps.storage)?;

  time.last_updated_time = env.block.time.seconds();

  TIME.save(deps.storage, &time)?;

  Ok(Response::new().add_attribute("action", "update_time"))
}

pub fn try_bridge(
  deps: DepsMut,
  _info: MessageInfo,
) -> Result<Response, ContractError> {
  let state : Config = CONFIG.load(deps.storage)?;

  if state.pause {
    return Err(ContractError::Paused {});
  }

  let reward : pool_resp::ClaimableRewardResponse = deps.querier.query(
    &QueryRequest::Wasm(WasmQuery::Smart {
      contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
      msg: to_binary(&pool_msg::QueryMsg::ClaimableReward {})?,
    }))?;

  let amount = Uint128::from(reward.amount);

  let res : pool_resp::ConfigResponse = deps.querier.query(
    &QueryRequest::Wasm(WasmQuery::Smart {
      contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
      msg: to_binary(&pool_msg::QueryMsg::Config {})?,
    }))?;

  let token = res.dp_token;

  let info: AssetInfo = AssetInfo::Token { contract_addr: token.clone() };
  let asset: Asset = Asset { info, amount };

  Ok(Response::new()
    .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
      contract_addr: deps
        .api
        .addr_humanize(&state.bank)
        .unwrap()
        .to_string(),
      msg: to_binary(&pool_msg::ExecuteMsg::Earn {})?,
      funds: vec![],
    }))
    .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
      contract_addr: token,
      msg: to_binary(&Cw20ExecuteMsg::IncreaseAllowance {
        spender: deps.api.addr_humanize(&state.bridge).unwrap().to_string(),
        amount,
        expires: None,
      })?,
      funds: vec![],
    }))
    .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
      contract_addr: deps.api.addr_humanize(&state.bridge).unwrap().to_string(),
      msg: to_binary(&bridge_msg::ExecuteMsg::InitiateTransfer {
        asset,
        recipient_chain: 1,
        recipient: state.target,
        fee: Uint128::from(0u32),
        nonce: 3,
      })?,
      funds: vec![],
    }))
  )
}

// pub fn try_approve(deps: DepsMut, _info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
//   let state : Config = CONFIG.load(deps.storage)?;

//   if state.pause {
//     return Err(ContractError::Paused {});
//   }

//   let res : pool_resp::ConfigResponse = deps.querier.query(
//     &QueryRequest::Wasm(WasmQuery::Smart {
//       contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
//       msg: to_binary(&pool_msg::QueryMsg::Config {})?,
//     }))?;

//   let token = res.dp_token;

//   Ok(Response::new()
//     .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
//       contract_addr: token,
//       msg: to_binary(&Cw20ExecuteMsg::IncreaseAllowance {
//         spender: deps.api.addr_humanize(&state.bridge).unwrap().to_string(),
//         amount: amount,
//         expires: None,
//       })?,
//       funds: vec![],
//     }))
//   )
// }

pub fn try_update_config(
  deps: DepsMut,
  info: MessageInfo,
  pause: Option<bool>,
  owner: Option<String>,
  receiver: Option<String>,
  bank: Option<String>,
  bridge: Option<String>,
  target: Option<Binary>,
) -> Result<Response, ContractError> {
    let mut config: Config = CONFIG.load(deps.storage)?;

    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
      return Err(ContractError::Unauthorized {});
    }

    if let Some(pause) = pause {
      config.pause = pause;
    }

    if let Some(owner) = owner {
      let _ = deps.api.addr_validate(&owner)?;

      config.owner = deps.api.addr_canonicalize(&owner)?;
    }

    if let Some(receiver) = receiver {
      let _ = deps.api.addr_validate(&receiver)?;

      config.receiver = deps.api.addr_canonicalize(&receiver)?;
    }

    if let Some(bank) = bank {
      let _ = deps.api.addr_validate(&bank)?;

      config.bank = deps.api.addr_canonicalize(&bank)?;
    }

    if let Some(bridge) = bridge {
      let _ = deps.api.addr_validate(&bridge)?;

      config.bridge = deps.api.addr_canonicalize(&bridge)?;
    }


    if let Some(target) = target {

      config.target = target;
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetBalance {} => to_binary(&query_balance(deps)?),
        QueryMsg::GetTime {} => to_binary(&query_time(deps, env)?)
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state : Config = CONFIG.load(deps.storage)?;

    let res : pool_resp::ConfigResponse = deps.querier.query(
      &QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
        msg: to_binary(&pool_msg::QueryMsg::Config {})?,
      }))?;
  
    let token = res.dp_token;
    Ok(ConfigResponse { 
      pause: state.pause,
      owner: deps.api.addr_humanize(&state.owner)?.to_string(),
      receiver: deps.api.addr_humanize(&state.receiver)?.to_string(),
      bank: deps.api.addr_humanize(&state.bank)?.to_string(),
      bridge: deps.api.addr_humanize(&state.bridge)?.to_string(),
      token: token,
      target: state.target,
    })
}

fn query_balance(deps: Deps) -> StdResult<BalanceResponse> {
  let state : Config = CONFIG.load(deps.storage)?;

  let res : pool_resp::ClaimableRewardResponse = deps.querier.query(
    &QueryRequest::Wasm(WasmQuery::Smart {
      contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
      msg: to_binary(&pool_msg::QueryMsg::ClaimableReward {})?,
    }))?;

  let token = res.amount;


  Ok(BalanceResponse {
    balance: token,
  })
}

fn query_time(deps: Deps, env: Env) -> StdResult<TimeResponse> {
  let time: TimeInfo = TIME.load(deps.storage)?;

  Ok(TimeResponse {
    time: env.block.time.seconds(),
    last_updated_time: time.last_updated_time,
  })
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
//     use cosmwasm_std::{coins, from_binary};

//     #[test]
//     fn proper_initialization() {
//         let mut deps = mock_dependencies(&[]);

//         let msg = InstantiateMsg {
//           receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string()

//          };
//         let info = mock_info("creator", &coins(1000, "earth"));

//         // we can just call .unwrap() to assert this was a success
//         let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
//         assert_eq!(0, res.messages.len());

//         // it worked, let's query the state
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(false, value.pause);
//         assert_eq!("terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string(), value.receiver)

//     }

//     // }

//     #[test]
//     fn pause() {
//         let mut deps = mock_dependencies(&coins(2, "token"));

//         let msg = InstantiateMsg { receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string() };
//         let info = mock_info("creator", &coins(2, "token"));
//         let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

//         // beneficiary can release it
//         let unauth_info = mock_info("anyone", &coins(2, "token"));
//         let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
//         let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
//         match res {
//             Err(ContractError::Unauthorized {}) => {}
//             _ => panic!("Must return unauthorized error"),
//         }

//         // only the original creator can reset the counter
//         let auth_info = mock_info("creator", &coins(2, "token"));
//         let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
//         let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

//         // should now be 5
//         let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
//         let value: ConfigResponse = from_binary(&res).unwrap();
//         assert_eq!(true, value.pause);
//     }
// }
