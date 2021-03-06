

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{ Uint128};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub total_supply:Uint128,
    pub presale_start:u64,
    pub presale_end:u64,
    pub vesting_period:u64,
    pub vesting_step_period:u64,
    pub token_price:Uint128,
    pub denom:String,
    pub admin_wallet:String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
 ChangeOwner{address:String},
 WithdrawToken{},
 WithdrawAdminToken{},
 BuyToken{amount:Uint128},
 SetTokenAddress{address:String},
 SendTokenContract{},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns a human-readable representation of the arbiter.
    GetStateInfo {},
    GetUserInfo{address:String},
    GetAllUsers{},
    CheckWithdraw{address:String}
}

