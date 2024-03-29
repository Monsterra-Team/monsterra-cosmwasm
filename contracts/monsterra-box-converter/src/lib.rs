pub mod contract;
mod error;
pub mod msg;
pub mod state;
pub mod execute;

pub use crate::error::ContractError;
use cosmwasm_std::Empty;
pub type Extension = Option<Empty>;