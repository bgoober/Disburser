use cosmwasm_schema::{cw_serde, QueryResponses};

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
}

#[cw_serde]
pub struct GetOwnersResponse {
    pub owners: Vec<Owner>,
}
