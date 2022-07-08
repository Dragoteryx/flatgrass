use super::*;

#[derive(Debug, Clone)]
pub enum GetFromLuaError {
  UnexpectedType(LuaType, LuaType),
  NoValue
}

impl Error for GetFromLuaError {}
impl fmt::Display for GetFromLuaError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedType(expected, got) => write!(f, "{expected} expected, got {got}"),
      Self::NoValue => write!(f, "got no value")
    }
  }
}