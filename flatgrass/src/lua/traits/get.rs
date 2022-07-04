use std::str::Utf8Error;
use std::error::Error;
use std::fmt;
use super::*;

pub trait GetFromLua: Sized {
  type Error: fmt::Display;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GetFromLuaError {
  UnexpectedType(LuaType, LuaType),
  Utf8Error(Utf8Error)
}

impl Error for GetFromLuaError {}
impl fmt::Display for GetFromLuaError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::UnexpectedType(expected, got) => write!(f, "expected {expected}, got {got}"),
      Self::Utf8Error(err) => write!(f, "{err}")
    }
  }
}

// primitive types ---------------------------

impl GetFromLua for () {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Nil {
      Err(GetFromLuaError::UnexpectedType(LuaType::Nil, typ))
    } else {
      Ok(())
    }
  }
}

impl GetFromLua for bool {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Boolean {
      Err(GetFromLuaError::UnexpectedType(LuaType::Boolean, typ))
    } else {
      Ok(state.lua_toboolean(idx) != 0)
    }
  }
}

impl GetFromLua for f64 {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Number {
      Err(GetFromLuaError::UnexpectedType(LuaType::Number, typ))
    } else {
      Ok(state.lua_tonumber(idx))
    }
  }
}

impl GetFromLua for isize {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Number {
      Err(GetFromLuaError::UnexpectedType(LuaType::Number, typ))
    } else {
      Ok(state.lua_tointeger(idx))
    }
  }
}

impl GetFromLua for String {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::String {
      Err(GetFromLuaError::UnexpectedType(LuaType::String, typ))
    } else {
      let mut len = 0;
      let ptr = state.lua_tolstring(idx, &mut len) as *const u8;
      let slice = std::slice::from_raw_parts(ptr, len);
      match std::str::from_utf8(slice) {
        Err(err) => Err(GetFromLuaError::Utf8Error(err)),
        Ok(str) => Ok(Self::from(str))
      }
    }
  }
}

// lua types

impl<'l> GetFromLua for LuaTable<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Table {
      Err(GetFromLuaError::UnexpectedType(LuaType::Table, typ))
    } else {
      state.fg_checkstack(1);
      state.fg_pushvalue(idx);
      Ok(LuaTable::pop(state))
    }
  }
}