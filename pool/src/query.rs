use cosmwasm_std::{Deps, StdResult, Binary, to_binary, Uint128};
use crate::state::POOL;
use crate::msg::{BalanceResponse, TokenInfoResponse};

pub fn balance(
    deps: Deps,
) -> StdResult<Binary> {
    let pool = POOL.load(deps.storage)?;
    to_binary(
        &TokenInfoResponse {
            tokenA: pool.tokenA,
            tokenB: pool.tokenB,
        }
    )
}
