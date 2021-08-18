use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, JsonSchema)]
pub struct State {
  pub users: Vec<Addr>,
  pub owner: String,
}

pub fn config(storage: &mut dyn Storage) -> Singleton<State> {
  singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
  singleton_read(storage, CONFIG_KEY)
}
