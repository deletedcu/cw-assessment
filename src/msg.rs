use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub owner: String,
  pub users: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  AddUser {user: String},
  RemoveUser {user: String},
  UpdateUsers {
    add: Vec<String>,
    remove: Vec<String>
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetUsers {},
  GetUser {user: String},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct UsersResponse {
  pub users: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ExistResponse {
  pub exist: bool,
}
