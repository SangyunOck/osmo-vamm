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

pub fn instantiate_msg(id: String, symbol: String, tokenA: Addr, tokenB: Addr, k: Uint128) -> InstantiateMsg {
    InstantiateMsg {
        id,
        symbol,
        tokenA: TokenInfo {
            addr: tokenA,
            amount: Uint128::zero(),
        },
        tokenB: TokenInfo {
            addr: tokenB,
            amount: Uint128::zero(),
        },
        k,
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub enum QueryMsg {
    TokensInfo {}
}

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct TokenInfoResponse {
    pub tokenA: TokenInfo,
    pub tokenB: TokenInfo,
}


