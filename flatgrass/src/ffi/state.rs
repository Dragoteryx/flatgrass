use crate::lua::traits::{GetFromLua, PushToLua};
use crate::lua::LuaType;
use std::fmt::Display;
use super::*;

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LuaState(NonNull<c_void>);

#[allow(clippy::missing_safety_doc)]
impl LuaState {
  pub unsafe fn fg_getvalue<T: GetFromLua>(self, idx: c_int) -> Result<T, T::Error> {
    GetFromLua::try_get(self, idx)
  }

  pub unsafe fn fg_checkstack(self, size: c_int) {
    if self.lua_checkstack(size) == 0 {
      self.fg_error("stack overflow");
    }
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

  pub unsafe fn fg_print(self, value: impl PushToLua) {
    self.fg_checkstack(2);
    self.lua_getglobal(cstr!("print"));
    self.fg_pushvalue(value);
    self.lua_call(1, 0);
  }

  pub unsafe fn fg_error(self, error: impl PushToLua) -> ! {
    self.fg_checkstack(1);
    self.fg_pushvalue(error);
    self.lua_error();
    unreachable!();
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

  pub unsafe fn fg_local_error(self, error: impl Display) -> ! {
    self.luaL_where(1);
    let location: String = self.fg_getvalue(-1).unwrap();
    self.lua_pop(1);
    self.fg_error(format!("{location}{error}"));
  }

  pub unsafe fn fg_badarg_error(self, mut narg: c_int, error: impl Display) -> ! {
    match self.fg_getdebug(cstr!("n")) {
      None => self.fg_local_error(format!("bad argument #{narg} ({error})")),
      Some(debug) => {
        let mut name = debug.name();
        if debug.namewhat() == "method" {
          narg -= 1;
          if narg == 0 {
            self.fg_local_error(format!("calling '{name}' on bad self ({error})"));
          }
        }
        if name.is_empty() { name = "?"; }
        self.fg_local_error(format!("bad argument #{narg} to '{name}' ({error})"));
      }
    }
  }
}