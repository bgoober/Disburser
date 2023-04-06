use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, StdError, StdResult, BankMsg};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg};
use crate::state::{CONFIG};

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Disburse {} => disburse(deps, info, env),
    }
}

pub fn disburse(deps: DepsMut, info: MessageInfo, env: Env) -> Result<Response, ContractError> {

    // dusbursing of funds follows this logic:
    // load config from storage
    // if the info.sender is not in the list of Owners, then send an unauthorized error

    let config = CONFIG.load(deps.storage)?;

    let mut owners = vec![];

    for owner in config {
        let owner = owner.address; {
            owners.push(owner);
        }
    };

    if !owners.contains(&info.sender) {
        Err(ContractError::Unauthorized {})
    } else {
        let bank_msg: BankMsg = send_tokens(deps, env)?;
        Ok(Response::new()
.add_message(bank_msg))
    }

}


pub fn send_tokens(_deps: DepsMut, _env: Env) -> Result<BankMsg, ContractError> {
    
    // let config = CONFIG.load(deps.storage)?;

//             SubMsg::new(CosmosMsg::Bank(BankMsg::Send {


//    // define a message payload as a new ExecuteMsg
//    // the tokenfactory_types::msg::ExecuteMsg::Mint path comes from the tokenfactory module
//    // another way of doing this would be to import above "use tokenfactory_types::msg::ExecuteMsg::Mint" and just use Mint here
//    let payload =  {
//        // same as using Mint without importing the entire module's path to the Mint function
//        address: info.sender.to_string(), // address to mint the tokens to is the info.sender of this execute message
//        denom: std::vec![NATIVE_TOKEN.load(deps.storage)?], // token denom is the NATIVE_TOKEN Item from storage, in a vector
//    };

//    // define the mint_msg as a Wasm Execute message
//    let mint_msg = WasmMsg::Execute {
//        contract_addr: contract_addr.to_string(), // contract address to send it to is the contract_addr defined above
//        msg: to_binary(&payload)?, // message to be sent, in binary format, is the payload object define above
//        funds: std::vec![], // an empty funds vector means no funds are being sent to the admin contract with the message
//    };

//    // add the mint_msg, which is the WasmMsg, to the Ok result/response
//    Ok(mint_msg)

todo!("Query the contract's wallet for the tokens it has. Create a BankMsg::Send for each owner in the config, and send each token the contract owns by that addresses' ownership percentage.")



}