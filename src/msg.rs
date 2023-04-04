use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owners: Vec<Owner>, // A vector of Owners called owners
    pub denominations: Vec<Denomination>, //
    pub quorum: Option<u8>, // Optional quorum required for executive functions
    pub tn_threshold: Optional<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddAddress { address: String, weight: u128 },
    RemoveAddress { address: String },
    AdjustWeight { address: String, weight: u128 },
    AddDenomination { denomination: String, threshold: u128 },
    ChangeThreshold { denomination: String, threshold: u128 },
    ChangeQuorum { quorum: u8 },
    ChangeTNThreshold { tn_threshold: u8 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetDisbursement {},
    GetAddresses {},
    GetThreshold {},
    GetQuorum {},
    GetTNThreshold {},
    GetAdmin {},
}
