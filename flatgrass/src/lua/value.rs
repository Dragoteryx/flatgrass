use crate::ffi;
use crate::lua::Lua;
use crate::lua::stack::Stack;
use crate::lua::traits::{FromLua, FromLuaError, ToLua};
use std::cell::UnsafeCell;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::mem::forget;

#[cfg(feature = "serde")]
mod serde;

mod tuple;
pub use tuple::*;

pub mod coroutine;
pub use coroutine::Coroutine;

pub mod function;
pub use function::Function;

pub mod string;
pub use string::LuaString;

pub mod table;
pub use table::Table;

pub mod userdata;
pub use userdata::{LightUserdata, Userdata};

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub enum Value {
	#[default]
	Nil,
	Bool(bool),
	Number(f64),
	String(LuaString),
	Table(Table),
	Function(Function),
	Userdata(Userdata),
	Coroutine(Coroutine),
	LightUserdata(LightUserdata),
}

impl Value {
	pub const fn get_type(&self) -> Type {
		match self {
			Self::Nil => Type::Nil,
			Self::Bool(_) => Type::Bool,
			Self::Number(_) => Type::Number,
			Self::String(_) => Type::String,
			Self::Table(_) => Type::Table,
			Self::Function(_) => Type::Function,
			Self::Userdata(_) => Type::Userdata,
			Self::Coroutine(_) => Type::Coroutine,
			Self::LightUserdata(_) => Type::LightUserdata,
		}
	}

	pub const fn truthy(&self) -> bool {
		!self.falsy()
	}

	pub const fn falsy(&self) -> bool {
		matches!(self, Self::Nil | Self::Bool(false))
	}

	pub const fn is_nil(&self) -> bool {
		matches!(self, Self::Nil)
	}

	pub const fn not_nil(self) -> Option<Self> {
		if !self.is_nil() {
			Some(self)
		} else {
			forget(self);
			None
		}
	}
}

impl ToLua for Value {
	fn to_lua_by_ref(&self) -> Value {
		self.clone()
	}

	fn to_lua(self) -> Value {
		self
	}
}

impl FromLua for Value {
	type Err = FromLuaError<'static>;

	fn from_lua(value: Value) -> Result<Self, Self::Err> {
		Ok(value)
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::no_value())
	}
}

impl Debug for Value {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Nil => write!(f, "nil"),
			Self::Bool(bl) => Debug::fmt(bl, f),
			Self::Number(num) => Debug::fmt(num, f),
			Self::String(lstr) => Debug::fmt(lstr, f),
			Self::Table(tbl) => Debug::fmt(tbl, f),
			Self::Function(func) => Debug::fmt(func, f),
			Self::Userdata(ud) => Debug::fmt(ud, f),
			Self::Coroutine(cor) => Debug::fmt(cor, f),
			Self::LightUserdata(lud) => Debug::fmt(lud, f),
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
	Nil,
	Bool,
	Number,
	String,
	Table,
	Function,
	Userdata,
	Coroutine,
	LightUserdata,
}

impl Type {
	pub const fn name(self) -> &'static str {
		match self {
			Self::Nil => "nil",
			Self::Bool => "boolean",
			Self::Number => "number",
			Self::String => "string",
			Self::Table => "table",
			Self::Function => "function",
			Self::Userdata => "userdata",
			Self::Coroutine => "coroutine",
			Self::LightUserdata => "lightuserdata",
		}
	}
}

#[repr(transparent)]
struct Reference {
	not_ref_unwind_safe: PhantomData<UnsafeCell<()>>,
	not_unwind_safe: PhantomData<&'static mut ()>,
	not_send_sync: PhantomData<*mut ()>,
	id: i32,
}

impl Stack<'_> {
	#[track_caller]
	fn push_reference(&self, reference: &Reference) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_rawgeti(self.to_ptr(), ffi::LUA_REGISTRYINDEX, reference.id);
			}
		} else {
			stack_overflow!();
		}
	}

	unsafe fn pop_reference_unchecked(&self) -> Reference {
		Reference {
			id: unsafe { ffi::luaL_ref(self.to_ptr(), ffi::LUA_REGISTRYINDEX) },
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
				ffi::luaL_unref(lua.to_ptr(), ffi::LUA_REGISTRYINDEX, self.id);
			}
		});
	}
}
