use std::fmt::Display;
use libc::c_void;
use super::*;

pub trait GetFromLua<'l>: Sized {
  type Error: Display;

  unsafe fn try_get(state: LuaState<'l>, idx: i32) -> Result<Self, Self::Error>;
}

// primitive types ---------------------------

impl<'l> GetFromLua<'l> for Nil {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Nil {
      Err(GetFromLuaError::UnexpectedType(LuaType::Nil, typ))
    } else {
      Ok(Nil)
    }
  }
}

impl<'l> GetFromLua<'l> for bool {
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

impl<'l> GetFromLua<'l> for f64 {
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

impl<'l> GetFromLua<'l> for String {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::String {
      Err(GetFromLuaError::UnexpectedType(LuaType::String, typ))
    } else {
      let mut len = 0;
      let ptr = state.lua_tolstring(idx, &mut len) as *const u8;
      let slice = std::slice::from_raw_parts(ptr, len);
      Ok(Self::from_utf8_lossy(slice).into_owned())
    }
  }
}

impl<'l> GetFromLua<'l> for *mut c_void {
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

// other number types --------------------

macro_rules! impl_get_number {
  ($num:ty) => {
    impl<'l> GetFromLua<'l> for $num {
      type Error = GetFromLuaError;

      unsafe fn try_get(state: LuaState, idx: i32) -> Result<Self, Self::Error> {
        f64::try_get(state, idx).map(|n| n as Self)
      }
    }
  };
}

impl_get_number!(i8);
impl_get_number!(i16);
impl_get_number!(i32);
impl_get_number!(i64);
impl_get_number!(i128);
impl_get_number!(isize);
impl_get_number!(u8);
impl_get_number!(u16);
impl_get_number!(u32);
impl_get_number!(u64);
impl_get_number!(u128);
impl_get_number!(usize);
impl_get_number!(f32);