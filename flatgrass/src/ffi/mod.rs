// I couldn't have done this without the help provided by https://github.com/danielga/garrysmod_common.
// Thank you.

use libc::{c_char, c_double, c_int, c_void, ptrdiff_t, size_t};
use std::ptr::NonNull;

mod shared; pub use shared::*;
mod types; pub use types::*;

/// See the Lua 5.1 manual: [`lua_call`](https://www.lua.org/manual/5.1/manual.html#lua_call)
pub const LUA_MULTRET: c_int = -1;

/// See the Lua 5.1 manual: [`Registry`](https://www.lua.org/manual/5.1/manual.html#3.5)
pub const LUA_REGISTRYINDEX: c_int = -10000;

/// See the Lua 5.1 manual: [`Pseudo-indices`](https://www.lua.org/manual/5.1/manual.html#3.3)
pub const LUA_ENVIRONINDEX: c_int = -10001;

/// See the Lua 5.1 manual: [`Pseudo-indices`](https://www.lua.org/manual/5.1/manual.html#3.3)
pub const LUA_GLOBALSINDEX: c_int = -10002;

/// See the Lua 5.1 manual: [`C Closures`](https://www.lua.org/manual/5.1/manual.html#3.4)
pub const fn lua_upvalueindex(idx: c_int) -> c_int {
  LUA_GLOBALSINDEX.wrapping_sub(idx)
}

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_YIELD: c_int = 1;

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_ERRRUN: c_int = 2;

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_ERRSYNTAX: c_int = 3;

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_ERRMEM: c_int = 4;

/// See the Lua 5.1 manual: [`lua_status`](https://www.lua.org/manual/5.1/manual.html#lua_status)
pub const LUA_ERRERR: c_int = 5;

/// See the Lua 5.1 manual: [`luaL_load`](https://www.lua.org/manual/5.1/manual.html#luaL_load)
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

macro_rules! lua_function {
  (
    $(#[$outer:meta])*
    $vis:vis fn $name:ident($($arg:ident: $argty:ty),*);
  ) => {
    $(#[$outer])*
    #[allow(non_snake_case)]
    $vis unsafe fn $name($($arg: $argty),*) {
      LUA_SHARED
        .get::<extern "C-unwind" fn($($argty),*)>(stringify!($name).as_bytes())
        .expect(concat!("Could not find ", stringify!($name)))($($arg),*)
    }
  };
  (
    $(#[$outer:meta])*
    $vis:vis fn $name:ident($($arg:ident: $argty:ty),*) -> bool;
  ) => {
    $(#[$outer])*
    #[allow(non_snake_case)]
    $vis unsafe fn $name($($arg: $argty),*) -> bool {
      LUA_SHARED
        .get::<extern "C-unwind" fn($($argty),*) -> c_int>(stringify!($name).as_bytes())
        .expect(concat!("Could not find ", stringify!($name)))($($arg),*) != 0
    }
  };
  (
    $(#[$outer:meta])*
    $vis:vis fn $name:ident($($arg:ident: $argty:ty),*) -> $ret:ty;
  ) => {
    $(#[$outer])*
    #[allow(non_snake_case)]
    $vis unsafe fn $name($($arg: $argty),*) -> $ret {
      LUA_SHARED
        .get::<extern "C-unwind" fn($($argty),*) -> $ret>(stringify!($name).as_bytes())
        .expect(concat!("Could not find ", stringify!($name)))($($arg),*)
    }
  };
}

mod lua_std; pub use lua_std::*;
mod lua_dbg; pub use lua_dbg::*;
mod lua_aux; pub use lua_aux::*;