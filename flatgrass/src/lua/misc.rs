use std::fmt;
use super::*;

mod tuple; pub use tuple::*;
mod gc; pub use gc::*;

// nil type ---------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Nil;

// lua type ---------------------------

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

// realm -------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Realm {
  Server,
  Client,
  Menu
}

impl<'l> LuaArg<'l> for Realm {
  type Error = &'static str;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Self::from_state(state).ok_or("invalid realm")
  }
}

impl Realm {
  pub unsafe fn from_state(state: LuaState) -> Option<Self> {
    let globals = Globals::from_state(state);
    let server = globals.get("SERVER").and_then(|v| v.try_as().ok()).unwrap_or_default();
    let client = globals.get("CLIENT").and_then(|v| v.try_as().ok()).unwrap_or_default();
    let menu = globals.get("MENU_DLL").and_then(|v| v.try_as().ok()).unwrap_or_default();
    match (server, client, menu) {
      (true, false, false) => Some(Self::Server),
      (false, true, false) => Some(Self::Client),
      (false, false, true) => Some(Self::Menu),
      _ => None
    }
  }
}