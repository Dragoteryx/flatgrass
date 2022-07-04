use std::error::Error;
use std::fmt;

use super::traits::PushToLua;
use super::*;

pub type LuaResult<T> = Result<T, LuaError>;

#[derive(Debug, Clone)]
pub enum LuaError {
  NoValue,
  Custom(String)
}

impl Error for LuaError {}
impl fmt::Display for LuaError {
  fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Self::NoValue => write!(fmt, "got no value"),//Self::UnexpectedType(expected, received) => write!(fmt, "expected {expected}, received {received}")
      Self::Custom(str) => write!(fmt, "{str}")
    }
  }
}

impl PushToLua for LuaError {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(format!("{value}"));
  }
}