use crate::ffi;
use std::cell::Cell;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::abort;
use std::ptr::{null_mut, NonNull};

/// Panics with a stack overflow message.
macro_rules! stack_overflow {
	() => {
		panic!("stack overflow")
	};
}

#[doc(inline)]
pub use crate::{func, globals, table};
mod macros;

mod stack;
pub use stack::*;

pub mod errors;

pub mod traits;
use traits::ToLua;

pub mod value;

thread_local! {
	static LUA_STATE: Cell<*mut ffi::lua_State> = const {
		Cell::new(null_mut())
	};
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Lua {
	state: NonNull<ffi::lua_State>,
}

impl Lua {
	pub unsafe fn init<T>(state: *mut ffi::lua_State, func: impl FnOnce(&Self) -> T) -> T {
		LUA_STATE.with(|static_state| {
			let old_state = static_state.get();
			static_state.set(state);
			let lua = Self {
				state: NonNull::new_unchecked(state),
			};
			match catch_unwind(AssertUnwindSafe(|| func(&lua))) {
				Err(_) => abort(),
				Ok(res) => {
					static_state.set(old_state);
					res
				}
			}
		})
	}

	pub fn try_get<T>(func: impl FnOnce(Option<&Self>) -> T) -> T {
		LUA_STATE.with(|static_state| {
			let lua = NonNull::new(static_state.get()).map(|state| Self { state });
			func(lua.as_ref())
		})
	}

	pub fn get<T>(func: impl FnOnce(&Self) -> T) -> T {
		Self::try_get(|lua| func(lua.expect("a Lua state")))
	}

	pub fn state(&self) -> *mut ffi::lua_State {
		self.state.as_ptr()
	}

	pub fn stack(&self) -> &LuaStack {
		unsafe { &*(self as *const Self).cast() }
	}

	pub fn collect_gc(&self) {
		unsafe {
			ffi::lua_gc(self.state(), ffi::LUA_GCCOLLECT, 0);
		}
	}

	pub fn restart_gc(&self) {
		unsafe {
			ffi::lua_gc(self.state(), ffi::LUA_GCRESTART, 0);
		}
	}

	pub fn stop_gc(&self) {
		unsafe {
			ffi::lua_gc(self.state(), ffi::LUA_GCSTOP, 0);
		}
	}

	pub fn equals<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Option<bool> {
		static EQUALS: ffi::lua_CFunction = ffi::raw_function!(|state| {
			let res = ffi::lua_equal(state, -1, -2);
			ffi::lua_pushboolean(state, res);
			1
		});

		let stack = self.stack();
		stack.push_c_function(EQUALS);
		stack.push_any(a);
		stack.push_any(b);

		unsafe {
			match ffi::lua_pcall(self.state(), 2, 1, 0) {
				0 => Some(stack.pop_bool_unchecked()),
				_ => {
					stack.pop_n(1);
					None
				}
			}
		}
	}

	pub fn less_than<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Option<bool> {
		static LESS_THAN: ffi::lua_CFunction = ffi::raw_function!(|state| {
			let res = ffi::lua_lessthan(state, -1, -2);
			ffi::lua_pushboolean(state, res);
			1
		});

		let stack = self.stack();
		stack.push_c_function(LESS_THAN);
		stack.push_any(a);
		stack.push_any(b);

		unsafe {
			match ffi::lua_pcall(self.state(), 2, 1, 0) {
				0 => Some(stack.pop_bool_unchecked()),
				_ => {
					stack.pop_n(1);
					None
				}
			}
		}
	}

	#[doc(hidden)]
	pub fn entry(&self) {}

	#[doc(hidden)]
	pub fn exit(&self) {}
}
