#![cfg_attr(fg_nightly, feature(doc_auto_cfg))]
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
	#[doc(no_inline)]
	pub use crate::gm::printfg;
	#[doc(no_inline)]
	pub use crate::lua::state::{State, StateRef};
	#[doc(no_inline)]
	pub use crate::lua::traits::{FromLua, ToLua};
	#[doc(no_inline)]
	pub use crate::lua::value::{Coroutine, Function, LuaString, LuaValue, Table, Userdata};
	#[doc(no_inline)]
	pub use crate::lua::{Lua, LuaError};
	#[doc(no_inline)]
	pub use crate::lua::{cfunction, table};
}
