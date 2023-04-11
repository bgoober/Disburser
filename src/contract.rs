use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{Owner, OWNERS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:disburser";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Define the `instantiate` function that takes in several arguments and returns a `StdResult<Response>`.
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    // Set the contract version.
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let mut total_ownership: u8 = 0;

    for owner in &msg.owners {
        total_ownership += owner.ownership;

        // check total onwership is 100%
        if total_ownership != 100 {
            return Err(ContractError::InvalidTotalOwnership {});
        }
        // check for 0 ownership values
        if owner.ownership == 0 {
            return Err(ContractError::InvalidIndividualOwnership {});
        }
    }

    // Save the owners and their ownership percentages to storage.
    OWNERS.save(deps.storage, &msg.owners)?;

    // Return a successful `Response`.
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
// Define the `execute` entry point function, which takes in several arguments and returns a `Result<Response, ContractError>`.
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // Match on the message type received and execute the appropriate code based on the type.
    match msg {
        ExecuteMsg::Disburse {} => disburse(deps, info, env),
    }
}

// Define the `disburse` function, which takes in several arguments and returns a `Result<Response, ContractError>`.
pub fn disburse(deps: DepsMut, info: MessageInfo, _envv: Env) -> Result<Response, ContractError> {
    // Load the `owners` data from storage.
    let owners = OWNERS.load(deps.storage)?;

    // Check if the sender is authorized to disburse funds by iterating over the `owners` and looking for a matching address.
    let authorized = owners.iter().any(|owner| owner.address == info.sender);
    if !authorized {
        return Err(ContractError::Unauthorized {});
    }

    // Build messages to disburse funds to each owner based on their ownership percentage.
    let messages = build_messages(&info.funds, &owners);

    // Return a successful `Response` with the built messages to disburse the funds.
    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("disbursed_by", info.sender.to_string()))
}

// Define a function called `build_messages` that takes in two arguments of type `&[Coin]` and `&[Owner]`
// and returns a `Vec<CosmosMsg>`.
pub fn build_messages(funds: &[Coin], owners: &[Owner]) -> Vec<CosmosMsg> {
    // Loop over each `owner` in the `owners` vector and apply a closure to it.
    owners
        .iter()
        .map(|Owner { address, ownership }| {
            // For each `owner`, calculate the amount of funds they should receive based on their ownership percentage.
            let amount = funds
                .iter()
                .map(|Coin { denom, amount }| Coin {
                    denom: denom.clone(),
                    amount: amount.multiply_ratio(*ownership, 100u128),
                })
                .collect();

            // Build a `CosmosMsg` that sends the calculated amount of funds to the `owner`'s address.
            CosmosMsg::Bank(BankMsg::Send {
                to_address: address.to_string(),
                amount,
            })
        })
        .collect() // Collect all the built `CosmosMsg` instances into a vector.
}
