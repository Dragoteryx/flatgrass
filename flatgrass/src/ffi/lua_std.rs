use super::*;

/// See the Lua 5.1 manual: [`lua_Alloc`](https://www.lua.org/manual/5.1/manual.html#lua_Alloc)
pub type LuaAlloc = unsafe extern "C-unwind" fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void;

/// See the Lua 5.1 manual: [`lua_CFunction`](https://www.lua.org/manual/5.1/manual.html#lua_CFunction)
pub type LuaCFunction = unsafe extern "C-unwind" fn(state: LuaState) -> c_int;

/// See the Lua 5.1 manual: [`lua_Reader`](https://www.lua.org/manual/5.1/manual.html#lua_Reader)
pub type LuaReader = unsafe extern "C-unwind" fn(state: LuaState, data: *mut c_void, size: *mut size_t) -> *const c_char;

/// See the Lua 5.1 manual: [`lua_Writer`](https://www.lua.org/manual/5.1/manual.html#lua_Writer)
pub type LuaWriter = unsafe extern "C-unwind" fn(state: LuaState, ptr: *const c_void, size: size_t, data: *mut c_void) -> c_int;

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

/// See the Lua 5.1 manual: [`Stack Size`](https://www.lua.org/manual/5.1/manual.html#3.2)
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

impl LuaState {
  fetch_lua!(fn lua_atpanic(self, panic_func: LuaCFunction) -> LuaCFunction);
  fetch_lua!(fn lua_call(self, nargs: c_int, nresults: c_int));
  fetch_lua!(fn lua_checkstack(self, size: c_int) -> c_int);
  fetch_lua!(fn lua_close(self));
  fetch_lua!(fn lua_concat(self, n: c_int));
  fetch_lua!(fn lua_cpcall(self, func: LuaCFunction, ud: *mut c_void) -> c_int);
  fetch_lua!(fn lua_createtable(self, narr: c_int, nrec: c_int));
  fetch_lua!(fn lua_dump(self, writer: LuaWriter, data: *mut c_void) -> c_int);
  fetch_lua!(fn lua_equal(self, idx1: c_int, idx2: c_int) -> c_int);
  fetch_lua!(fn lua_error(self) -> c_int);
  fetch_lua!(fn lua_gc(self, what: c_int, data: c_int) -> c_int);
  fetch_lua!(fn lua_getallocf(self, ud: *mut *mut c_void) -> LuaAlloc);
  fetch_lua!(fn lua_getfenv(self, idx: c_int));
  fetch_lua!(fn lua_getfield(self, idx: c_int, key: *const c_char));
  fetch_lua!(macro lua_getglobal(self, name: *const c_char) { self.lua_getfield(LUA_GLOBALSINDEX, name); });
  fetch_lua!(fn lua_getmetatable(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_gettable(self, idx: c_int));
  fetch_lua!(fn lua_gettop(self) -> c_int);
  fetch_lua!(fn lua_insert(self, idx: c_int));
  fetch_lua!(macro lua_isboolean(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TBOOLEAN { 1 } else { 0 } });
  fetch_lua!(fn lua_iscfunction(self, idx: c_int) -> c_int);
  fetch_lua!(macro lua_isfunction(self, idx: c_int) -> c_int	{ if self.lua_type(idx) == LUA_TFUNCTION { 1 } else { 0 } });
  fetch_lua!(macro lua_islightuserdata(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TLIGHTUSERDATA { 1 } else { 0 } });
  fetch_lua!(macro lua_isnil(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TNIL { 1 } else { 0 } });
  fetch_lua!(macro lua_isnone(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TNONE { 1 } else { 0 } });
  fetch_lua!(macro lua_isnoneornil(self, idx: c_int) -> c_int { if self.lua_type(idx) <= 0 { 1 } else { 0 } });
  fetch_lua!(fn lua_isnumber(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_isstring(self, idx: c_int) -> c_int);
  fetch_lua!(macro lua_istable(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TTABLE { 1 } else { 0 } });
  fetch_lua!(macro lua_isthread(self, idx: c_int) -> c_int { if self.lua_type(idx) == LUA_TTHREAD { 1 } else { 0 } });
  fetch_lua!(fn lua_isuserdata(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_lessthan(self, idx1: c_int, idx2: c_int) -> c_int);
  fetch_lua!(fn lua_load(self, reader: LuaReader, data: *mut c_void, chunkname: *const c_char) -> c_int);
  fetch_lua!(fn lua_newstate(alloc_func: LuaAlloc, ud: *mut c_void) -> Option<Self>);
  fetch_lua!(macro lua_newtable(self) { self.lua_createtable(0, 0); });
  fetch_lua!(fn lua_newthread(self) -> Option<Self>);
  fetch_lua!(fn lua_newuserdata(self, size: size_t) -> *mut c_void);
  fetch_lua!(fn lua_next(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_objlen(self, idx: c_int) -> size_t);
  fetch_lua!(fn lua_pcall(self, nargs: c_int, nresults: c_int, err_func: c_int) -> c_int);
  fetch_lua!(macro lua_pop(self, n: c_int) { self.lua_settop(-n-1); });
  fetch_lua!(fn lua_pushboolean(self, boolean: c_int));
  fetch_lua!(fn lua_pushcclosure(self, func: LuaCFunction, nvalues: c_int));
  fetch_lua!(macro lua_pushcfunction(self, func: LuaCFunction) { self.lua_pushcclosure(func, 0) });
  fetch_lua!(fn lua_pushinteger(self, num: ptrdiff_t));
  fetch_lua!(fn lua_pushlightuserdata(self, ptr: *mut c_void));
  fetch_lua!(fn lua_pushlstring(self, ptr: *const c_char, len: size_t));
  fetch_lua!(fn lua_pushnil(self));
  fetch_lua!(fn lua_pushnumber(self, num: c_double));
  fetch_lua!(fn lua_pushstring(self, ptr: *const c_char));
  fetch_lua!(fn lua_pushthread(self) -> c_int);
  fetch_lua!(fn lua_pushvalue(self, idx: c_int));
  fetch_lua!(fn lua_rawequal(self, idx1: c_int, idx2: c_int) -> c_int);
  fetch_lua!(fn lua_rawget(self, idx: c_int));
  fetch_lua!(fn lua_rawgeti(self, idx: c_int, n: c_int));
  fetch_lua!(fn lua_rawset(self, idx: c_int));
  fetch_lua!(fn lua_rawseti(self, idx: c_int, n: c_int));
  fetch_lua!(macro lua_register(self, name: *const c_char, func: LuaCFunction) { self.lua_pushcfunction(func); self.lua_setglobal(name); });
  fetch_lua!(fn lua_remove(self, idx: c_int));
  fetch_lua!(fn lua_replace(self, idx: c_int));
  fetch_lua!(fn lua_resume(self, nargs: c_int) -> c_int);
  fetch_lua!(fn lua_setallocf(self, alloc_func: LuaAlloc, ud: *mut c_void));
  fetch_lua!(fn lua_setfenv(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_setfield(self, idx: c_int, key: *const c_char));
  fetch_lua!(macro lua_setglobal(self, name: *const c_char) { self.lua_setfield(LUA_GLOBALSINDEX, name); });
  fetch_lua!(fn lua_setmetatable(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_settable(self, idx: c_int));
  fetch_lua!(fn lua_settop(self, idx: c_int));
  fetch_lua!(fn lua_status(self) -> c_int);
  fetch_lua!(fn lua_toboolean(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_tocfunction(self, idx: c_int) -> Option<LuaCFunction>);
  fetch_lua!(fn lua_tointeger(self, idx: c_int) -> ptrdiff_t);
  fetch_lua!(fn lua_tolstring(self, idx: c_int, len: *mut size_t) -> *const c_char);
  fetch_lua!(fn lua_tonumber(self, idx: c_int) -> c_double);
  fetch_lua!(fn lua_topointer(self, idx: c_int) -> *const c_void);
  fetch_lua!(macro lua_tostring(self, idx: c_int) -> *const c_char	{ self.lua_tolstring(idx, ::std::ptr::null_mut()) });
  fetch_lua!(fn lua_tothread(self, idx: c_int) -> Option<Self>);
  fetch_lua!(fn lua_touserdata(self, idx: c_int) -> *mut c_void);
  fetch_lua!(fn lua_type(self, idx: c_int) -> c_int);
  fetch_lua!(fn lua_typename(self, tp: c_int) -> *const c_char);
  fetch_lua!(fn lua_xmove(self, to: Self, n: c_int));
  fetch_lua!(fn lua_yield(self, nresults: c_int) -> c_int);
}