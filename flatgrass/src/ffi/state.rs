use crate::lua::traits::{GetFromLua, PushToLua};
use crate::lua::LuaType;
use super::*;

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LuaState(NonNull<c_void>);

#[allow(clippy::missing_safety_doc)]
impl LuaState {
  pub unsafe fn fg_checkstack(self, size: c_int) {
    if self.lua_checkstack(size) == 0 {
      panic!("stack overflow");
    }
  }

  pub unsafe fn fg_getvalue<T: GetFromLua>(self, idx: c_int) -> Result<T, T::Error> {
    GetFromLua::try_get(self, idx)
  }

  pub unsafe fn fg_pushvalue(self, value: impl PushToLua) {
    PushToLua::push(self, value);
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

  pub unsafe fn fg_getdebug(self, what: *const c_char) -> Option<LuaDebug> {
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