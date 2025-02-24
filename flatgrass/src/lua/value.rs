use super::errors::FromLuaError;
use super::traits::{FromLua, ToLua, ToLuaIter};
use std::fmt::{self, Debug};
use std::mem::forget;

mod reference;
pub use reference::*;

#[cfg(feature = "serde")]
mod serde;

pub type LightUserdata = *mut crate::ffi::libc::c_void;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LuaType {
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

impl LuaType {
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

#[derive(Default, Clone, PartialEq, PartialOrd)]
pub enum LuaValue {
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

impl LuaValue {
	pub const fn get_type(&self) -> LuaType {
		match self {
			Self::Nil => LuaType::Nil,
			Self::Bool(_) => LuaType::Bool,
			Self::Number(_) => LuaType::Number,
			Self::String(_) => LuaType::String,
			Self::Table(_) => LuaType::Table,
			Self::Function(_) => LuaType::Function,
			Self::Userdata(_) => LuaType::Userdata,
			Self::Coroutine(_) => LuaType::Coroutine,
			Self::LightUserdata(_) => LuaType::LightUserdata,
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

impl ToLua for LuaValue {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone()
	}

	fn to_lua(self) -> LuaValue {
		self
	}
}

impl FromLua for LuaValue {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		Ok(value)
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::no_value())
	}
}

impl Debug for LuaValue {
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
