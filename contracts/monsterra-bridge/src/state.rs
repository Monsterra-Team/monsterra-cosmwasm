use crate::error::ContractError;
use cosmwasm_schema::cw_serde;

use cosmwasm_std::{to_binary, Addr, Binary, MessageInfo, Response, StdResult, Storage, Uint128};
use cw_storage_plus::{Item, Map};

use crate::msg::SwapMessage;

#[cw_serde]
pub struct Swapdata {
    pub cur_token: String,
    pub des_token: String,
    pub cur_user: String,
    pub des_user: String,
    pub amount: Uint128,
    pub side: String,
}

impl Swapdata {
    pub fn default(swap_message: &SwapMessage, side: String) -> Swapdata {
        Swapdata {
            cur_token: swap_message.cur_token.to_string(),
            des_token: swap_message.des_token.to_string(),
            cur_user: swap_message.cur_user.to_string(),
            des_user: swap_message.des_user.to_string(),
            amount: swap_message.amount,
            side: side,
        }
    }

    pub fn empty_default() -> Swapdata {
        Swapdata {
            cur_token: String::new(),
            des_token: String::new(),
            cur_user: String::new(),
            des_user: String::new(),
            amount: Uint128::new(0),
            side: String::from("NaN"),
        }
    }
}

//getter setter of administration data
pub fn set_new_owner(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
) -> Result<Response, ContractError> {
    if info.sender != get_owner(storage) {
        return Err(ContractError::Unauthorized {});
    }

    let result = OWNER.save(storage, &user);
    match result {
        Ok(_) => Ok(Response::new().add_attribute("method", "transfer_ownership")),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_owner(storage: &dyn Storage) -> Addr {
    OWNER.load(storage).unwrap()
}

pub fn set_admin(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    user: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    if info.sender != get_owner(storage) {
        return Err(ContractError::Unauthorized {});
    }

    let result = ADMIN.save(storage, user.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_admin")
            .add_attribute("user", user)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_admin(storage: &dyn Storage, user: Addr) -> bool {
    let result = ADMIN.load(storage, user);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_signer(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    public_key: Binary,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = SIGNER.save(storage, &public_key);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_signer")
            .add_attribute("public_key", public_key.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_signer(storage: &dyn Storage) -> Binary {
    let result = SIGNER.load(storage);
    match result {
        Ok(value) => value,
        Err(_) => {
            let mes = "";
            to_binary(&mes).unwrap()
        }
    }
}

pub fn set_accepted_token(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    token: String,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = ACCEPTED_TOKENS.save(storage, token.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_accepted_token")
            .add_attribute("accpeted_token", token)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_accepted_token(storage: &dyn Storage, token: &String) -> bool {
    let result = ACCEPTED_TOKENS.load(storage, token.to_string());
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_accepted_des_token(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    token: String,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = DES_ACCEPTED_TOKENS.save(storage, token.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_des_accepted_token")
            .add_attribute("des_accpeted_token", token)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_accepted_des_token(storage: &dyn Storage, token: &String) -> bool {
    let result = DES_ACCEPTED_TOKENS.load(storage, token.to_string());
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

//getter setter of swapdata
pub fn set_swap_data(
    storage: &mut dyn Storage,
    transaction_id: String,
    swapdata: Swapdata,
) -> StdResult<()> {
    SWAPDATAS.save(storage, transaction_id, &swapdata)
}

pub fn get_swap_data(storage: &dyn Storage, transaction_id: String) -> Swapdata {
    let result = SWAPDATAS.load(storage, transaction_id);

    match result {
        Ok(value) => value,
        Err(_) => Swapdata::empty_default(),
    }
}

pub fn set_max_swap_amount(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    token: String,
    max_amount: &Uint128,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = MAX_SWAP_AMOUNTS.save(storage, token.clone(), &max_amount);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_max_swap_amount")
            .add_attribute("token", token)
            .add_attribute("max_amount", max_amount.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn get_max_swap_amount(storage: &dyn Storage, token: &String) -> Uint128 {
    let result: Result<Uint128, cosmwasm_std::StdError> =
        MAX_SWAP_AMOUNTS.load(storage, token.to_string());
    match result {
        Ok(value) => value,
        Err(_) => Uint128::new(0),
    }
}

pub fn set_approve_transaction(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    transaction_id: String,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_operator(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = APPROVED_TRANSACTIONS.save(storage, transaction_id.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_approve_transaction")
            .add_attribute("transaction_id", transaction_id)
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_approve_transaction(storage: &dyn Storage, transaction_id: &String) -> bool {
    let result: Result<bool, cosmwasm_std::StdError> =
        APPROVED_TRANSACTIONS.load(storage, transaction_id.to_string());
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub fn set_operator(
    storage: &mut dyn Storage,
    info: &MessageInfo,
    operator: Addr,
    status: bool,
) -> Result<Response, ContractError> {
    if !is_admin(storage, info.sender.clone()) {
        return Err(ContractError::Unauthorized {});
    }

    let result = OPERATORS.save(storage, operator.clone(), &status);
    match result {
        Ok(_) => Ok(Response::new()
            .add_attribute("method", "set_operator")
            .add_attribute("operator", operator.to_string())
            .add_attribute("status", status.to_string())),
        Err(_) => Err(ContractError::Internal {}),
    }
}

pub fn is_operator(storage: &dyn Storage, operator: Addr) -> bool {
    let result: Result<bool, cosmwasm_std::StdError> = OPERATORS.load(storage, operator);
    match result {
        Ok(value) => value,
        Err(_) => false,
    }
}

pub const OWNER: Item<Addr> = Item::new("owner");
pub const ADMIN: Map<Addr, bool> = Map::new("admin");
pub const SIGNER: Item<Binary> = Item::new("signer");

pub const SWAPDATAS: Map<String, Swapdata> = Map::new("swapdata");
pub const ACCEPTED_TOKENS: Map<String, bool> = Map::new("acceptedToken");
pub const DES_ACCEPTED_TOKENS: Map<String, bool> = Map::new("desAcceptedToken");
pub const MAX_SWAP_AMOUNTS: Map<String, Uint128> = Map::new("maxSwapAmounts");
pub const APPROVED_TRANSACTIONS: Map<String, bool> = Map::new("approvedTransactions");
pub const OPERATORS: Map<Addr, bool> = Map::new("operators");
