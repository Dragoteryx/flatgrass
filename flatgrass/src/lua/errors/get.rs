use std::str::Utf8Error;
use super::*;

#[derive(Debug, Clone)]
pub enum GetFromLuaError {
  UnexpectedType(LuaType, LuaType),
  NoValue,
  Utf8Error(Utf8Error)
}

impl Error for GetFromLuaError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      Self::Utf8Error(err) => Some(err),
      _ => None
    }
  }
}

impl fmt::Display for GetFromLuaError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedType(expected, got) => write!(f, "{expected} expected, got {got}"),
      Self::NoValue => write!(f, "got no value"),
      Self::Utf8Error(err) => write!(f, "{err}")
    }
  }
}