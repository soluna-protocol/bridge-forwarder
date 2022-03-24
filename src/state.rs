use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Binary};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub pause: bool,
    pub owner: CanonicalAddr,
    pub bank: CanonicalAddr,
    pub bridge: CanonicalAddr,
    pub target: Binary,
    pub period: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TimeInfo {
  pub last_updated_time: u64,
}

pub const TIME: Item<TimeInfo> = Item::new("time");
