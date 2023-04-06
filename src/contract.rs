#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::CONFIG;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:disburser";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut total_ownership: u8 = 0;

    // Check that the sum of the individual ownerships is equal to 100
    for owner in &msg.owners {
        total_ownership += owner.ownership;
    }

    if total_ownership != 100 {
        return Err(StdError::generic_err("Total Ownership must equal 100%."));
    }

    for owner in &msg.owners {
        if owner.ownership == 0 {
            return Err(StdError::generic_err(
                "Individual Ownership must be greater than 0",
            ));
        }
    }

    CONFIG.save(deps.storage, &msg.owners)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Disburse {} => disburse(deps, info),
    }
}

pub fn disburse(mut _deps: DepsMut, _info: MessageInfo) -> Result<Response, ContractError> {
    todo!()

    // dusbursing of funds follows this logic:



}
