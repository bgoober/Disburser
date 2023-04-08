use cosmwasm_std::{entry_point, to_binary, Binary, Deps, Env, StdResult};

use crate::msg::QueryMsg;

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query::get_config(deps)?),
    }
}

pub mod query {
    use cosmwasm_std::{Deps, StdResult};

    use crate::{msg::GetConfigResponse, state::CONFIG};

    pub fn get_config(deps: Deps) -> StdResult<GetConfigResponse> {
        let config = CONFIG.load(deps.storage)?;
        Ok(GetConfigResponse { owners: config })
    }
}
