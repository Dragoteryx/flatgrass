use super::{FromLua, LuaType, LuaValue, ToLua, ToLuaIter};
use crate::ffi;
use crate::lua::errors::FromLuaError;
use crate::lua::{Lua, LuaStack};
use std::cell::UnsafeCell;
use std::marker::PhantomData;

mod string;
pub use string::LuaString;

mod function;
pub use function::Function;

pub mod userdata;
#[doc(no_inline)]
pub use userdata::Userdata;

pub mod table;
#[doc(no_inline)]
pub use table::Table;

pub mod coroutine;
#[doc(no_inline)]
pub use coroutine::Coroutine;

#[repr(transparent)]
struct Reference {
	not_ref_unwind_safe: PhantomData<UnsafeCell<()>>,
	not_unwind_safe: PhantomData<&'static mut ()>,
	not_send_sync: PhantomData<*mut ()>,
	id: i32,
}

impl LuaStack {
	#[track_caller]
	fn push_reference(&self, reference: &Reference) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_rawgeti(self.state(), ffi::LUA_REGISTRYINDEX, reference.id);
			}
		} else {
			stack_overflow!();
		}
	}

	unsafe fn pop_reference_unchecked(&self) -> Reference {
		Reference {
			id: unsafe { ffi::luaL_ref(self.state(), ffi::LUA_REGISTRYINDEX) },
			not_ref_unwind_safe: PhantomData,
			not_unwind_safe: PhantomData,
			not_send_sync: PhantomData,
		}
	}

	unsafe fn get_reference_unchecked(&self, idx: i32) -> Reference {
		unsafe {
			self.push_index_unchecked(idx);
			self.pop_reference_unchecked()
		}
	}
}

impl Clone for Reference {
	fn clone(&self) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_reference(self);
			stack.pop_reference_unchecked()
		})
	}
}

impl Drop for Reference {
	fn drop(&mut self) {
		Lua::try_get(|lua| unsafe {
			if let Some(lua) = lua {
				ffi::luaL_unref(lua.state(), ffi::LUA_REGISTRYINDEX, self.id);
			}
		});
	}
}
