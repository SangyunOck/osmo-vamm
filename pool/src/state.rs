use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PoolInfo {
    pub id: String,
    pub symbol: String,
    pub tokenA: TokenInfo,
    pub tokenB: TokenInfo,
    pub main_contract: Addr,
    pub k: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub addr: Addr,
    pub amount: Uint128,
}


pub const POOL: Item<PoolInfo> = Item::new("pool");
