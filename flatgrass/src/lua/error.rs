use std::error::Error;
use std::fmt;

use super::traits::ToLua;
use super::*;

pub type LuaResult<T> = Result<T, LuaError>;

#[derive(Debug, Clone)]
pub enum LuaError {
  UnexpectedType(String, String)
}

impl Error for LuaError {}

impl fmt::Display for LuaError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::UnexpectedType(expected, received) => write!(fmt, "expected {expected}, received {received}")
    }
  }
}

impl ToLua for LuaError {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, format!("{value}"));
  }
}