use super::*;

/// See the Lua 5.1 manual: [`lua_Debug`](https://www.lua.org/manual/5.1/manual.html#lua_Debug)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LuaDebug {
  pub event: c_int,
  pub name: *const c_char,
  pub namewhat: *const c_char,
  pub what: *const c_char,
  pub source: *const c_char,
  pub currentline: c_int,
  pub nups: c_int,
  pub linedefined: c_int,
  pub lastlinedefined: c_int,
  pub short_src: [c_char; 128],
  i_ci: c_int
}

impl Default for LuaDebug {
	fn default() -> Self {
		Self {
			event: 0,
      name: std::ptr::null(),
      namewhat: std::ptr::null(),
      what: std::ptr::null(),
      source: std::ptr::null(),
      currentline: 0, nups: 0,
      linedefined: 0,
      lastlinedefined: 0,
      short_src: [0; 128],
      i_ci: 0
		}
	}
}

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