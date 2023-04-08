use cosmwasm_schema::{cw_serde, QueryResponses};

use cosmwasm_std::{BalanceResponse, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::Owner;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct InstantiateMsg {
    pub owners: Vec<Owner>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Disburse {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema, QueryResponses)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    #[returns(GetOwnersResponse)]
    GetOwners {},

    #[returns(BalanceResponse)]
    Balance {address: String}
}

#[cw_serde]
pub struct GetOwnersResponse {
    pub owners: Vec<Owner>,
}

#[cw_serde]
pub struct GetBalanceResponse {
    pub balances: Vec<Coin>,
}