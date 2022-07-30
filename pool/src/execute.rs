use cosmwasm_std::{DepsMut, Response, StdResult, Uint128};
use crate::error::ContractError;
use crate::state::POOL;

pub fn update_token_amount(
    deps: DepsMut,
    tokenA: Uint128,
    tokenB: Uint128,
) -> Result<Response, ContractError> {
    POOL.update(deps.storage, |mut p| -> StdResult<_>{
        p.tokenA = tokenA;
        p.tokenB = tokenB;
        Ok(p)
    })?;

    Ok(
        Response::new()
            .add_attribute("action", "update_token_amount")
            .add_attribute("tokenA", tokenA.to_string())
            .add_attribute("tokenB", tokenB.to_string())
    )
}
