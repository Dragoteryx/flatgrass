use super::*;

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LuaState(NonNull<c_void>);

/// See the Lua 5.1 manual: [`lua_Alloc`](https://www.lua.org/manual/5.1/manual.html#lua_Alloc)
pub type LuaAlloc = unsafe extern "C-unwind" fn(ud: *mut c_void, ptr: *mut c_void, osize: size_t, nsize: size_t) -> *mut c_void;

/// See the Lua 5.1 manual: [`lua_CFunction`](https://www.lua.org/manual/5.1/manual.html#lua_CFunction)
pub type LuaCFunction = unsafe extern "C-unwind" fn(state: LuaState) -> c_int;

/// See the Lua 5.1 manual: [`lua_Reader`](https://www.lua.org/manual/5.1/manual.html#lua_Reader)
pub type LuaReader = unsafe extern "C-unwind" fn(state: LuaState, data: *mut c_void, size: *mut size_t) -> *const c_char;

/// See the Lua 5.1 manual: [`lua_Writer`](https://www.lua.org/manual/5.1/manual.html#lua_Writer)
pub type LuaWriter = unsafe extern "C-unwind" fn(state: LuaState, ptr: *const c_void, size: size_t, data: *mut c_void) -> c_int;

/// See the Lua 5.1 manual: [`luaL_Reg`](https://www.lua.org/manual/5.1/manual.html#luaL_Reg)
#[repr(C)]
pub struct LuaLReg {
  pub name: *const c_char,
  pub func: LuaCFunction
}