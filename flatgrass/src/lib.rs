#![cfg_attr(fg_nightly, feature(doc_notable_trait))]
#![cfg_attr(fg_nightly, feature(doc_auto_cfg))]
#![warn(clippy::use_self)]

#[cfg(feature = "macros")]
pub use flatgrass_macros::{entry, exit, function};

#[doc(inline)]
pub use flatgrass_ffi as ffi;

/// Safe abstraction over the Lua C API, you should probably use this.
pub mod lua;

/// Garry's Mod types and libraries.
pub mod gm;

/// Re-exports commonly used macros and types.
pub mod prelude {
	pub use crate::gm::printfg;
	pub use crate::lua::errors::LuaError;
	#[doc(no_inline)]
	pub use crate::lua::traits::{FromLua, ToLua};
	pub use crate::lua::value::{Function, LuaValue, Table, Userdata};
	pub use crate::lua::{func, table};
	pub use crate::lua::{Lua, LuaStack};
}
