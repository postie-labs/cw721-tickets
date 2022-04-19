use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Empty;
use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub location: String,
    pub date: String,
    pub note: String,
    pub attributes: Option<Attribute>
}

pub type Extension = Option<Metadata>;
