use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::CanonicalAddr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub pause: bool,
    pub owner: CanonicalAddr,
    pub receiver: CanonicalAddr,
    pub bank: CanonicalAddr,
    pub bridge: CanonicalAddr,
}

pub const CONFIG: Item<Config> = Item::new("config");
