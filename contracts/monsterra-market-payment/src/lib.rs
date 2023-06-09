mod error;
mod execute;
pub mod msg;
pub mod state;
mod query;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
pub use crate::state::GamePaymentContract;
use cosmwasm_std::Empty;

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let tract = GamePaymentContract::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<>,
    ) -> Result<Response, ContractError> {
        let tract = GamePaymentContract::default();
        tract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let tract = GamePaymentContract::default();
        tract.query(deps, env, msg)
    }
}
