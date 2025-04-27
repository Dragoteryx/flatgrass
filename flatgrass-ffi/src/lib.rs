#![forbid(improper_ctypes_definitions)]
#![allow(non_camel_case_types)]

//! Flatgrass FFI docs here

#[doc(no_inline)]
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
