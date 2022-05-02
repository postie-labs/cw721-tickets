mod custom;

pub use crate::custom::{Extension, MigrateMsg};

use cosmwasm_std::Empty;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

pub type Cw721TicketsContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[entry_point]
    pub fn migrate(
        _deps: DepsMut,
        _env: Env,
        _msg: MigrateMsg,
    ) -> StdResult<Response> {
        Ok(Response::default())
    }

    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        Cw721TicketsContract::default().instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721TicketsContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(
        deps: Deps,
        env: Env,
        msg: QueryMsg,
    ) -> StdResult<Binary> {
        Cw721TicketsContract::default().query(deps, env, msg)
    }
}
