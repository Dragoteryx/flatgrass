use std::convert::Infallible;

use super::*;

pub trait ToLua {
  unsafe fn push(state: LuaState, value: Self);
}

impl ToLua for () {
  unsafe fn push(state: LuaState, _: Self) {
    lua_pushnil(state);
  }
}

impl ToLua for bool {
  unsafe fn push(state: LuaState, value: Self) {
    lua_pushboolean(state, if value { 1 } else { 0 });
  }
}

impl ToLua for f64 {
  unsafe fn push(state: LuaState, value: Self) {
    lua_pushnumber(state, value);
  }
}

impl ToLua for isize {
  unsafe fn push(state: LuaState, value: Self) {
    lua_pushinteger(state, value);
  }
}

impl ToLua for &str {
  unsafe fn push(state: LuaState, value: Self) {
    let ptr = value.as_ptr() as *const i8;
    lua_pushlstring(state, ptr, value.len());
  }
}

impl ToLua for &String {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value as &str);
  }
}

impl ToLua for String {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, &value);
  }
}

impl<T: ToLua> ToLua for Option<T> {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Some(value) => ToLua::push(state, value),
      None => lua_pushnil(state)
    }
  }
}

impl ToLua for Infallible {
  unsafe fn push(_: LuaState, _: Self) {
    unreachable!()
  }
}