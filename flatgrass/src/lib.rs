#![deny(clippy::cast_possible_truncation)]
#![warn(clippy::use_self)]
#![feature(c_unwind)]

extern crate self as flatgrass;
pub use flatgrass_macros::{
  entry, exit, function
};

/// Contains the FFI bindings for the Lua C API, as well as type definitions.
/// 
/// For more information, see [the Lua 5.1 manual](https://www.lua.org/manual/5.1/manual.html#3).
pub mod ffi;

/// Safe abstraction over the Lua C API, you should probably use this.
pub mod lua;

/// Garry's Mod specific types. (Angles and Vectors)
pub mod gmod;

pub mod prelude {
  pub use crate::lua::*;
  pub use crate::lua::value::*;
  
  pub use crate::ffi::LuaState;
  pub use crate::ffi::LuaCFunction;

  pub use crate::printfg;
  pub use crate::errorfg;
}

mod macros;