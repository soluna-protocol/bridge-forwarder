use cosmwasm_std::{Uint128, Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_bignumber::{Uint256};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub receiver: String,
  pub bank: String,
  pub bridge: String,
  pub target: Binary,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Earn {},
    UpdateConfig { 
      pause: Option<bool>, 
      owner: Option<String>, 
      receiver: Option<String>,
      bank: Option<String>,
      bridge: Option<String>,
      target: Option<Binary>,
    },
    Time {},
    // ApproveBridge {
    //   amount: Uint128,
    // },
    Bridge {
      amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetBalance {},
    GetTime {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub pause: bool,
    pub owner: String,
    pub receiver: String,
    pub bank: String,
    pub bridge: String,
    pub token: String,
    pub target: Binary,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Uint256,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimeResponse {
    pub time: u64,
    pub last_updated_time: u64,
}