use cosmwasm_std::{Binary};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub bank: String,
  pub bridge: String,
  pub target: Binary,
  pub period: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Rebase {},
    UpdateConfig { 
      pause: Option<bool>, 
      owner: Option<String>, 
      bank: Option<String>,
      bridge: Option<String>,
      target: Option<Binary>,
      period: Option<u64>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetTime {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub pause: bool,
    pub owner: String,
    pub bank: String,
    pub bridge: String,
    pub token: String,
    pub target: Binary,
    pub period: u64,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimeResponse {
    pub time: u64,
    pub last_updated_time: u64,
}
