#![cfg_attr(docsrs, feature(doc_notable_trait))]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(clippy::use_self)]

// Re-export the ffi module.
#[doc(no_inline)]
pub use flatgrass_ffi as ffi;

// Import the macros used to define functions.
#[cfg(feature = "macros")]
pub use flatgrass_macros::{entry, exit, function};

/// Safe abstraction over the Lua C API, you should probably use this.
pub mod lua;

/// Garry's Mod types and libraries.
pub mod gm;

/// Re-exports commonly used macros and types.
pub mod prelude {
	pub use crate::gm::printfg;
	pub use crate::lua::errors::LuaError;
	pub use crate::lua::traits::{FromLua, ToLua};
	pub use crate::lua::value::{Function, LuaString, LuaValue, Table, Userdata};
	pub use crate::lua::{Lua, LuaStack};
	pub use crate::lua::{func, table};
}
