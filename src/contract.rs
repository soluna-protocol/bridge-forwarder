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
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, TimeResponse};
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
      bank: deps.api.addr_canonicalize(msg.bank.as_str())?,
      bridge:  deps.api.addr_canonicalize(msg.bridge.as_str())?,
      target: msg.target,
      period: msg.period,
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
        ExecuteMsg::Rebase {} => try_rebase(deps, env, info),
        ExecuteMsg::UpdateConfig { pause, owner, bank, bridge, target, period }
          => try_update_config(deps, info, pause, owner, bank, bridge, target, period),
    }
}

pub fn try_rebase(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
) -> Result<Response, ContractError> {
  let state : Config = CONFIG.load(deps.storage)?;
  let mut time: TimeInfo = TIME.load(deps.storage)?;

  if state.pause {
    return Err(ContractError::Paused {});
  }

  if env.block.time.seconds() < time.last_updated_time + state.period {
    return Err(ContractError::Time {});
  }

  time.last_updated_time = env.block.time.seconds();

  TIME.save(deps.storage, &time)?;

  let nonce = env.block.time.seconds() as u32;

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
        nonce,
      })?,
      funds: vec![],
    }))
  )
}

pub fn try_update_config(
  deps: DepsMut,
  info: MessageInfo,
  pause: Option<bool>,
  owner: Option<String>,
  bank: Option<String>,
  bridge: Option<String>,
  target: Option<Binary>,
  period: Option<u64>
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

    if let Some(bank) = bank {
      let _ = deps.api.addr_validate(&bank)?;

      config.bank = deps.api.addr_canonicalize(&bank)?;
    }

    if let Some(bridge) = bridge {
      let _ = deps.api.addr_validate(&bridge)?;

      config.bridge = deps.api.addr_canonicalize(&bridge)?;
    }

    if let Some(period) = period {
      config.period = period;
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
      bank: deps.api.addr_humanize(&state.bank)?.to_string(),
      bridge: deps.api.addr_humanize(&state.bridge)?.to_string(),
      token,
      target: state.target,
      period: state.period,
    })
}

fn query_time(deps: Deps, env: Env) -> StdResult<TimeResponse> {
  let time: TimeInfo = TIME.load(deps.storage)?;

  Ok(TimeResponse {
    time: env.block.time.seconds(),
    last_updated_time: time.last_updated_time,
  })
}
