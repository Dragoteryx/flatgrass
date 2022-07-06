use super::*;

/// See the Lua 5.1 manual: [`luaL_Reg`](https://www.lua.org/manual/5.1/manual.html#luaL_Reg)
#[repr(C)]
pub struct LuaLReg {
  pub name: *const c_char,
  pub func: LuaCFunction
}

/// See the Lua 5.1 manual: [`luaL_ref`](https://www.lua.org/manual/5.1/manual.html#luaL_ref)
pub const LUA_NOREF: c_int = -2;

/// See the Lua 5.1 manual: [`luaL_ref`](https://www.lua.org/manual/5.1/manual.html#luaL_ref)
pub const LUA_REFNIL: c_int = -1;

impl LuaState<'_> {
  fetch_lua!(fn luaL_argerror(self, narg: c_int, msg: *const c_char));
  fetch_lua!(fn luaL_ref(self, idx: c_int) -> c_int);
  fetch_lua!(fn luaL_unref(self, idx: c_int, r#ref: c_int));
  fetch_lua!(fn luaL_where(self, lvl: c_int));
}