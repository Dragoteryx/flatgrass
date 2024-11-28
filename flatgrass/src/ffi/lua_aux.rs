use super::*;

/// See the Lua 5.1 manual: [`luaL_Reg`](https://www.lua.org/manual/5.1/manual.html#luaL_Reg)
#[repr(C)]
pub struct luaL_Reg {
	pub name: *const c_char,
	pub func: Option<lua_CFunction>,
}

impl Default for luaL_Reg {
	fn default() -> Self {
		Self {
			name: std::ptr::null(),
			func: None,
		}
	}
}

/// See the Lua 5.1 manual: [`luaL_ref`](https://www.lua.org/manual/5.1/manual.html#luaL_ref)
pub const LUA_NOREF: c_int = -2;

/// See the Lua 5.1 manual: [`luaL_ref`](https://www.lua.org/manual/5.1/manual.html#luaL_ref)
pub const LUA_REFNIL: c_int = -1;

import_lua! {
	/// See the Lua 5.1 manual: [`luaL_ref`](https://www.lua.org/manual/5.1/manual.html#luaL_ref)
	pub fn luaL_ref(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`luaL_unref`](https://www.lua.org/manual/5.1/manual.html#luaL_unref)
	pub fn luaL_unref(state: *mut lua_State, idx: c_int, rf: c_int);

	/// See the Lua 5.1 manual: [`luaL_where`](https://www.lua.org/manual/5.1/manual.html#luaL_where)
	pub fn luaL_where(state: *mut lua_State, lvl: c_int);
}
