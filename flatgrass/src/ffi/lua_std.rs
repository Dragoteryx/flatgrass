use super::*;

// state manipulation ------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_newstate`](https://www.lua.org/manual/5.1/manual.html#lua_newstate)
  pub fn lua_newstate(alloc_func: LuaAlloc, ud: *mut c_void) -> Option<LuaState>;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_close`](https://www.lua.org/manual/5.1/manual.html#lua_close)
  pub fn lua_close(state: LuaState);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_newthread`](https://www.lua.org/manual/5.1/manual.html#lua_newthread)
  pub fn lua_newthread(state: LuaState) -> Option<LuaState>;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_atpanic`](https://www.lua.org/manual/5.1/manual.html#lua_atpanic)
  pub fn lua_atpanic(state: LuaState, panic_func: LuaCFunction) -> LuaCFunction;
}

// basic stack manipulation ------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_gettop`](https://www.lua.org/manual/5.1/manual.html#lua_gettop)
  pub fn lua_gettop(state: LuaState) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_settop`](https://www.lua.org/manual/5.1/manual.html#lua_settop)
  pub fn lua_settop(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushvalue`](https://www.lua.org/manual/5.1/manual.html#lua_pushvalue)
  pub fn lua_pushvalue(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_remove`](https://www.lua.org/manual/5.1/manual.html#lua_remove)
  pub fn lua_remove(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_insert`](https://www.lua.org/manual/5.1/manual.html#lua_insert)
  pub fn lua_insert(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_replace`](https://www.lua.org/manual/5.1/manual.html#lua_replace)
  pub fn lua_replace(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_checkstack`](https://www.lua.org/manual/5.1/manual.html#lua_checkstack)
  pub fn lua_checkstack(state: LuaState, n: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_xmove`](https://www.lua.org/manual/5.1/manual.html#lua_xmove)
  pub fn lua_xmove(from: LuaState, to: LuaState, n: c_int);
}

// access functions (stack -> C) ----------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_isnumber`](https://www.lua.org/manual/5.1/manual.html#lua_isnumber)
  pub fn lua_isnumber(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_isstring`](https://www.lua.org/manual/5.1/manual.html#lua_isstring)
  pub fn lua_isstring(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_iscfunction`](https://www.lua.org/manual/5.1/manual.html#lua_iscfunction)
  pub fn lua_iscfunction(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_isuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_isuserdata)
  pub fn lua_isuserdata(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_type`](https://www.lua.org/manual/5.1/manual.html#lua_type)
  pub fn lua_type(state: LuaState, idx: c_int) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_typename`](https://www.lua.org/manual/5.1/manual.html#lua_typename)
  pub fn lua_typename(state: LuaState, tp: c_int) -> *const c_char;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_equal`](https://www.lua.org/manual/5.1/manual.html#lua_equal)
  pub fn lua_equal(state: LuaState, idx1: c_int, idx2: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_rawequal`](https://www.lua.org/manual/5.1/manual.html#lua_rawequal)
  pub fn lua_rawequal(state: LuaState, idx1: c_int, idx2: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_lessthan`](https://www.lua.org/manual/5.1/manual.html#lua_lessthan)
  pub fn lua_lessthan(state: LuaState, idx1: c_int, idx2: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_tonumber`](https://www.lua.org/manual/5.1/manual.html#lua_tonumber)
  pub fn lua_tonumber(state: LuaState, idx: c_int) -> c_double;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_tointeger`](https://www.lua.org/manual/5.1/manual.html#lua_tointeger)
  pub fn lua_tointeger(state: LuaState, idx: c_int) -> ptrdiff_t;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_toboolean`](https://www.lua.org/manual/5.1/manual.html#lua_toboolean)
  pub fn lua_toboolean(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_tolstring`](https://www.lua.org/manual/5.1/manual.html#lua_tolstring)
  pub fn lua_tolstring(state: LuaState, idx: c_int, len: *mut size_t) -> *const c_char;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_objlen`](https://www.lua.org/manual/5.1/manual.html#lua_objlen)
  pub fn lua_objlen(state: LuaState, idx: c_int) -> size_t;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_tocfunction`](https://www.lua.org/manual/5.1/manual.html#lua_tocfunction)
  pub fn lua_tocfunction(state: LuaState, idx: c_int) -> Option<LuaCFunction>;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_touserdata`](https://www.lua.org/manual/5.1/manual.html#lua_touserdata)
  pub fn lua_touserdata(state: LuaState, idx: c_int) -> *mut c_void;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_tothread`](https://www.lua.org/manual/5.1/manual.html#lua_tothread)
  pub fn lua_tothread(state: LuaState, idx: c_int) -> Option<LuaState>;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_topointer`](https://www.lua.org/manual/5.1/manual.html#lua_topointer)
  pub fn lua_topointer(state: LuaState, idx: c_int) -> *mut c_void;
}

// push functions (C -> stack) -------------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushnil`](https://www.lua.org/manual/5.1/manual.html#lua_pushnil)
  pub fn lua_pushnil(state: LuaState);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushnumber`](https://www.lua.org/manual/5.1/manual.html#lua_pushnumber)
  pub fn lua_pushnumber(state: LuaState, num: c_double);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushinteger`](https://www.lua.org/manual/5.1/manual.html#lua_pushinteger)
  pub fn lua_pushinteger(state: LuaState, num: ptrdiff_t);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushlstring`](https://www.lua.org/manual/5.1/manual.html#lua_pushlstring)
  pub fn lua_pushlstring(state: LuaState, ptr: *const c_char, len: size_t);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushstring`](https://www.lua.org/manual/5.1/manual.html#lua_pushstring)
  pub fn lua_pushstring(state: LuaState, ptr: *const c_char);
}

// skipping lua_pushvfstring and lua_pushfstring

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushcclosure`](https://www.lua.org/manual/5.1/manual.html#lua_pushcclosure)
  pub fn lua_pushcclosure(state: LuaState, func: LuaCFunction, nvalues: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushboolean`](https://www.lua.org/manual/5.1/manual.html#lua_pushboolean)
  pub fn lua_pushboolean(state: LuaState, boolean: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushlightuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_pushlightuserdata)
  pub fn lua_pushlightuserdata(state: LuaState, ptr: *mut c_void);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pushthread`](https://www.lua.org/manual/5.1/manual.html#lua_pushthread)
  pub fn lua_pushthread(state: LuaState) -> bool;
}

// get functions (Lua -> stack) ----------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_gettable`](https://www.lua.org/manual/5.1/manual.html#lua_gettable)
  pub fn lua_gettable(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_getfield`](https://www.lua.org/manual/5.1/manual.html#lua_getfield)
  pub fn lua_getfield(state: LuaState, idx: c_int, key: *const c_char);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_rawget`](https://www.lua.org/manual/5.1/manual.html#lua_rawget)
  pub fn lua_rawget(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_rawgeti`](https://www.lua.org/manual/5.1/manual.html#lua_rawgeti)
  pub fn lua_rawgeti(state: LuaState, idx: c_int, n: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_createtable`](https://www.lua.org/manual/5.1/manual.html#lua_createtable)
  pub fn lua_createtable(state: LuaState, narr: c_int, nrec: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_newuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_newuserdata)
  pub fn lua_newuserdata(state: LuaState, size: size_t) -> *mut c_void;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_getmetatable`](https://www.lua.org/manual/5.1/manual.html#lua_getmetatable)
  pub fn lua_getmetatable(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_getfenv`](https://www.lua.org/manual/5.1/manual.html#lua_getfenv)
  pub fn lua_getfenv(state: LuaState, idx: c_int);
}

// set functions (stack -> Lua) ---------------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_settable`](https://www.lua.org/manual/5.1/manual.html#lua_settable)
  pub fn lua_settable(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_setfield`](https://www.lua.org/manual/5.1/manual.html#lua_setfield)
  pub fn lua_setfield(state: LuaState, idx: c_int, key: *const c_char);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_rawset`](https://www.lua.org/manual/5.1/manual.html#lua_rawset)
  pub fn lua_rawset(state: LuaState, idx: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_rawseti`](https://www.lua.org/manual/5.1/manual.html#lua_rawseti)
  pub fn lua_rawseti(state: LuaState, idx: c_int, n: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_setmetatable`](https://www.lua.org/manual/5.1/manual.html#lua_setmetatable)
  pub fn lua_setmetatable(state: LuaState, idx: c_int) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_setfenv`](https://www.lua.org/manual/5.1/manual.html#lua_setfenv)
  pub fn lua_setfenv(state: LuaState, idx: c_int) -> bool;
}

// `load' and `call' functions (load and run Lua code) -----------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_call`](https://www.lua.org/manual/5.1/manual.html#lua_call)
  pub fn lua_call(state: LuaState, nargs: c_int, nresults: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_pcall`](https://www.lua.org/manual/5.1/manual.html#lua_pcall)
  pub fn lua_pcall(state: LuaState, nargs: c_int, nresults: c_int, err_func: c_int) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_cpcall`](https://www.lua.org/manual/5.1/manual.html#lua_cpcall)
  pub fn lua_cpcall(state: LuaState, func: LuaCFunction, ud: *mut c_void) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_load`](https://www.lua.org/manual/5.1/manual.html#lua_load)
  pub fn lua_load(state: LuaState, reader: LuaReader, data: *mut c_void, chunkname: *const c_char) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_dump`](https://www.lua.org/manual/5.1/manual.html#lua_dump)
  pub fn lua_dump(state: LuaState, writer: LuaWriter, data: *mut c_void) -> c_int;
}

// coroutine functions ------------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_yield`](https://www.lua.org/manual/5.1/manual.html#lua_yield)
  pub fn lua_yield(state: LuaState, nresults: c_int) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_resume`](https://www.lua.org/manual/5.1/manual.html#lua_resume)
  pub fn lua_resume(state: LuaState, nargs: c_int) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
  pub fn lua_status(state: LuaState) -> c_int;
}

// garbage-collection function and options ---------------------

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

lua_function! {
  /// See the Lua 5.1 manual: [`lua_gc`](https://www.lua.org/manual/5.1/manual.html#lua_gc)
  pub fn lua_gc(state: LuaState, what: c_int, data: c_int) -> c_int;
}

// miscellaneous functions -------------------------------

lua_function! {
  /// See the Lua 5.1 manual: [`lua_error`](https://www.lua.org/manual/5.1/manual.html#lua_error)
  pub fn lua_error(state: LuaState) -> c_int;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_next`](https://www.lua.org/manual/5.1/manual.html#lua_next)
  pub fn lua_next(state: LuaState, idx: c_int) -> bool;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_concat`](https://www.lua.org/manual/5.1/manual.html#lua_concat)
  pub fn lua_concat(state: LuaState, n: c_int);
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_getallocf`](https://www.lua.org/manual/5.1/manual.html#lua_getallocf)
  pub fn lua_getallocf(state: LuaState, ud: *mut *mut c_void) -> LuaAlloc;
}

lua_function! {
  /// See the Lua 5.1 manual: [`lua_setallocf`](https://www.lua.org/manual/5.1/manual.html#lua_setallocf)
  pub fn lua_setallocf(state: LuaState, alloc_func: LuaAlloc, ud: *mut c_void);
}

// some useful macros --------------------------------------

/// See the Lua 5.1 manual: [`lua_pop`](https://www.lua.org/manual/5.1/manual.html#lua_pop)
pub unsafe fn lua_pop(state: LuaState, n: c_int) { lua_settop(state, -n-1); }		

/// See the Lua 5.1 manual: [`lua_newtable`](https://www.lua.org/manual/5.1/manual.html#lua_newtable)
pub unsafe fn lua_newtable(state: LuaState) { lua_createtable(state, 0, 0); }

/// See the Lua 5.1 manual: [`lua_register`](https://www.lua.org/manual/5.1/manual.html#lua_register)
pub unsafe fn lua_register(state: LuaState, name: *const c_char, func: LuaCFunction) {
  lua_pushcfunction(state, func);
  lua_setglobal(state, name);
}

/// See the Lua 5.1 manual: [`lua_pushcfunction`](https://www.lua.org/manual/5.1/manual.html#lua_pushcfunction)
pub unsafe fn lua_pushcfunction(state: LuaState, func: LuaCFunction){ lua_pushcclosure(state, func, 0); }

/// See the Lua 5.1 manual: [`lua_isfunction`](https://www.lua.org/manual/5.1/manual.html#lua_isfunction)
pub unsafe fn lua_isfunction(state: LuaState, idx: c_int) -> bool	{ lua_type(state, idx) == LUA_TFUNCTION }

/// See the Lua 5.1 manual: [`lua_istable`](https://www.lua.org/manual/5.1/manual.html#lua_istable)
pub unsafe fn lua_istable(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TTABLE }

/// See the Lua 5.1 manual: [`lua_islightuserdata`](https://www.lua.org/manual/5.1/manual.html#lua_islightuserdata)
pub unsafe fn lua_islightuserdata(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TLIGHTUSERDATA }

/// See the Lua 5.1 manual: [`lua_isnil`](https://www.lua.org/manual/5.1/manual.html#lua_isnil)
pub unsafe fn lua_isnil(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TNIL }

/// See the Lua 5.1 manual: [`lua_isboolean`](https://www.lua.org/manual/5.1/manual.html#lua_isboolean)
pub unsafe fn lua_isboolean(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TBOOLEAN }

/// See the Lua 5.1 manual: [`lua_isthread`](https://www.lua.org/manual/5.1/manual.html#lua_isthread)
pub unsafe fn lua_isthread(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TTHREAD }

/// See the Lua 5.1 manual: [`lua_isnone`](https://www.lua.org/manual/5.1/manual.html#lua_isnone)
pub unsafe fn lua_isnone(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) == LUA_TNONE }

/// See the Lua 5.1 manual: [`lua_isnoneornil`](https://www.lua.org/manual/5.1/manual.html#lua_isnoneornil)
pub unsafe fn lua_isnoneornil(state: LuaState, idx: c_int) -> bool { lua_type(state, idx) <= 0 }

// skipping lua_pushliteral

/// See the Lua 5.1 manual: [`lua_setglobal`](https://www.lua.org/manual/5.1/manual.html#lua_setglobal)
pub unsafe fn lua_setglobal(state: LuaState, name: *const c_char)	{ lua_setfield(state, LUA_GLOBALSINDEX, name) }

/// See the Lua 5.1 manual: [`lua_getglobal`](https://www.lua.org/manual/5.1/manual.html#lua_getglobal)
pub unsafe fn lua_getglobal(state: LuaState, name: *const c_char)	{ lua_getfield(state, LUA_GLOBALSINDEX, name) }

/// See the Lua 5.1 manual: [`lua_tostring`](https://www.lua.org/manual/5.1/manual.html#lua_tostring)
pub unsafe fn lua_tostring(state: LuaState, idx: c_int) -> *const c_char	{ lua_tolstring(state, idx, ::std::ptr::null_mut()) }