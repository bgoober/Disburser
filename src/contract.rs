use std::env;

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, BalanceResponse, BankMsg, Coin, Decimal, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, QueryResponse, Uint128,
};

use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
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
                "Individual Ownership must be greater than 0.",
            ));
        }
    }

    CONFIG.save(deps.storage, &msg.owners)?;

    Ok(Response::new().add_attribute("action", "instantiate"))
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
        let owner = owner.address;
        {
            owners.push(owner);
        }
    }

    if !owners.contains(&info.sender) {
        Err(ContractError::Unauthorized {})
    } else {
        let bank_msg: BankMsg = build_messages(deps, env)?;
        Ok(Response::new().add_message(bank_msg))
    }
}

pub fn build_messages_(deps: DepsMut, env: Env) -> Result<BankMsg, ContractError> {
    // the build_messages function will be used to build the individual BankMsg sends to each beneficiary.
    // the logic for this function will be as follows:
    // load the config from storage
    // query the contract's wallet for the the tokens it currently has, and the amount of those tokens
    // for each owner in the config, calculate the amount of tokens they are entitled to, for each token held by the address, based on their ownership percentage
    // create a BankMsg::Send for each owner in the config, and send each token the contract owns by that addresses' ownership percentage

    // let contract_addr = deps.api.addr_humanize(&env.contract.address)?;

    // // load the config from storage
    // let config = CONFIG.load(deps.storage)?;

    // // query the contract's wallet for the the tokens it currently has, and the amount of those tokens
    // let query_msg = QueryMsg::Balance { address: contract_addr.to_string() };

    // let res: BalanceResponse = deps.querier.query(&query_msg)?;

    // let mut bank_msg = BankMsg::new();

    // for owner in config {
    //     let owner_ = owner.address;
    //     let ownership = owner.ownership;

    //     for balance in res.balance {
    //         let denom = balance.denom;
    //         let amount = balance.amount;

    //         let amount = amount * ownership / 100;

    //         let send_msg = BankMsg::Send {
    //             to_address: owner_.to_string(),
    //             amount: vec![Coin {
    //                 denom: denom,
    //                 amount: amount,
    //             }],
    //         };

    //         bank_msg = bank_msg.add(send_msg);
    //     }
    // }

    // Ok(bank_msg)
    todo!("build_messages")
}


pub fn build_messages(deps: DepsMut, env: Env) -> StdResult<BankMsg> {
    let contract_addr = Addr::unchecked(env.contract.address);

    // load the config from storage
    let config = CONFIG.load(deps.storage)?;

    // query the contract's wallet for the tokens it currently has
  let query_msg = QueryMsg::Balance {
        address: contract_addr.to_string(),
    };
    let res: QueryResponse = deps.querier.query(&query_msg)?;

    let mut bank_msg = BankMsg::Send();

    for owner in &config {
        let owner_ = Addr::unchecked(owner.address.clone());
        let ownership = owner.ownership as u8;

        for coin in &res.balance {
            let denom = coin.denom.clone();
            let amount = coin.amount;

            let amount = (Uint128::from(amount) * Uint128::from(ownership) / Uint128::from(100));

            let send_msg = BankMsg::Send {
                to_address: owner_.to_string(),
                amount: vec![Coin {
                    denom: denom,
                    amount: amount.into(),
                }],
            };

            bank_msg = bank_msg.add(send_msg);
        }
    }

    Ok(bank_msg)
}

