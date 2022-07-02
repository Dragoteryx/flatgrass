#![deny(clippy::cast_possible_truncation)]
#![allow(clippy::missing_safety_doc)]
#![warn(clippy::use_self)]
#![feature(c_unwind)]

pub use flatgrass_macros::*;

/// Contains the FFI bindings for the Lua C API, as well as type definitions.
/// 
/// For more information, see [the Lua 5.1 manual](https://www.lua.org/manual/5.1/manual.html#3).
pub mod ffi;

/// Safe abstraction over the Lua C API, you should probably use this.
pub mod lua;

/// Garry's Mod specific types. (Angles and Vectors)
pub mod gmod;

pub mod prelude {
  pub use crate::lua::Lua;
  pub use crate::lua::LuaGc;
  pub use crate::lua::LuaRealm;
  pub use crate::lua::value::LuaValue;

  pub use crate::cstr;
  pub use crate::table;
  pub use crate::printfg;
  pub use crate::errorfg;
}

mod macros;