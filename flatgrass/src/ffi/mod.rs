use libc::{c_char, c_double, c_int, c_void, ptrdiff_t, size_t};
use std::ptr::NonNull;

pub use crate::cstr;
mod shared; pub use shared::*;
mod state; pub use state::*;
mod types; pub use types::*;

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

macro_rules! fetch_lua {
  (
    fn $name:ident($self:ident $(,$arg:ident: $argty:ty)*) $(-> $ret:ty)?
  ) => {
    #[doc = concat!(" See the Lua 5.1 manual: [`", stringify!($name), "`](https://www.lua.org/manual/5.1/manual.html#", stringify!($name), ")")]
    #[allow(non_snake_case)]
    pub unsafe fn $name($self $(,$arg: $argty)*) $(-> $ret)? {
      LUA_SHARED
        .get::<extern "C-unwind" fn(LuaState $(,$argty)*) $(-> $ret)?>(stringify!($name).as_bytes())
        .expect(concat!("Could not find '", stringify!($name), "'"))($self $(,$arg)*)
    }
  };
  (
    macro $name:ident($self:ident $(,$arg:ident: $argty:ty)*) $(-> $ret:ty)? $body:block
  ) => {
    #[doc = concat!(" See the Lua 5.1 manual: [`", stringify!($name), "`](https://www.lua.org/manual/5.1/manual.html#", stringify!($name), ")")]
    #[allow(non_snake_case)]
    pub unsafe fn $name($self $(,$arg: $argty)*) $(-> $ret)? $body
  };
  (
    fn $name:ident($($arg:ident: $argty:ty),*) $(-> $ret:ty)?
  ) => {
    #[doc = concat!(" See the Lua 5.1 manual: [`", stringify!($name), "`](https://www.lua.org/manual/5.1/manual.html#", stringify!($name), ")")]
    #[allow(non_snake_case)]
    pub unsafe fn $name($($arg: $argty),*) $(-> $ret)? {
      LUA_SHARED
        .get::<extern "C-unwind" fn($($argty),*) $(-> $ret)?>(stringify!($name).as_bytes())
        .expect(concat!("Could not find '", stringify!($name), ))($($arg),*)
    }
  };
  (
    macro $name:ident($($arg:ident: $argty:ty),*) $(-> $ret:ty)? $body:block
  ) => {
    #[doc = concat!(" See the Lua 5.1 manual: [`", stringify!($name), "`](https://www.lua.org/manual/5.1/manual.html#", stringify!($name), ")")]
    #[allow(non_snake_case)]
    pub unsafe fn $name($($arg: $argty),*) $(-> $ret)? $body
  };
}

mod lua_std; pub use lua_std::*;
mod lua_dbg; pub use lua_dbg::*;
mod lua_aux; pub use lua_aux::*;