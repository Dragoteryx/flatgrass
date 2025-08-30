use super::*;

/// See the Lua 5.1 manual: [`lua_Debug`](https://www.lua.org/manual/5.1/manual.html#lua_Debug)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct lua_Debug {
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
	i_ci: c_int,
}

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub type lua_Hook = unsafe extern "C-unwind" fn(state: *mut lua_State, debug: *mut lua_Debug);

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKCALL: c_int = 0;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKRET: c_int = 1;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKLINE: c_int = 2;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKCOUNT: c_int = 3;

/// See the Lua 5.1 manual: [`lua_Hook`](https://www.lua.org/manual/5.1/manual.html#lua_Hook)
pub const LUA_HOOKTAILRET: c_int = 4;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKCALL: c_int = 1 << LUA_HOOKCALL;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKRET: c_int = 1 << LUA_HOOKRET;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKLINE: c_int = 1 << LUA_HOOKLINE;

/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
pub const LUA_MASKCOUNT: c_int = 1 << LUA_HOOKCOUNT;

import_lua! {
	/// See the Lua 5.1 manual: [`lua_gethook`](https://www.lua.org/manual/5.1/manual.html#lua_gethook)
	pub fn lua_gethook(state: *mut lua_State) -> lua_Hook;

	/// See the Lua 5.1 manual: [`lua_gethookcount`](https://www.lua.org/manual/5.1/manual.html#lua_gethookcount)
	pub fn lua_gethookcount(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_gethookmask`](https://www.lua.org/manual/5.1/manual.html#lua_gethookmask)
	pub fn lua_gethookmask(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_getinfo`](https://www.lua.org/manual/5.1/manual.html#lua_getinfo)
	pub fn lua_getinfo(state: *mut lua_State, what: *const c_char, debug: *mut lua_Debug) -> c_int;

	/// See the Lua 5.1 manual: [`lua_getlocal`](https://www.lua.org/manual/5.1/manual.html#lua_getlocal)
	pub fn lua_getlocal(state: *mut lua_State, debug: *mut lua_Debug, n: c_int) -> *const c_char;

	/// See the Lua 5.1 manual: [`lua_getstack`](https://www.lua.org/manual/5.1/manual.html#lua_getstack)
	pub fn lua_getstack(state: *mut lua_State, lvl: c_int, debug: *mut lua_Debug) -> c_int;

	/// See the Lua 5.1 manual: [`lua_getupvalue`](https://www.lua.org/manual/5.1/manual.html#lua_getupvalue)
	pub fn lua_getupvalue(state: *mut lua_State, idx: c_int, n: c_int) -> *const c_char;

	/// See the Lua 5.1 manual: [`lua_sethook`](https://www.lua.org/manual/5.1/manual.html#lua_sethook)
	pub fn lua_sethook(state: *mut lua_State, hook: lua_Hook, mask: c_int, count: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_setlocal`](https://www.lua.org/manual/5.1/manual.html#lua_setlocal)
	pub fn lua_setlocal(state: *mut lua_State, debug: *mut lua_Debug, n: c_int) -> *const c_char;

	/// See the Lua 5.1 manual: [`lua_setupvalue`](https://www.lua.org/manual/5.1/manual.html#lua_setupvalue)
	pub fn lua_setupvalue(state: *mut lua_State, idx: c_int, n: c_int) -> *const c_char;

	/// Backported from Lua 5.2: [`lua_upvalueid`](https://www.lua.org/manual/5.2/manual.html#lua_upvalueid)
	pub fn lua_upvalueid(state: *mut lua_State, idx: c_int, n: c_int) -> *mut c_void;

	/// Backported from Lua 5.2: [`lua_upvaluejoin`](https://www.lua.org/manual/5.2/manual.html#lua_upvaluejoin)
	pub fn lua_upvaluejoin(state: *mut lua_State, idx1: c_int, n1: c_int, idx2: c_int, n2: c_int);
}
