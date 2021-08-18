use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
  AddUser {user: Addr},
  RemoveUser {user: Addr},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  GetUsers {},
  GetUser {user: Addr},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct UsersResponse {
  pub users: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ExistResponse {
  pub exist: bool,
}