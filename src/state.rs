use cosmwasm_std::Addr;
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, JsonSchema, Debug, PartialEq)]
pub struct Owner {
    pub address: Addr,
    pub ownership: u8,
}

pub const OWNERS: Item<Vec<Owner>> = Item::new("owners");
