use cosmwasm_std::{Addr, Uint128};
use crate::state::TokenInfo;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct InstantiateMsg {
    pub id: String,
    pub symbol: String,
    pub tokenA: TokenInfo,
    pub tokenB: TokenInfo,
    pub k: Uint128,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum ExecuteMsg {
    UpdateTokenAmount {
        tokenA: Uint128,
        tokenB: Uint128,
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    PoolInfo {}
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct PoolInfoResponse {
    pub id: String,
    pub symbol: String,
    pub tokenA: TokenInfo,
    pub tokenB: TokenInfo,
    pub k: Uint128,
}




