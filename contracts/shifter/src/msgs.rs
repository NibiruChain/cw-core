use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Decimal, Uint256};

use crate::state::Whitelist;

/// InitMsg specifies the args for the instantiate entry point of the contract.
#[cw_serde]
pub struct InitMsg {
    pub admin: String,
}

/// ExecuteMsg specifies the args for the execute entry point of the contract.
#[cw_serde]
pub enum ExecuteMsg {
    DepthShift { pair: String, depth_mult: Uint256 },
    PegShift { pair: String, peg_mult: Decimal },
    AddMember { address: String },
    RemoveMember { address: String },
    ChangeAdmin { address: String },
    NoOp {},
}

/// QueryMsg specifies the args for the query entry point of the contract.
#[cw_serde]
pub enum QueryMsg {
    IsMember { address: String },
    Whitelist {},
}

#[cw_serde]
pub struct IsMemberResponse {
    pub is_member: bool,
    pub whitelist: Whitelist,
}

#[cw_serde]
pub struct WhitelistResponse {
    pub whitelist: Whitelist,
}
