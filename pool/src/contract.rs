use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::msg::ExecuteMsg::UpdateTokenAmount;
use crate::{execute, query};
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
        k: msg.k
    })?;

    Ok(
        Response::new()
            .add_attribute("action", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateTokenAmount {tokenA, tokenB} => execute::update_token_amount(deps, tokenA, tokenB)
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::TokenInfo {} => query::pool_info(deps)
    }
}
