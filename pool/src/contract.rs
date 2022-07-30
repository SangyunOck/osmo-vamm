use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{InstantiateMsg, QueryMsg};
use crate::query;
use crate::state::{POOL, PoolInfo};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    POOL.save(deps.storage, &PoolInfo {
        id: msg.id,
        symbol: msg.symbol,
        tokenA: msg.tokenA,
        tokenB: msg.tokenB,
        main_contract: info.sender,
    })?;

    Ok(
        Response::new()
            .add_attribute("action", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenInfo {} => query::balance(deps)
    }
}
