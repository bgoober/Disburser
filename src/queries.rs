use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwners {} => to_binary(&query::get_owners(deps)?),
    }
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::GetOwnersResponse, state::OWNERS};

    pub fn get_owners(deps: Deps) -> StdResult<GetOwnersResponse> {
        let owners = OWNERS.load(deps.storage)?;
        Ok(GetOwnersResponse { owners })
    }
}
