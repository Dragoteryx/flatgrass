use std::fmt;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaType {
  None,
  Nil,
  Boolean,
  Number,
  String,
  Table,
  Function,
  Userdata,
  Thread,
  LightUserdata
}

impl fmt::Display for LuaType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::None => write!(f, "no value"),
      Self::Nil => write!(f, "nil"),
      Self::Boolean => write!(f, "boolean"),
      Self::Number => write!(f, "number"),
      Self::String => write!(f, "string"),
      Self::Table => write!(f, "table"),
      Self::Function => write!(f, "function"),
      Self::Userdata => write!(f, "userdata"),
      Self::Thread => write!(f, "thread"),
      Self::LightUserdata => write!(f, "lightuserdata")
    }
  }    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Realm {
  Server,
  Client,
  Menu
}

impl LuaArg for Realm {
  type Error = Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Lua::from_state(state).realm())
  }
}