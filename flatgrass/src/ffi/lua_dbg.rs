use super::*;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub type LuaHook = unsafe extern "C-unwind" fn(state: LuaState, debug: *mut LuaDebug);

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKCALL: c_int =	0;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKRET: c_int =	1;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKLINE: c_int =	2;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKCOUNT: c_int =	3;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKTAILRET: c_int = 4;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKCALL: c_int =	1 << LUA_HOOKCALL;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKRET: c_int = 1 << LUA_HOOKRET;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKLINE: c_int =	1 << LUA_HOOKLINE;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKCOUNT: c_int = 1 << LUA_HOOKCOUNT;

impl LuaState {
  fetch_lua!(fn lua_gethook(self) -> LuaHook);
  fetch_lua!(fn lua_gethookcount(self) -> c_int);
  fetch_lua!(fn lua_gethookmask(self) -> c_int);
  fetch_lua!(fn lua_getinfo(self, what: *const c_char, debug: *mut LuaDebug) -> c_int);
  fetch_lua!(fn lua_getlocal(self, debug: *mut LuaDebug, n: c_int) -> *const c_char);
  fetch_lua!(fn lua_getstack(self, lvl: c_int, debug: *mut LuaDebug) -> c_int);
  fetch_lua!(fn lua_getupvalue(self, idx: c_int, n: c_int) -> *const c_char);
  fetch_lua!(fn lua_sethook(self, hook: LuaHook, mask: c_int, count: c_int) -> c_int);
  fetch_lua!(fn lua_setlocal(self, debug: *mut LuaDebug, n: c_int) -> *const c_char);
  fetch_lua!(fn lua_setupvalue(self, idx: c_int, n: c_int) -> *const c_char);
}

// todo: add the functions from Lua 5.2