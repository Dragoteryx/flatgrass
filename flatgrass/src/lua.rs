use crate::ffi;
use std::any::{Any, TypeId};
use std::cell::{Cell, OnceCell, Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
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
use stack::Stack;

pub mod traits;
use traits::ToLua;

pub mod value;
use value::LuaValue;

pub mod errors;

thread_local! {
	static LUA: Lua = Lua {
		ptr: Cell::new(null_mut()),
		states: OnceCell::default(),
	};
}

/// Safe abstraction over the Lua C API.
#[derive(Debug)]
pub struct Lua {
	ptr: Cell<*mut ffi::lua_State>,
	states: OnceCell<HashMap<TypeId, RefCell<Box<dyn Any>>>>,
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
	pub fn stack(&self) -> Stack<'_> {
		Stack { lua: self }
	}

	pub fn init_states(&self, init: impl FnOnce(&mut StateInitializer<'_>)) -> bool {
		match self.states.get().is_some() {
			true => false,
			false => {
				let mut states = HashMap::new();
				init(&mut StateInitializer { states: &mut states });
				let _ = self.states.set(states);
				true
			}
		}
	}

	pub fn state<T: 'static>(&self) -> Option<State<'_, T>> {
		self.states.get().and_then(|states| {
			let type_id = TypeId::of::<T>();
			states.get(&type_id).and_then(|cell| {
				let borrow = cell.try_borrow_mut().ok()?;
				Some(State {
					inner: RefMut::map(borrow, |value| value.downcast_mut::<T>().unwrap()),
				})
			})
		})
	}

	pub fn state_ref<T: 'static>(&self) -> Option<StateRef<'_, T>> {
		self.states.get().and_then(|states| {
			let type_id = TypeId::of::<T>();
			states.get(&type_id).and_then(|cell| {
				let borrow = cell.try_borrow().ok()?;
				Some(StateRef {
					inner: Ref::map(borrow, |value| value.downcast_ref::<T>().unwrap()),
				})
			})
		})
	}

	/// Forces garbage collection.
	pub fn collect_gc(&self) {
		unsafe {
			ffi::lua_gc(self.to_ptr(), ffi::LUA_GCCOLLECT, 0);
		}
	}

	/// Restarts the garbage collector.
	pub fn restart_gc(&self) {
		unsafe {
			ffi::lua_gc(self.to_ptr(), ffi::LUA_GCRESTART, 0);
		}
	}

	/// Stops the garbage collector.
	pub fn stop_gc(&self) {
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

#[repr(transparent)]
#[derive(Debug)]
pub struct StateInitializer<'l> {
	states: &'l mut HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl StateInitializer<'_> {
	pub fn init<T: 'static>(&mut self, value: T) {
		self.states.insert(TypeId::of::<T>(), RefCell::new(Box::new(value)));
	}
}

#[repr(transparent)]
#[derive(Debug)]
pub struct State<'l, T> {
	inner: RefMut<'l, T>,
}

impl<T> Deref for State<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for State<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

#[repr(transparent)]
#[derive(Debug)]
pub struct StateRef<'l, T> {
	inner: Ref<'l, T>,
}

impl<T> Deref for StateRef<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
