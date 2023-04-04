use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, HandleResult, HumanAddr, InitResult,
    Querier, StdError, StdResult, Storage, Uint128, WasmQuery,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Owner { // An entity with rights to disbursement of the feeshare
    pub address: String, // A member's address
    pub weight: Uint128, // A members weight or ownership
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Denomination { // An expected or accepted denomination of fees
    pub denomination: String, // The denomination
    pub threshold: Uint128, // The denominations balance threshold to trigger disbursement
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct Config {
    pub owners: Vec<Owner>, // A vector of Owners called owners
    pub denominations: Vec<Denomination>, //
    pub quorum: Option<u8>, // Optional quorum required for executive functions
    pub tn_threshold: Optional<u8>, // Optional t/n threshold required for executive functions
    }
