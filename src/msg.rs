use cosmwasm_schema::cw_serde;

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
   Disburse {}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
 GetOwners {}
}

#[cw_serde]
pub struct GetOwnersResponse {
    pub owners: Vec<Owner>
}