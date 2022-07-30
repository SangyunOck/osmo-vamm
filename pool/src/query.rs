use cosmwasm_std::{Deps, StdResult, Binary, to_binary, Uint128};
use crate::state::POOL;
use crate::msg::{PoolInfoResponse};

pub fn pool_info(
    deps: Deps,
) -> StdResult<Binary> {
    let pool = POOL.load(deps.storage)?;
    to_binary(
        &PoolInfoResponse {
            id: pool.id.to_string(),
            symbol: pool.symbol.to_string(),
            tokenA: pool.tokenA,
            tokenB: pool.tokenB,
            k: pool.k,
        }
    )
}
