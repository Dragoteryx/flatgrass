#![forbid(improper_ctypes_definitions)]
#![allow(non_camel_case_types)]

//! FFI bindings to the Lua C API, as well as type definitions.
//!
//! For more information, see [the Lua 5.1 manual](https://www.lua.org/manual/5.1/manual.html#3).

pub use libc;
use libc::*;

mod macros;

mod shared;
pub use shared::*;

mod lua_std;
pub use lua_std::*;

mod lua_dbg;
pub use lua_dbg::*;

mod lua_aux;
pub use lua_aux::*;
