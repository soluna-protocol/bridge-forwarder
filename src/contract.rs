#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
  to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, CosmosMsg, 
  WasmMsg, WasmQuery, QueryRequest, Uint128,
};
use cosmwasm_bignumber::Uint256;
use cw20::Cw20ExecuteMsg;
use terraswap::asset::{
  Asset,
  AssetInfo,
};

use crate::error::ContractError;
use crate::msg::{BalanceResponse, ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};
use crate::tax_utils::deduct_tax;
use crate::token_utils::balance_of;
use crate::pool_msg;
use crate::pool_resp;
use crate::bridge_msg;


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
      pause: false,
      owner: deps.api.addr_canonicalize(info.sender.as_str())?,
      receiver: deps.api.addr_canonicalize(msg.receiver.as_str())?,
      bank: deps.api.addr_canonicalize(msg.bank.as_str())?,
      bridge:  deps.api.addr_canonicalize(msg.bridge.as_str())?,
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Earn {} => try_earn(deps),
        ExecuteMsg::UpdateConfig { pause, owner, receiver, bank, bridge }
          => try_update_config(deps, info, pause, owner, receiver, bank, bridge),
        ExecuteMsg::Bridge { amount, recipient_chain, recipient, nonce }
          => try_bridge(deps, info, amount, recipient_chain, recipient, nonce),
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

pub fn try_bridge(
  deps: DepsMut,
  _info: MessageInfo,
  amount: Uint128,
  recipient_chain: u16,
  recipient: String,
  nonce: u32,
) -> Result<Response, ContractError> {
  let state : Config = CONFIG.load(deps.storage)?;

  if state.pause {
    return Err(ContractError::Paused {});
  }

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
      contract_addr: token,
      msg: to_binary(&Cw20ExecuteMsg::IncreaseAllowance {
        spender: deps.api.addr_humanize(&state.bridge).unwrap().to_string(),
        amount: amount,
        expires: None,
      })?,
      funds: vec![],
    }))
    .add_message(CosmosMsg::Wasm(WasmMsg::Execute {
      contract_addr: deps.api.addr_humanize(&state.bridge).unwrap().to_string(),
      msg: to_binary(&bridge_msg::ExecuteMsg::InitiateTransfer {
        asset,
        recipient_chain,
        recipient: to_binary(&recipient)?,
        fee: Uint128::from(10u64),
        nonce,
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

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetBalance {} => to_binary(&query_balance(deps, env)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state : Config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse { 
      pause: state.pause,
      owner: deps.api.addr_humanize(&state.owner)?.to_string(),
      receiver: deps.api.addr_humanize(&state.receiver)?.to_string(),
      bank: deps.api.addr_humanize(&state.bank)?.to_string(),
      bridge: deps.api.addr_humanize(&state.bridge)?.to_string(),
    })
}

fn query_balance(deps: Deps, env: Env) -> StdResult<BalanceResponse> {
  let state : Config = CONFIG.load(deps.storage)?;

  let res : pool_resp::ConfigResponse = deps.querier.query(
    &QueryRequest::Wasm(WasmQuery::Smart {
      contract_addr: deps.api.addr_humanize(&state.bank).unwrap().to_string(),
      msg: to_binary(&pool_msg::QueryMsg::Config {})?,
    }))?;

  let token = res.dp_token;

  let token_balance = balance_of(
    deps,
    token.clone(),
    env.contract.address.to_string(),
  ).unwrap();

  let real_value = Uint256::from(
    deduct_tax(
      deps,
      Coin {
        denom: token,
        amount: token_balance.into(),
      }
    )?
    .amount,
  );

  Ok(BalanceResponse {
    balance: real_value,
  })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let msg = InstantiateMsg {
          receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string()

         };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(false, value.pause);
        assert_eq!("terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string(), value.receiver)

    }

    // }

    #[test]
    fn pause() {
        let mut deps = mock_dependencies(&coins(2, "token"));

        let msg = InstantiateMsg { receiver: "terra1sh36qn08g4cqg685cfzmyxqv2952q6r8gpczrt".to_string() };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::UpdateConfig { pause: Some(true), owner: None, receiver: None };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(true, value.pause);
    }
}
