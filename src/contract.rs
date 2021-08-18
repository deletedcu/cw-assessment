use cosmwasm_std::{to_binary, Addr, Binary, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::state::{config, config_read, State};
use crate::error::ContractError;
use crate::msg::{HandleMsg, QueryMsg, UsersResponse, ExistResponse};

const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

pub fn init(
  deps: DepsMut, 
  env: Env
) -> Result<Response, ContractError> {
  let state = State {
    users: vec![],
    owner: env.contract.address.clone(),
  };

  config(deps.storage).save(&state)?;
  Ok(Response::default())
}

pub fn handle(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: HandleMsg,
) -> Result<Response, ContractError> {
  match msg {
    HandleMsg::AddUser {user} => add_user(deps, env, info, user),
    HandleMsg::RemoveUser {user} => remove_user(deps, env, info, user),
  }
}

fn add_user(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  recipient: Addr,
) -> Result<Response, ContractError> {
  let mut state = config(deps.storage).load()?;

  if env.contract.address != state.owner {
    return Err(ContractError::Unauthorized {});
  }

  validate_name(recipient.as_str())?;

  state.users.push(recipient);
  config(deps.storage).save(&state)?;

  Ok(Response::default())
}

fn remove_user(
  deps: DepsMut,
  env: Env,
  _info: MessageInfo,
  recipient: Addr,
) -> Result<Response, ContractError> {
  let mut state = config(deps.storage).load()?;

  if env.contract.address != state.owner {
    return Err(ContractError::Unauthorized {});
  }

  // TODO: check vec delete function
  let index = state.users.iter().position(|x| *x == recipient).unwrap();
  if index > 0 {
    state.users.remove(index);
    config(deps.storage).save(&state)?;
  }

  Ok(Response::default())
}

pub fn query(
  deps: DepsMut,
  _env: Env,
  msg: QueryMsg,
) -> StdResult<Binary> {
  match msg {
    QueryMsg::GetUsers {} => get_users(deps),
    QueryMsg::GetUser {user} => exist_user(deps, user),
  }
}

fn get_users(
  deps: DepsMut,
) -> StdResult<Binary> {
  let state = config_read(deps.storage).load()?;
  let resp = UsersResponse {users: state.users};
  
  to_binary(&resp)
}

fn exist_user(
  deps: DepsMut,
  user: Addr,
) -> StdResult<Binary> {
  let state = config_read(deps.storage).load()?;
  let exist = state.users.contains(&user);
  let resp = ExistResponse {exist};
  
  to_binary(&resp)
}

// let's not import a regexp library and just do these checks by hand
fn invalid_char(c: char) -> bool {
  let is_valid =
    (c >= '0' && c <= '9') || (c >= 'a' && c <= 'z') || (c == '.' || c == '-' || c == '_');
  !is_valid
}

/// validate_name returns an error if the name is invalid
/// (we require 3-64 lowercase ascii letters, numbers, or . - _)
fn validate_name(name: &str) -> Result<(), ContractError> {
  let length = name.len() as u64;
  if (name.len() as u64) < MIN_NAME_LENGTH {
    Err(ContractError::NameTooShort {
      length,
      min_length: MIN_NAME_LENGTH,
    })
  } else if (name.len() as u64) > MAX_NAME_LENGTH {
    Err(ContractError::NameTooLong {
      length,
      max_length: MAX_NAME_LENGTH,
    })
  } else {
    match name.find(invalid_char) {
      None => Ok(()),
      Some(bytepos_invalid_char_start) => {
        let c = name[bytepos_invalid_char_start..].chars().next().unwrap();
        Err(ContractError::InvalidCharacter { c })
      }
    }
  }
}