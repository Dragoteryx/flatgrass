use crate::ffi;
use std::cell::Cell;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::process::abort;
use std::ptr::null_mut;

#[cfg(feature = "async")]
use crate::task::AsyncRuntime;

/// Panics with a stack overflow message.
macro_rules! stack_overflow {
	() => {
		panic!("stack overflow")
	};
}

#[doc(inline)]
pub use crate::{call, cfunction, resume, table};
mod macros;

/// Serialization and deserialization support for Lua values.
#[cfg(feature = "serde")]
mod serde;

mod stack;
pub use stack::*;

mod value;
pub use value::*;

mod traits;
pub use traits::*;

pub mod util;

pub mod error;

thread_local! {
	static LUA: Lua = Lua {
		ptr: Cell::new(null_mut()),
		#[cfg(feature = "async")]
		runtime: AsyncRuntime::new(),
	};
}

/// Safe abstraction over the Lua C API.
#[derive(Debug)]
pub struct Lua {
	ptr: Cell<*mut ffi::lua_State>,
	#[cfg(feature = "async")]
	runtime: AsyncRuntime,
}

impl Lua {
	/// Enters a new Lua context and executes the provided closure.\
	/// Lua objects can be created and Lua functions can be called within the scope of the closure.\
	/// The previous Lua context is restored after the closure returns.
	///
	/// # Safety
	///
	/// The Lua state passed as an argument must be valid.\
	/// Created Lua objects contained by the closure must be used within the same Lua context as the one they were created in.\
	/// Using the Lua state provided by GMod guarantees that.
	///
	/// # Panics
	///
	/// Will abort the process if the closure panics to prevent unwinding through the FFI boundary.
	pub unsafe fn enter<T>(ptr: *mut ffi::lua_State, func: impl FnOnce(&Self) -> T) -> T {
		LUA.with(|lua| {
			#[cfg(feature = "tokio")]
			let _guard = lua.runtime.tokio_handle().enter();
			let old_ptr = lua.ptr.replace(ptr);
			let func = AssertUnwindSafe(|| func(lua));
			match catch_unwind(func) {
				Err(_) => abort(),
				Ok(value) => {
					lua.ptr.set(old_ptr);
					value
				}
			}
		})
	}

	/// Tries to get the current Lua state.
	pub fn try_get<T>(func: impl FnOnce(Option<&Self>) -> T) -> T {
		LUA.with(|lua| match lua.ptr.get().is_null() {
			false => func(Some(lua)),
			true => func(None),
		})
	}

	/// Gets the current Lua state.
	///
	/// # Panics
	///
	/// Panics if the Lua state is not valid.
	pub fn get<T>(func: impl FnOnce(&Self) -> T) -> T {
		Self::try_get(|lua| func(lua.expect("uninitialized Lua state")))
	}

	/// Checks if the Lua state is valid.
	pub fn is_valid() -> bool {
		Lua::try_get(|lua| lua.is_some())
	}

	/// The associated raw Lua state.
	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		self.ptr.get()
	}

	/// The associated Lua stack.
	pub fn stack(&self) -> Stack<'_> {
		unsafe { Stack::new(self.to_ptr()) }
	}

	#[cfg(feature = "async")]
	pub fn async_runtime(&self) -> &AsyncRuntime {
		&self.runtime
	}

	/// Checks if two values are equal according to Lua semantics.
	pub fn equals<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Result<bool, Value> {
		static EQUALS: ffi::lua_CFunction = ffi::raw_function!(|state| unsafe {
			let res = ffi::lua_equal(state, -1, -2);
			ffi::lua_pushboolean(state, res);
			1
		});

		let stack = self.stack();
		stack.push_c_function(EQUALS);
		stack.push_any(a);
		stack.push_any(b);

		unsafe {
			match ffi::lua_pcall(self.to_ptr(), 2, 1, 0) {
				0 => Ok(stack.pop_bool_unchecked()),
				_ => Err(stack.pop_value_unchecked()),
			}
		}
	}

	/// Checks if the first value is less than the second value according to Lua semantics.
	pub fn less_than<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Result<bool, Value> {
		static LESS_THAN: ffi::lua_CFunction = ffi::raw_function!(|state| unsafe {
			let res = ffi::lua_lessthan(state, -1, -2);
			ffi::lua_pushboolean(state, res);
			1
		});

		let stack = self.stack();
		stack.push_c_function(LESS_THAN);
		stack.push_any(a);
		stack.push_any(b);

		unsafe {
			match ffi::lua_pcall(self.to_ptr(), 2, 1, 0) {
				0 => Ok(stack.pop_bool_unchecked()),
				_ => Err(stack.pop_value_unchecked()),
			}
		}
	}

	#[doc(hidden)]
	pub fn __fg_entry(&self) {
		#[cfg(feature = "async")]
		if let Value::Table(timer) = value::Table::globals().raw_get("timer") {
			if let Value::Function(timer_create) = timer.raw_get("Create") {
				static TICK: ffi::lua_CFunction = ffi::raw_function!(|state| unsafe {
					Lua::enter(state, |lua| lua.runtime.tick());
					0
				});

				let id = format!("__fg_poll_{:p}", self);
				let _ = call!(timer_create, id, 0.0, 0.0, TICK);
			}
		}
	}

	#[doc(hidden)]
	pub fn __fg_exit(&self) {
		#[cfg(feature = "async")]
		self.runtime.shutdown();
	}
}
