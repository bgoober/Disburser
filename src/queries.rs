use std::env;

use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwners {} => to_binary(&query::get_owners(deps)?),
        QueryMsg::Balance { address } => to_binary(&query::get_balance(deps, env)?),
    }
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult, Env, BalanceResponse};

    use crate::{msg::{GetOwnersResponse, QueryMsg}, state::CONFIG};

    pub fn get_owners(deps: Deps) -> StdResult<GetOwnersResponse> {
        let config = CONFIG.load(deps.storage)?;
        Ok(GetOwnersResponse { owners: config })
    }

    pub fn get_balance(deps: Deps, env: Env) -> StdResult<()> {
        // let contract_addr = env.contract.address;
        let query_msg = QueryMsg::Balance { address: env.contract.address.to_string() };
        let res: BalanceResponse = deps.querier.query(&query_msg)?;
        Ok(res)
    }
}
