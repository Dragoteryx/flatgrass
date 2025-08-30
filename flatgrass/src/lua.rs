use crate::ffi;
use std::cell::{Cell, OnceCell};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::process::abort;
use std::ptr::null_mut;

/// Panics with a stack overflow message.
macro_rules! stack_overflow {
	() => {
		panic!("stack overflow")
	};
}

#[doc(inline)]
pub use crate::{func, table};
mod macros;

pub mod stack;
use stack::LuaStack;

pub mod state;
use state::*;

pub mod traits;
use traits::ToLua;

pub mod value;
use value::LuaValue;

mod error;
pub use error::*;

thread_local! {
	static LUA: Lua = Lua {
		ptr: Cell::new(null_mut()),
		state_manager: OnceCell::default(),
	};
}

/// Safe abstraction over the Lua C API.
#[derive(Debug)]
pub struct Lua {
	ptr: Cell<*mut ffi::lua_State>,
	state_manager: OnceCell<StateManager>,
}

impl Lua {
	/// Initializes a new Lua state and runs the given function with it.
	///
	/// # Safety
	///
	/// The Lua state passed as an argument must be valid.
	pub unsafe fn init<T>(ptr: *mut ffi::lua_State, func: impl FnOnce(&Self) -> T) -> T {
		LUA.with(|lua| {
			let old_ptr = lua.ptr.replace(ptr);
			match catch_unwind(AssertUnwindSafe(|| func(lua))) {
				Err(_) => abort(),
				Ok(res) => {
					lua.ptr.set(old_ptr);
					res
				}
			}
		})
	}

	/// Tries to get the current Lua state.
	pub fn try_get<T>(func: impl FnOnce(Option<&Self>) -> T) -> T {
		LUA.with(|lua| {
			if lua.ptr.get().is_null() {
				func(None)
			} else {
				func(Some(lua))
			}
		})
	}

	/// Gets the current Lua state.
	///
	/// # Panics
	///
	/// Panics if the Lua state is not initialized.
	pub fn get<T>(func: impl FnOnce(&Self) -> T) -> T {
		Self::try_get(|lua| func(lua.expect("a Lua state")))
	}

	/// The associated raw Lua state.
	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		self.ptr.get()
	}

	/// The associated Lua stack.
	pub fn stack(&self) -> LuaStack<'_> {
		unsafe {
			LuaStack::new(self.to_ptr())
		}
	}

	pub fn state_manager(&self) -> Option<&StateManager> {
		self.state_manager.get()
	}

	pub fn state<T: 'static>(&self) -> Option<State<'_, T>> {
		self.state_manager().and_then(StateManager::get)
	}

	pub fn state_ref<T: 'static>(&self) -> Option<StateRef<'_, T>> {
		self.state_manager().and_then(StateManager::get_ref)
	}

	pub fn init_state(&self, init: impl FnOnce(&mut StateManager)) -> bool {
		match self.state_manager() {
			Some(_) => false,
			None => {
				let mut state_manager = StateManager::default();
				init(&mut state_manager);
				let _ = self.state_manager.set(state_manager);
				true
			}
		}
	}

	/// Forces garbage collection.
	pub fn gc_collect(&self) {
		unsafe {
			ffi::lua_gc(self.to_ptr(), ffi::LUA_GCCOLLECT, 0);
		}
	}

	/// Restarts the garbage collector.
	pub fn gc_restart(&self) {
		unsafe {
			ffi::lua_gc(self.to_ptr(), ffi::LUA_GCRESTART, 0);
		}
	}

	/// Stops the garbage collector.
	pub fn gc_stop(&self) {
		unsafe {
			ffi::lua_gc(self.to_ptr(), ffi::LUA_GCSTOP, 0);
		}
	}

	/// Checks if two values are equal according to Lua semantics.
	pub fn equals<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Result<bool, LuaValue> {
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
	pub fn less_than<T: ToLua, U: ToLua>(&self, a: T, b: U) -> Result<bool, LuaValue> {
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
	pub fn __fg_entry(&self) {}

	#[doc(hidden)]
	pub fn __fg_exit(&self) {}
}
