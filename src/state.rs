use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug, PartialEq)]
pub struct Owner {
    pub address: Addr,
    pub ownership: u8,
}

// #[derive(Serialize, Deserialize, Clone, JsonSchema)]
// pub struct Config {
//     beneficiaries: Vec<Beneficiary>
// }

pub const CONFIG: Item<Vec<Owner>> = Item::new("config");

// pub const CONFIG: Item<Config> = Item::new("config");

