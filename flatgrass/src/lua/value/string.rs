use crate::ffi;
use crate::lua::Lua;
use crate::lua::error::FromLuaError;
use crate::lua::stack::LuaStack;
use crate::lua::traits::{FromLua, ToLua};
use crate::lua::value::{LuaReference, LuaType, LuaValue};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use std::slice;

#[repr(transparent)]
#[derive(Clone)]
pub struct LuaString {
	reference: Rc<LuaReference>,
}

impl LuaStack<'_> {
	pub fn push_lua_string(&self, lstr: &LuaString) {
		self.push_reference(&lstr.reference);
	}

	pub fn pop_lua_string(&self) -> Option<LuaString> {
		if self.get_type(-1) == Some(LuaType::String) {
			unsafe { Some(self.pop_lua_string_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_lua_string_unchecked(&self) -> LuaString {
		LuaString {
			reference: Rc::new(unsafe { self.pop_reference_unchecked() }),
		}
	}

	pub fn get_lua_string(&self, idx: i32) -> Option<LuaString> {
		if self.get_type(idx) == Some(LuaType::String) {
			Some(unsafe { self.get_lua_string_unchecked(idx) })
		} else {
			None
		}
	}

	pub unsafe fn get_lua_string_unchecked(&self, idx: i32) -> LuaString {
		LuaString {
			reference: Rc::new(unsafe { self.get_reference_unchecked(idx) }),
		}
	}
}

impl LuaString {
	pub fn new() -> Self {
		Self::from("")
	}

	pub fn to_bytes(&self) -> &[u8] {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_lua_string(self);
			let mut len = 0;
			let ptr = ffi::lua_tolstring(lua.to_ptr(), -1, &mut len);
			let bytes = slice::from_raw_parts(ptr.cast(), len);
			stack.pop_n(1);
			bytes
		})
	}

	pub fn to_str(&self) -> Cow<'_, str> {
		String::from_utf8_lossy(self.to_bytes())
	}

	pub fn to_c_str(&self) -> &CStr {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_lua_string(self);
			let ptr = ffi::lua_tostring(lua.to_ptr(), -1);
			let cstr = CStr::from_ptr(ptr);
			stack.pop_n(1);
			cstr
		})
	}
}

impl ToLua for LuaString {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		LuaValue::String(self)
	}
}

impl FromLua for LuaString {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::String(lstr) = value {
			Ok(lstr)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::String,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::String))
	}
}

impl Default for LuaString {
	fn default() -> Self {
		Self::new()
	}
}

impl Debug for LuaString {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:?}", self.to_str())
	}
}

impl Display for LuaString {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_str())
	}
}

impl Eq for LuaString {}
impl PartialEq for LuaString {
	fn eq(&self, other: &Self) -> bool {
		self.to_bytes() == other.to_bytes()
	}
}

impl PartialOrd for LuaString {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for LuaString {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_bytes().cmp(other.to_bytes())
	}
}

impl Hash for LuaString {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_bytes().hash(state);
	}
}

impl From<&str> for LuaString {
	fn from(value: &str) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_string(value);
			stack.pop_lua_string_unchecked()
		})
	}
}

impl From<String> for LuaString {
	fn from(value: String) -> Self {
		Self::from(value.as_str())
	}
}

impl From<&CStr> for LuaString {
	fn from(value: &CStr) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_c_string(value);
			stack.pop_lua_string_unchecked()
		})
	}
}

impl From<CString> for LuaString {
	fn from(value: CString) -> Self {
		Self::from(value.as_c_str())
	}
}
