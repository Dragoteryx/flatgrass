use libc::c_void;
use super::*;

pub trait PushToLua: Sized {
  unsafe fn push(state: LuaState, value: Self);
}

// primitive lua types ---------------------------

impl PushToLua for () {
  unsafe fn push(state: LuaState, _: Self) {
    state.lua_pushnil();
  }
}

impl PushToLua for bool {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushboolean(if value { 1 } else { 0 });
  }
}

impl PushToLua for f64 {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushnumber(value);
  }
}

impl PushToLua for isize {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushinteger(value);
  }
}

impl PushToLua for &str {
  unsafe fn push(state: LuaState, value: Self) {
    let ptr = value.as_ptr() as *const i8;
    state.lua_pushlstring(ptr, value.len());
  }
}

impl PushToLua for &String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value as &str);
  }
}

impl PushToLua for String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value);
  }
}

impl PushToLua for LuaCFunction {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushcfunction(value);
  }
}

impl PushToLua for *mut c_void {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_pushlightuserdata(value);
  }
}

// other numbers types --------------------------

impl PushToLua for f32 {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value as f64);
  }
}

macro_rules! impl_pushtolua_signed {
  ($int:ty) => {
    impl PushToLua for $int {
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
    impl PushToLua for $uint {
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

impl<T: PushToLua> PushToLua for Option<T> {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Some(value) => state.fg_pushvalue(value),
      None => state.fg_pushvalue(())
    }
  }
}

// slice types ------------------------------------

impl<'l, T: 'l> PushToLua for &'l [T] where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(&lua, value.iter()))
  }
}

impl<const N: usize, T: PushToLua> PushToLua for [T; N] {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(&lua, value.into_iter()))
  }
}

impl<'l, T: 'l> PushToLua for &'l Vec<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(&lua, value.iter()))
  }
}

impl<T: PushToLua> PushToLua for Vec<T> {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_list(&lua, value.into_iter()))
  }
}

// hash types --------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl<'l, K: 'l, V: 'l> PushToLua for &'l HashMap<K, V>
where &'l K: PushToLua,
      &'l V: PushToLua {

  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(&lua, value.iter()))
  }    
}

impl<K: PushToLua, V: PushToLua> PushToLua for HashMap<K, V> {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(&lua, value.into_iter()))
  }    
}

impl<'l, T: 'l> PushToLua for &'l HashSet<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(&lua, value.iter()))
  }    
}

impl<T: PushToLua> PushToLua for HashSet<T> {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(&lua, value.into_iter()))
  }    
}

// btree types -------------------------------------

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl<'l, K: 'l, V: 'l> PushToLua for &'l BTreeMap<K, V>
where &'l K: PushToLua,
      &'l V: PushToLua {

  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(&lua, value.iter()))
  }    
}

impl<K: PushToLua, V: PushToLua> PushToLua for BTreeMap<K, V> {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_map(&lua, value.into_iter()))
  }    
}

impl<'l, T: 'l> PushToLua for &'l BTreeSet<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(&lua, value.iter()))
  }    
}

impl<T: PushToLua> PushToLua for BTreeSet<T> {
  unsafe fn push(state: LuaState, value: Self) {
    let lua = Lua::from_state(state);
    state.fg_pushvalue(Table::new_set(&lua, value.into_iter()))
  }    
}