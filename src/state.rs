use schemars::JsonSchema;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use cosmwasm_std::{Addr, BlockInfo, StdResult, Storage};

use cw721::{ContractInfoResponse, CustomMsg, Cw721, Expiration};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub struct Cw721Contract<'a> {
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub master: Item<'a, Addr>,
    pub token_count: Item<'a, u64>,
    pub operator: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub tokens: IndexedMap<'a, &'a str, TokenInfo, TokenIndexes<'a>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub owner: Addr,
    pub approvals: Vec<Approval>,
    pub ticket: Ticket,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Ticket {
    pub issuer: Addr,
    pub timestamp: u32,
    pub ticket_type: u8,
    pub data: Vec<u8>,
    pub metadata: Vec<u8>,
}

pub struct TokenIndexes<'a> {
    pub owner: MultiIndex<'a, Addr, TokenInfo, Addr>,
}

impl<'a> IndexList<TokenInfo> for TokenIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo>> + '_> {
        let v: Vec<&dyn Index<TokenInfo>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx(d: &TokenInfo) -> Addr {
    d.owner.clone()
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Approval {
    pub spender: Addr,
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}
