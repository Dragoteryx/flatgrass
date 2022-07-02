use super::*;

pub trait FromLua: Sized {
  type Error: ToLua;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error>;
}

impl FromLua for () {
  type Error = LuaError;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl FromLua for bool {
  type Error = LuaError;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl FromLua for isize {
  type Error = LuaError;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl FromLua for f64 {
  type Error = LuaError;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl FromLua for String {
  type Error = LuaError;

  unsafe fn pop(state: LuaState) -> Result<Self, Self::Error> {
    todo!()
  }
}