use crate::lua::traits::{GetFromLua, PushToLua, PushManyToLua};
use crate::lua::misc::LuaType;
use std::marker::PhantomData;
use std::fmt;
use super::*;

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LuaState<'l> {
  phantom: PhantomData<&'l ()>,
  ptr: NonNull<c_void>
}

// misc impls -----------------------

impl<'l> fmt::Pointer for LuaState<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:p}", self.ptr)
  }
}

impl<'l> fmt::Debug for LuaState<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "LuaState({self:p})")
  }
}

// main impl ------------------------

#[allow(clippy::missing_safety_doc)]
impl<'l> LuaState<'l> {
  pub unsafe fn fg_getvalue<T: GetFromLua<'l>>(self, idx: c_int) -> Result<T, T::Error> {
    GetFromLua::try_get(self, idx)
  }

  pub unsafe fn fg_checkstack(self, size: c_int) {
    if self.lua_checkstack(size) == 0 {
      panic!("stack overflow");
    }
  }

  pub unsafe fn fg_pushvalue(self, value: impl PushToLua) {
    PushToLua::push(self, value);
  }

  pub unsafe fn fg_pushmany(self, values: impl PushManyToLua) -> i32 {
    let top = self.lua_gettop();
    PushManyToLua::push_many(self, values);
    self.lua_gettop() - top
  }

  pub unsafe fn fg_pushindex(self, idx: i32) {
    self.fg_checkstack(1);
    self.lua_pushvalue(idx);
  }

  pub unsafe fn fg_pushtable(self) {
    self.fg_checkstack(1);
    self.lua_newtable();
  }

  pub unsafe fn fg_pushfunction(self, func: LuaCFunction, upvalues: impl PushManyToLua) {
    let nvalues = self.fg_pushmany(upvalues);
    self.fg_checkstack(1);
    self.lua_pushcclosure(func, nvalues);
  }

  pub unsafe fn fg_type(self, idx: c_int) -> LuaType {
    match self.lua_type(idx) {
      LUA_TNONE => LuaType::None,
      LUA_TNIL => LuaType::Nil,
      LUA_TBOOLEAN => LuaType::Boolean,
      LUA_TNUMBER => LuaType::Number,
      LUA_TSTRING => LuaType::String,
      LUA_TTABLE => LuaType::Table,
      LUA_TFUNCTION => LuaType::Function,
      LUA_TUSERDATA => LuaType::Userdata,
      LUA_TTHREAD => LuaType::Thread,
      LUA_TLIGHTUSERDATA => LuaType::LightUserdata,
      _ => unreachable!()
    }
  }

  pub unsafe fn fg_debug(self, what: *const c_char) -> Option<LuaDebug> {
    let mut debug = LuaDebug::default();
    if self.lua_getstack(0, &mut debug) == 0 {
      None
    } else {
      self.lua_getinfo(what, &mut debug);
      Some(debug)
    }
  }

  pub unsafe fn fg_where(self, lvl: c_int) -> String {
    self.fg_checkstack(1);
    self.luaL_where(lvl);
    let location = self.fg_getvalue(-1).unwrap();
    self.lua_pop(1);
    location
  }
}