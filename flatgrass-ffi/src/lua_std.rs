use super::*;
use std::cell::UnsafeCell;
use std::marker::{PhantomData, PhantomPinned};

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(C)]
pub struct lua_State {
	not_ref_unwind_safe: PhantomData<UnsafeCell<u8>>,
	not_unwind_safe: PhantomData<&'static mut u8>,
	not_send_sync: PhantomData<*mut u8>,
	not_unpin: PhantomPinned,
}

/// See the Lua 5.1 manual: [`lua_Alloc`](https://www.lua.org/manual/5.1/manual.html#lua_Alloc)
pub type lua_Alloc = unsafe extern "C-unwind" fn(
	ud: *mut c_void,
	ptr: *mut c_void,
	osize: size_t,
	nsize: size_t,
) -> *mut c_void;

/// See the Lua 5.1 manual: [`lua_CFunction`](https://www.lua.org/manual/5.1/manual.html#lua_CFunction)
pub type lua_CFunction = unsafe extern "C-unwind" fn(state: *mut lua_State) -> c_int;

/// See the Lua 5.1 manual: [`lua_Reader`](https://www.lua.org/manual/5.1/manual.html#lua_Reader)
pub type lua_Reader = unsafe extern "C-unwind" fn(
	state: *mut lua_State,
	data: *mut c_void,
	size: *mut size_t,
) -> *const c_char;

/// See the Lua 5.1 manual: [`lua_Writer`](https://www.lua.org/manual/5.1/manual.html#lua_Writer)
pub type lua_Writer = unsafe extern "C-unwind" fn(
	state: *mut lua_State,
	ptr: *const c_void,
	size: size_t,
	data: *mut c_void,
) -> c_int;

/// See the Lua 5.1 manual: [`lua_call`](https://www.lua.org/manual/5.1/manual.html#lua_call)
pub const LUA_MULTRET: c_int = -1;

/// See the Lua 5.1 manual: [`Registry`](https://www.lua.org/manual/5.1/manual.html#3.5)
pub const LUA_REGISTRYINDEX: c_int = -10000;

/// See the Lua 5.1 manual: [`Pseudo-indices`](https://www.lua.org/manual/5.1/manual.html#3.3)
pub const LUA_ENVIRONINDEX: c_int = -10001;

/// See the Lua 5.1 manual: [`Pseudo-indices`](https://www.lua.org/manual/5.1/manual.html#3.3)
pub const LUA_GLOBALSINDEX: c_int = -10002;

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_YIELD: c_int = 1;

/// See the Lua 5.1 manual: [`lua_pcall`](https://www.lua.org/manual/5.1/manual.html#lua_pcall)
pub const LUA_ERRRUN: c_int = 2;

/// See the Lua 5.1 manual: [`lua_load`](https://www.lua.org/manual/5.1/manual.html#lua_load)
pub const LUA_ERRSYNTAX: c_int = 3;

/// See the Lua 5.1 manual: [`lua_pcall`](https://www.lua.org/manual/5.1/manual.html#lua_pcall)
pub const LUA_ERRMEM: c_int = 4;

/// See the Lua 5.1 manual: [`lua_pcall`](https://www.lua.org/manual/5.1/manual.html#lua_pcall)
pub const LUA_ERRERR: c_int = 5;

/// See the Lua 5.1 manual: [`luaL_loadfile`](https://www.lua.org/manual/5.1/manual.html#luaL_loadfile)
pub const LUA_ERRFILE: c_int = 6;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TNONE: c_int = -1;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TNIL: c_int = 0;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TBOOLEAN: c_int = 1;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TLIGHTUSERDATA: c_int = 2;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TNUMBER: c_int = 3;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TSTRING: c_int = 4;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TTABLE: c_int = 5;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TFUNCTION: c_int = 6;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TUSERDATA: c_int = 7;

/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
pub const LUA_TTHREAD: c_int = 8;

/// See the Lua 5.1 manual: [`LuaStack Size`](https://www.lua.org/manual/5.1/manual.html#3.2)
pub const LUA_MINSTACK: c_int = 20;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCSTOP: c_int = 0;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCRESTART: c_int = 1;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCCOLLECT: c_int = 2;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCCOUNT: c_int = 3;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCCOUNTB: c_int = 4;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCSTEP: c_int = 5;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCSETPAUSE: c_int = 6;

/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
pub const LUA_GCSETSTEPMUL: c_int = 7;

/// See the Lua 5.1 manual: [`C Closures`](https://www.lua.org/manual/5.1/manual.html#3.4)
pub const fn lua_upvalueindex(idx: c_int) -> c_int {
	LUA_GLOBALSINDEX.wrapping_sub(idx)
}

import_lua! {
	/// See the Lua 5.1 manual: [`lua_atpanic`](https://www.lua.org/manual/5.1/manual.html#lua_atpanic)
	pub fn lua_atpanic(state: *mut lua_State, at_panic: lua_CFunction) -> lua_CFunction;

	/// See the Lua 5.1 manual: [`lua_call`](https://www.lua.org/manual/5.1/manual.html#lua_call)
	pub fn lua_call(state: *mut lua_State, nargs: c_int, nresults: c_int);

	/// See the Lua 5.1 manual: [`lua_checkstack`](https://www.lua.org/manual/5.1/manual.html#lua_checkstack)
	pub fn lua_checkstack(state: *mut lua_State, size: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_close`](https://www.lua.org/manual/5.1/manual.html#lua_close)
	pub fn lua_close(state: *mut lua_State);

	/// See the Lua 5.1 manual: [`lua_concat`](https://www.lua.org/manual/5.1/manual.html#lua_concat)
	pub fn lua_concat(state: *mut lua_State, n: c_int);

	/// See the Lua 5.1 manual: [`lua_cpcall`](https://www.lua.org/manual/5.1/manual.html#lua_cpcall)
	pub fn lua_cpcall(state: *mut lua_State, func: lua_CFunction, ud: *mut c_void) -> c_int;

	/// See the Lua 5.1 manual: [`lua_createtable`](https://www.lua.org/manual/5.1/manual.html#lua_createtable)
	pub fn lua_createtable(state: *mut lua_State, narr: c_int, nrec: c_int);

	/// See the Lua 5.1 manual: [`lua_dump`](https://www.lua.org/manual/5.1/manual.html#lua_dump)
	pub fn lua_dump(state: *mut lua_State, writer: lua_Writer, data: *mut c_void) -> c_int;

	/// See the Lua 5.1 manual: [`lua_equal`](https://www.lua.org/manual/5.1/manual.html#lua_equal)
	pub fn lua_equal(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_error`](https://www.lua.org/manual/5.1/manual.html#lua_error)
	pub fn lua_error(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
	pub fn lua_gc(state: *mut lua_State, what: c_int, data: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_getallocf`](https://www.lua.org/manual/5.1/manual.html#lua_getallocf)
	pub fn lua_getallocf(state: *mut lua_State, ud: *mut *mut c_void) -> lua_Alloc;

	/// See the Lua 5.1 manual: [`lua_getfenv`](https://www.lua.org/manual/5.1/manual.html#lua_getfenv)
	pub fn lua_getfenv(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_getfield`](https://www.lua.org/manual/5.1/manual.html#lua_getfield)
	pub fn lua_getfield(state: *mut lua_State, idx: c_int, key: *const c_char);

	/// See the Lua 5.1 manual: [`lua_getglobal`](https://www.lua.org/manual/5.1/manual.html#lua_getglobal)
	pub fn lua_getglobal(state: *mut lua_State, name: *const c_char) {
		unsafe {
			lua_getfield(state, LUA_GLOBALSINDEX, name);
		}
	}

	/// See the Lua 5.1 manual: [`lua_getmetatable`](https://www.lua.org/manual/5.1/manual.html#lua_getmetatable)
	pub fn lua_getmetatable(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_gettable`](https://www.lua.org/manual/5.1/manual.html#lua_gettable)
	pub fn lua_gettable(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_gettop`](https://www.lua.org/manual/5.1/manual.html#lua_gettop)
	pub fn lua_gettop(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_insert`](https://www.lua.org/manual/5.1/manual.html#lua_insert)
	pub fn lua_insert(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_isboolean`](https://www.lua.org/manual/5.1/manual.html#lua_isboolean)
	pub fn lua_isboolean(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TBOOLEAN) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_iscfunction`](https://www.lua.org/manual/5.1/manual.html#lua_iscfunction)
	pub fn lua_iscfunction(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_function`](https://www.lua.org/manual/5.1/manual.html#lua_function)
	pub fn lua_isfunction(state: *mut lua_State, idx: c_int) -> c_int	{
		unsafe {
			(lua_type(state, idx) == LUA_TFUNCTION) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_islightuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_islightuserdata)
	pub fn lua_islightuserdata(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TLIGHTUSERDATA) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isnil`](https://www.lua.org/manual/5.1/manual.html#lua_isnil)
	pub fn lua_isnil(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TNIL) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isnone`](https://www.lua.org/manual/5.1/manual.html#lua_isnone)
	pub fn lua_isnone(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TNONE) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isnoneornil`](https://www.lua.org/manual/5.1/manual.html#lua_isnoneornil)
	pub fn lua_isnoneornil(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) <= 0) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isnumber`](https://www.lua.org/manual/5.1/manual.html#lua_isnumber)
	pub fn lua_isnumber(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_isstring`](https://www.lua.org/manual/5.1/manual.html#lua_isstring)
	pub fn lua_isstring(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_istable`](https://www.lua.org/manual/5.1/manual.html#lua_istable)
	pub fn lua_istable(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TTABLE) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isthread`](https://www.lua.org/manual/5.1/manual.html#lua_isthread)
	pub fn lua_isthread(state: *mut lua_State, idx: c_int) -> c_int {
		unsafe {
			(lua_type(state, idx) == LUA_TTHREAD) as _
		}
	}

	/// See the Lua 5.1 manual: [`lua_isuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_isuserdata)
	pub fn lua_isuserdata(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_lessthan`](https://www.lua.org/manual/5.1/manual.html#lua_lessthan)
	pub fn lua_lessthan(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_load`](https://www.lua.org/manual/5.1/manual.html#lua_load)
	pub fn lua_load(state: *mut lua_State, reader: lua_Reader, data: *mut c_void, chunkname: *const c_char) -> c_int;

	/// See the Lua 5.1 manual: [`lua_newstate`](https://www.lua.org/manual/5.1/manual.html#lua_newstate)
	pub fn lua_newstate(alloc_func: lua_Alloc, ud: *mut c_void) -> *mut lua_State;

	/// See the Lua 5.1 manual: [`lua_newtable`](https://www.lua.org/manual/5.1/manual.html#lua_newtable)
	pub fn lua_newtable(state: *mut lua_State) {
		unsafe {
			lua_createtable(state, 0, 0);
		}
	}

	/// See the Lua 5.1 manual: [`lua_newthread`](https://www.lua.org/manual/5.1/manual.html#lua_newthread)
	pub fn lua_newthread(state: *mut lua_State) -> *mut lua_State;

	/// See the Lua 5.1 manual: [`lua_newuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_newuserdata)
	pub fn lua_newuserdata(state: *mut lua_State, size: size_t) -> *mut c_void;

	/// See the Lua 5.1 manual: [`lua_next`](https://www.lua.org/manual/5.1/manual.html#lua_next)
	pub fn lua_next(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_objlen`](https://www.lua.org/manual/5.1/manual.html#lua_objlen)
	pub fn lua_objlen(state: *mut lua_State, idx: c_int) -> size_t;

	/// See the Lua 5.1 manual: [`lua_pcall`](https://www.lua.org/manual/5.1/manual.html#lua_pcall)
	pub fn lua_pcall(state: *mut lua_State, nargs: c_int, nresults: c_int, err_func: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_pop`](https://www.lua.org/manual/5.1/manual.html#lua_pop)
	pub fn lua_pop(state: *mut lua_State, n: c_int) {
		unsafe {
			lua_settop(state, -n-1);
		}
	}

	/// See the Lua 5.1 manual: [`lua_pushboolean`](https://www.lua.org/manual/5.1/manual.html#lua_pushboolean)
	pub fn lua_pushboolean(state: *mut lua_State, boolean: c_int);

	/// See the Lua 5.1 manual: [`lua_pushcclosure`](https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure)
	pub fn lua_pushcclosure(state: *mut lua_State, func: lua_CFunction, nvalues: c_int);

	/// See the Lua 5.1 manual: [`lua_pushcfunction`](https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction)
	pub fn lua_pushcfunction(state: *mut lua_State, func: lua_CFunction) {
		unsafe {
			lua_pushcclosure(state, func, 0)
		}
	}

	/// See the Lua 5.1 manual: [`lua_pushinteger`](https://www.lua.org/manual/5.1/manual.html#lua_pushinteger)
	pub fn lua_pushinteger(state: *mut lua_State, num: ptrdiff_t);

	/// See the Lua 5.1 manual: [`lua_pushlightuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_pushlightuserdata)
	pub fn lua_pushlightuserdata(state: *mut lua_State, ptr: *mut c_void);

	/// See the Lua 5.1 manual: [`lua_pushlstring`](https://www.lua.org/manual/5.1/manual.html#lua_pushlstring)
	pub fn lua_pushlstring(state: *mut lua_State, ptr: *const c_char, len: size_t);

	/// See the Lua 5.1 manual: [`lua_pushnil`](https://www.lua.org/manual/5.1/manual.html#lua_pushnil)
	pub fn lua_pushnil(state: *mut lua_State);

	/// See the Lua 5.1 manual: [`lua_pushnumber`](https://www.lua.org/manual/5.1/manual.html#lua_pushnumber)
	pub fn lua_pushnumber(state: *mut lua_State, num: c_double);

	/// See the Lua 5.1 manual: [`lua_pushstring`](https://www.lua.org/manual/5.1/manual.html#lua_pushstring)
	pub fn lua_pushstring(state: *mut lua_State, ptr: *const c_char);

	/// See the Lua 5.1 manual: [`lua_pushthread`](https://www.lua.org/manual/5.1/manual.html#lua_pushthread)
	pub fn lua_pushthread(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_pushvalue`](https://www.lua.org/manual/5.1/manual.html#lua_pushvalue)
	pub fn lua_pushvalue(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_rawequal`](https://www.lua.org/manual/5.1/manual.html#lua_rawequal)
	pub fn lua_rawequal(state: *mut lua_State, idx1: c_int, idx2: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_rawget`](https://www.lua.org/manual/5.1/manual.html#lua_rawget)
	pub fn lua_rawget(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_rawgeti`](https://www.lua.org/manual/5.1/manual.html#lua_rawgeti)
	pub fn lua_rawgeti(state: *mut lua_State, idx: c_int, n: c_int);

	/// See the Lua 5.1 manual: [`lua_rawset`](https://www.lua.org/manual/5.1/manual.html#lua_rawset)
	pub fn lua_rawset(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_rawseti`](https://www.lua.org/manual/5.1/manual.html#lua_rawseti)
	pub fn lua_rawseti(state: *mut lua_State, idx: c_int, n: c_int);

	/// See the Lua 5.1 manual: [`lua_register`](https://www.lua.org/manual/5.1/manual.html#lua_register)
	pub fn lua_register(state: *mut lua_State, name: *const c_char, func: lua_CFunction) {
		unsafe {
			lua_pushcfunction(state, func);
			lua_setglobal(state, name);
		}
	}

	/// See the Lua 5.1 manual: [`lua_remove`](https://www.lua.org/manual/5.1/manual.html#lua_remove)
	pub fn lua_remove(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_replace`](https://www.lua.org/manual/5.1/manual.html#lua_replace)
	pub fn lua_replace(state: *mut lua_State, idx: c_int);

	/// Why is this necessary?
	fn lua_resume_real(state: *mut lua_State, nargs: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_resume`](https://www.lua.org/manual/5.1/manual.html#lua_resume)
	pub fn lua_resume(state: *mut lua_State, nargs: c_int) -> c_int {
		unsafe {
			lua_resume_real(state, nargs)
		}
	}

	/// See the Lua 5.1 manual: [`lua_setallocf`](https://www.lua.org/manual/5.1/manual.html#lua_setallocf)
	pub fn lua_setallocf(state: *mut lua_State, alloc_func: lua_Alloc, ud: *mut c_void);

	/// See the Lua 5.1 manual: [`lua_setfenv`](https://www.lua.org/manual/5.1/manual.html#lua_setfenv)
	pub fn lua_setfenv(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_setfield`](https://www.lua.org/manual/5.1/manual.html#lua_setfield)
	pub fn lua_setfield(state: *mut lua_State, idx: c_int, key: *const c_char);

	/// See the Lua 5.1 manual: [`lua_setglobal`](https://www.lua.org/manual/5.1/manual.html#lua_setglobal)
	pub fn lua_setglobal(state: *mut lua_State, name: *const c_char) {
		unsafe {
			lua_setfield(state, LUA_GLOBALSINDEX, name);
		}
	}

	/// See the Lua 5.1 manual: [`lua_setmetatable`](https://www.lua.org/manual/5.1/manual.html#lua_setmetatable)
	pub fn lua_setmetatable(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_settable`](https://www.lua.org/manual/5.1/manual.html#lua_settable)
	pub fn lua_settable(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_settop`](https://www.lua.org/manual/5.1/manual.html#lua_settop)
	pub fn lua_settop(state: *mut lua_State, idx: c_int);

	/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
	pub fn lua_status(state: *mut lua_State) -> c_int;

	/// See the Lua 5.1 manual: [`lua_toboolean`](https://www.lua.org/manual/5.1/manual.html#lua_toboolean)
	pub fn lua_toboolean(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_tocfunction`](https://www.lua.org/manual/5.1/manual.html#lua_tocfunction)
	pub fn lua_tocfunction(state: *mut lua_State, idx: c_int) -> Option<lua_CFunction>;

	/// See the Lua 5.1 manual: [`lua_tointeger`](https://www.lua.org/manual/5.1/manual.html#lua_tointeger)
	pub fn lua_tointeger(state: *mut lua_State, idx: c_int) -> ptrdiff_t;

	/// See the Lua 5.1 manual: [`lua_tolstring`](https://www.lua.org/manual/5.1/manual.html#lua_tolstring)
	pub fn lua_tolstring(state: *mut lua_State, idx: c_int, len: *mut size_t) -> *const c_char;

	/// See the Lua 5.1 manual: [`lua_tonumber`](https://www.lua.org/manual/5.1/manual.html#lua_tonumber)
	pub fn lua_tonumber(state: *mut lua_State, idx: c_int) -> c_double;

	/// See the Lua 5.1 manual: [`lua_topointer`](https://www.lua.org/manual/5.1/manual.html#lua_topointer)
	pub fn lua_topointer(state: *mut lua_State, idx: c_int) -> *const c_void;

	/// See the Lua 5.1 manual: [`lua_tostring`](https://www.lua.org/manual/5.1/manual.html#lua_tostring)
	pub fn lua_tostring(state: *mut lua_State, idx: c_int) -> *const c_char	{
		unsafe {
			lua_tolstring(state, idx, std::ptr::null_mut())
		}
	}

	/// See the Lua 5.1 manual: [`lua_tothread`](https://www.lua.org/manual/5.1/manual.html#lua_tothread)
	pub fn lua_tothread(state: *mut lua_State, idx: c_int) -> *mut lua_State;

	/// See the Lua 5.1 manual: [`lua_touserdata`](https://www.lua.org/manual/5.1/manual.html#lua_touserdata)
	pub fn lua_touserdata(state: *mut lua_State, idx: c_int) -> *mut c_void;

	/// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
	pub fn lua_type(state: *mut lua_State, idx: c_int) -> c_int;

	/// See the Lua 5.1 manual: [`lua_typename`](https://www.lua.org/manual/5.1/manual.html#lua_typename)
	pub fn lua_typename(state: *mut lua_State, tp: c_int) -> *const c_char;

	/// See the Lua 5.1 manual: [`lua_xmove`](https://www.lua.org/manual/5.1/manual.html#lua_xmove)
	pub fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: c_int);

	/// See the Lua 5.1 manual: [`lua_yield`](https://www.lua.org/manual/5.1/manual.html#lua_yield)
	pub fn lua_yield(state: *mut lua_State, nresults: c_int) -> c_int;
}
