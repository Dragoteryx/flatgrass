use libc::c_void;
use std::fmt;
use super::*;

pub trait GetFromLua: Sized {
  type Error: fmt::Display;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error>;
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

impl GetFromLua for *mut c_void {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::LightUserdata {
      Err(GetFromLuaError::UnexpectedType(LuaType::LightUserdata, typ))
    } else {
      Ok(state.lua_touserdata(idx))
    }
  }
}

// lua types

impl<'l> GetFromLua for LuaValue<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    if state.fg_type(idx) == LuaType::None {
      Err(GetFromLuaError::NoValue)
    } else {
      state.fg_checkstack(1);
      state.lua_pushvalue(idx);
      Ok(Self::pop(state))
    }
  }
}

impl<'l> GetFromLua for Table<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Table {
      Err(GetFromLuaError::UnexpectedType(LuaType::Table, typ))
    } else {
      state.fg_checkstack(1);
      state.lua_pushvalue(idx);
      Ok(Self::pop(state))
    }
  }
}

impl<'l> GetFromLua for Function<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Function {
      Err(GetFromLuaError::UnexpectedType(LuaType::Function, typ))
    } else {
      state.fg_checkstack(1);
      state.lua_pushvalue(idx);
      Ok(Self::pop(state))
    }
  }
}