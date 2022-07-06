use super::table::Table;
use libc::c_void;
use super::*;

pub trait PushToLua<'l>: Sized {
  unsafe fn push(state: LuaState<'l>, value: Self);
}

// primitive lua types ---------------------------

impl<'l> PushToLua<'l> for () {
  unsafe fn push(state: LuaState, _: Self) {
    state.lua_pushnil();
  }
}

impl<'l> PushToLua<'l> for bool {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushboolean(if value { 1 } else { 0 });
  }
}

impl<'l> PushToLua<'l> for f64 {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushnumber(value);
  }
}

impl<'l> PushToLua<'l> for isize {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushinteger(value);
  }
}

impl<'l> PushToLua<'l> for &str {
  unsafe fn push(state: LuaState, value: Self) {
    let ptr = value.as_ptr() as *const i8;
    state.lua_pushlstring(ptr, value.len());
  }
}

impl<'l> PushToLua<'l> for &String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value as &str);
  }
}

impl<'l> PushToLua<'l> for String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value);
  }
}

impl<'l> PushToLua<'l> for LuaCFunction {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushcfunction(value);
  }
}

impl<'l> PushToLua<'l> for *mut c_void {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushlightuserdata(value);
  }
}

// other numbers types --------------------------

impl<'l> PushToLua<'l> for f32 {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value as f64);
  }
}

macro_rules! impl_pushtolua_signed {
  ($int:ty) => {
    impl<'l> PushToLua<'l> for $int {
      unsafe fn push(state: LuaState, value: Self) {
        match isize::try_from(value) {
          Ok(value) => state.fg_pushvalue(value),
          Err(_) if value > 0 => state.fg_pushvalue(f64::INFINITY),
          Err(_) =>  state.fg_pushvalue(f64::NEG_INFINITY)
        }
      }
    }
  };
}

impl_pushtolua_signed!(i8);
impl_pushtolua_signed!(i16);
impl_pushtolua_signed!(i32);
impl_pushtolua_signed!(i64);
impl_pushtolua_signed!(i128);

macro_rules! impl_pushtolua_unsigned {
  ($uint:ty) => {
    impl<'l> PushToLua<'l> for $uint {
      unsafe fn push(state: LuaState, value: Self) {
        match isize::try_from(value) {
          Ok(value) => state.fg_pushvalue(value),
          Err(_) => state.fg_pushvalue(f64::INFINITY)
        }
      }
    }
  };
}

impl_pushtolua_unsigned!(u8);
impl_pushtolua_unsigned!(u16);
impl_pushtolua_unsigned!(u32);
impl_pushtolua_unsigned!(u64);
impl_pushtolua_unsigned!(u128);
impl_pushtolua_unsigned!(usize);

// misc types ------------------------------

impl<'l, T: 'l> PushToLua<'l> for &'l Option<T> where &'l T: PushToLua<'l> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    match value {
      Some(value) => state.fg_pushvalue(value),
      None => state.fg_pushvalue(())
    }
  }
}

impl<'l, T: PushToLua<'l>> PushToLua<'l> for Option<T> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    match value {
      Some(value) => state.fg_pushvalue(value),
      None => state.fg_pushvalue(())
    }
  }
}

// slice types ------------------------------------

impl<'l, T: 'l> PushToLua<'l> for &'l [T] where &'l T: PushToLua<'l> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(lua, value.iter()))
  }
}

impl<'l, const N: usize, T: PushToLua<'l>> PushToLua<'l> for [T; N] {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(lua, value.into_iter()))
  }
}

impl<'l, T: 'l> PushToLua<'l> for &'l Vec<T> where &'l T: PushToLua<'l> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(lua, value.iter()))
  }
}

impl<'l, T: PushToLua<'l>> PushToLua<'l> for Vec<T> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(lua, value.into_iter()))
  }
}

// hash types --------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl<'l, K: 'l, V: 'l> PushToLua<'l> for &'l HashMap<K, V>
where &'l K: PushToLua<'l>,
      &'l V: PushToLua<'l> {

  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(lua, value.iter()))
  }    
}

impl<'l, K: PushToLua<'l>, V: PushToLua<'l>> PushToLua<'l> for HashMap<K, V> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(lua, value.into_iter()))
  }    
}

impl<'l, T: 'l> PushToLua<'l> for &'l HashSet<T> where &'l T: PushToLua<'l> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(lua, value.iter()))
  }    
}

impl<'l, T: PushToLua<'l>> PushToLua<'l> for HashSet<T> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(lua, value.into_iter()))
  }    
}

// btree types -------------------------------------

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl<'l, K: 'l, V: 'l> PushToLua<'l> for &'l BTreeMap<K, V>
where &'l K: PushToLua<'l>,
      &'l V: PushToLua<'l> {

  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(lua, value.iter()))
  }    
}

impl<'l, K: PushToLua<'l>, V: PushToLua<'l>> PushToLua<'l> for BTreeMap<K, V> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(lua, value.into_iter()))
  }    
}

impl<'l, T: 'l> PushToLua<'l> for &'l BTreeSet<T> where &'l T: PushToLua<'l> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(lua, value.iter()))
  }    
}

impl<'l, T: PushToLua<'l>> PushToLua<'l> for BTreeSet<T> {
  unsafe fn push(state: LuaState<'l>, value: Self) {
    let lua = &Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(lua, value.into_iter()))
  }    
}