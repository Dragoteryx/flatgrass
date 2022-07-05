use libc::{c_char, c_double, c_int, c_void, ptrdiff_t, size_t};
use std::ptr::NonNull;

pub use crate::cstr;
mod shared; pub use shared::*;
mod state; pub use state::*;
mod debug; pub use debug::*;

macro_rules! fetch_lua {
  (
    fn $name:ident($self:ident $(,$arg:ident: $argty:ty)*) $(-> $ret:ty)?
  ) => {
    #[doc = concat!(" See the Lua 5.1 manual: [`", stringify!($name), "`](https://www.lua.org/manual/5.1/manual.html#", stringify!($name), ")")]
    #[allow(non_snake_case)]
    pub unsafe fn $name($self $(,$arg: $argty)*) $(-> $ret)? {
      LUA_SHARED
        .get::<extern "C-unwind" fn(Self $(,$argty)*) $(-> $ret)?>(stringify!($name).as_bytes())
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