#![cfg_attr(fg_nightly, feature(doc_cfg))]
#![warn(clippy::use_self)]

// Re-export the ffi module.
#[doc(no_inline)]
pub use flatgrass_ffi as ffi;

// Re-export the macros used to define functions.
#[cfg(feature = "macros")]
pub use flatgrass_macros::{entry, exit, function};

/// Safe abstraction over the Lua C API.
pub mod lua;

/// Garry's Mod types and libraries.
pub mod gm;

/// Asynchronous programming utilities.
#[cfg(feature = "async")]
pub mod task;

/// Re-exports commonly used macros and types.
pub mod prelude {
	#[doc(no_inline)]
	pub use crate::gm::printfg;
	#[doc(no_inline)]
	pub use crate::lua::Lua;
	#[doc(no_inline)]
	pub use crate::lua::util::{Tuple, Upvalue, Yield};
	#[doc(no_inline)]
	pub use crate::lua::{Coroutine, FromLua, Function, Table, ToLua, Userdata, Value};
	#[doc(no_inline)]
	pub use crate::lua::{call, cfunction, resume, table};
	#[doc(no_inline)]
	pub use crate::{entry, exit, function};
}
