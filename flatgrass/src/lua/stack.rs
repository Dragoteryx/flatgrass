use super::Lua;
use super::traits::{ToLua, ToLuaIter};
use super::value::{LightUserdata, LuaType, LuaValue};
use crate::ffi;
use std::ffi::CStr;

/// Provides a safe interface to the Lua stack.
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Stack<'l> {
	pub(super) lua: &'l Lua,
}

/// An iterator over the values on the stack.
#[derive(Debug, Clone, Copy)]
pub struct StackIter<'l> {
	stack: Stack<'l>,
	idx: i32,
}

impl Iterator for StackIter<'_> {
	type Item = LuaValue;

	fn next(&mut self) -> Option<Self::Item> {
		match self.stack.get_value(self.idx) {
			Some(value) => {
				self.idx += 1;
				Some(value)
			}
			None => {
				self.idx = 1;
				None
			}
		}
	}
}

impl<'l> Stack<'l> {
	/// The raw Lua state associated with this stack.
	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		self.lua.to_ptr()
	}

	/// Iterate over the values on the stack.
	pub fn iter(&self) -> StackIter<'l> {
		StackIter {
			stack: *self,
			idx: 1,
		}
	}

	/// The current size of the stack.
	pub fn size(&self) -> i32 {
		unsafe { ffi::lua_gettop(self.to_ptr()) }
	}

	/// Ensures that there are at least `n` free slots on top of the stack.\
	/// Returns `false` if the stack cannot grow to that size.
	#[must_use = "trying to push a value on the stack when it is full will cause a panic"]
	pub fn check_size(&self, n: i32) -> bool {
		unsafe { ffi::lua_checkstack(self.to_ptr(), n.max(0)) != 0 }
	}

	/// Pops `n` values from the stack.
	pub fn pop_n(&self, n: i32) {
		unsafe { ffi::lua_pop(self.to_ptr(), n.clamp(0, self.size())) }
	}

	/// Clear the stack, removing every element.
	pub fn clear(&self) {
		unsafe {
			ffi::lua_settop(self.to_ptr(), 0);
		}
	}

	/// Checks if the `idx` index is valid.
	pub fn is_valid(&self, idx: i32) -> bool {
		self.get_type(idx).is_some()
	}

	/// Returns the type of the value at the `idx` index, or `None` if the index isn't valid.
	pub fn get_type(&self, idx: i32) -> Option<LuaType> {
		unsafe {
			match ffi::lua_type(self.to_ptr(), idx) {
				ffi::LUA_TNONE => None,
				ffi::LUA_TNIL => Some(LuaType::Nil),
				ffi::LUA_TNUMBER => Some(LuaType::Number),
				ffi::LUA_TBOOLEAN => Some(LuaType::Bool),
				ffi::LUA_TSTRING => Some(LuaType::String),
				ffi::LUA_TTABLE => Some(LuaType::Table),
				ffi::LUA_TFUNCTION => Some(LuaType::Function),
				ffi::LUA_TUSERDATA => Some(LuaType::Userdata),
				ffi::LUA_TTHREAD => Some(LuaType::Coroutine),
				ffi::LUA_TLIGHTUSERDATA => Some(LuaType::LightUserdata),
				_ => unreachable!(),
			}
		}
	}

	/// Returns the type of the value at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index is valid.
	pub unsafe fn get_type_unchecked(&self, idx: i32) -> LuaType {
		unsafe { self.get_type(idx).unwrap_unchecked() }
	}

	/// Returns the type of the value at the `idx` index, or `None` if the index isn't valid.
	pub fn get_value(&self, idx: i32) -> Option<LuaValue> {
		self.get_type(idx).map(|ty| unsafe {
			match ty {
				LuaType::Nil => LuaValue::Nil,
				LuaType::Bool => self.get_bool_unchecked(idx).to_lua(),
				LuaType::Number => self.get_number_unchecked(idx).to_lua(),
				LuaType::String => self.get_lua_string_unchecked(idx).to_lua(),
				LuaType::Table => self.get_table_unchecked(idx).to_lua(),
				LuaType::Function => self.get_function_unchecked(idx).to_lua(),
				LuaType::Userdata => self.get_userdata_unchecked(idx).to_lua(),
				LuaType::Coroutine => self.get_coroutine_unchecked(idx).to_lua(),
				LuaType::LightUserdata => self.get_light_userdata_unchecked(idx).to_lua(),
			}
		})
	}

	/// Returns the value at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index is valid.
	pub unsafe fn get_value_unchecked(&self, idx: i32) -> LuaValue {
		unsafe { self.get_value(idx).unwrap_unchecked() }
	}

	/// Returns the boolean at the `idx` index, or `None` if the value at that index isn't a boolean.
	pub fn get_bool(&self, idx: i32) -> Option<bool> {
		if self.get_type(idx) == Some(LuaType::Bool) {
			Some(unsafe { self.get_bool_unchecked(idx) })
		} else {
			None
		}
	}

	/// Returns the boolean at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index contains a boolean.
	pub unsafe fn get_bool_unchecked(&self, idx: i32) -> bool {
		unsafe { ffi::lua_toboolean(self.to_ptr(), idx) != 0 }
	}

	/// Returns the number at the `idx` index, or `None` if the value at that index isn't a number.
	pub fn get_number(&self, idx: i32) -> Option<f64> {
		if self.get_type(idx) == Some(LuaType::Number) {
			Some(unsafe { self.get_number_unchecked(idx) })
		} else {
			None
		}
	}

	/// Returns the number at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index contains a number.
	pub unsafe fn get_number_unchecked(&self, idx: i32) -> f64 {
		unsafe { ffi::lua_tonumber(self.to_ptr(), idx) }
	}

	// Returns the light userdata at the `idx` index, or `None` if the value at that index isn't a light userdata.
	pub fn get_light_userdata(&self, idx: i32) -> Option<LightUserdata> {
		if self.get_type(idx) == Some(LuaType::LightUserdata) {
			Some(unsafe { self.get_light_userdata_unchecked(idx) })
		} else {
			None
		}
	}

	/// Returns the light userdata at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index contains a light userdata.
	pub unsafe fn get_light_userdata_unchecked(&self, idx: i32) -> LightUserdata {
		unsafe { ffi::lua_touserdata(self.to_ptr(), idx) }
	}

	/// Pops the value at the top of the stack, returning it, or `None` if the stack is empty.
	pub fn pop_value(&self) -> Option<LuaValue> {
		self.get_type(-1).map(|ty| unsafe {
			match ty {
				LuaType::Bool => self.pop_bool_unchecked().to_lua(),
				LuaType::Number => self.pop_number_unchecked().to_lua(),
				LuaType::String => self.pop_lua_string_unchecked().to_lua(),
				LuaType::Table => self.pop_table_unchecked().to_lua(),
				LuaType::Function => self.pop_function_unchecked().to_lua(),
				LuaType::Userdata => self.pop_userdata_unchecked().to_lua(),
				LuaType::Coroutine => self.pop_coroutine_unchecked().to_lua(),
				LuaType::LightUserdata => self.pop_light_userdata_unchecked().to_lua(),
				LuaType::Nil => {
					self.pop_n(1);
					LuaValue::Nil
				}
			}
		})
	}

	/// Pops the value at the top of the stack, returning it.
	///
	/// # Safety
	///
	/// You must ensure that the stack is not empty.
	pub unsafe fn pop_value_unchecked(&self) -> LuaValue {
		unsafe { self.pop_value().unwrap_unchecked() }
	}

	/// Pops the boolean at the top of the stack, returning it, or `None` if the stack is empty.
	pub fn pop_bool(&self) -> Option<bool> {
		if self.get_type(-1) == Some(LuaType::Bool) {
			Some(unsafe { self.pop_bool_unchecked() })
		} else {
			None
		}
	}

	/// Pops the boolean at the top of the stack, returning it.
	///
	/// # Safety
	///
	/// You must ensure that the stack is not empty.
	pub unsafe fn pop_bool_unchecked(&self) -> bool {
		let bl = unsafe { self.get_bool_unchecked(-1) };
		self.pop_n(1);
		bl
	}

	/// Pops the number at the top of the stack, returning it, or `None` if the stack is empty.
	pub fn pop_number(&self) -> Option<f64> {
		if self.get_type(-1) == Some(LuaType::Number) {
			Some(unsafe { self.pop_number_unchecked() })
		} else {
			None
		}
	}

	/// Pops the number at the top of the stack, returning it.
	///
	/// # Safety
	///
	/// You must ensure that the value at the top of the stack is a number.
	pub unsafe fn pop_number_unchecked(&self) -> f64 {
		let num = unsafe { self.get_number_unchecked(-1) };
		self.pop_n(1);
		num
	}

	/// Pops the light userdata at the top of the stack, returning it, or `None` if the stack is empty.
	pub fn pop_light_userdata(&self) -> Option<LightUserdata> {
		if self.get_type(-1) == Some(LuaType::LightUserdata) {
			Some(unsafe { self.pop_light_userdata_unchecked() })
		} else {
			None
		}
	}

	/// Pops the light userdata at the top of the stack, returning it.
	///
	/// # Safety
	///
	/// You must ensure that the value at the top of the stack is a light userdata.
	pub unsafe fn pop_light_userdata_unchecked(&self) -> LightUserdata {
		let ptr = unsafe { self.get_light_userdata_unchecked(-1) };
		self.pop_n(1);
		ptr
	}

	/// Pushes any value on the stack.
	#[track_caller]
	pub fn push_any<T: ToLua>(&self, value: T) {
		self.push_value(&value.to_lua());
	}

	/// Pushes multiple values on the stack.
	#[track_caller]
	pub fn push_many<T: ToLuaIter>(&self, values: T) -> i32 {
		values.to_lua_iter().into_iter().fold(0, |n, value| {
			self.push_any(value);
			n + 1
		})
	}

	/// Pushes the value on the stack.
	#[track_caller]
	pub fn push_value(&self, value: &LuaValue) {
		match value {
			LuaValue::Nil => self.push_nil(),
			LuaValue::Bool(bl) => self.push_bool(*bl),
			LuaValue::Number(num) => self.push_number(*num),
			LuaValue::String(lstr) => self.push_lua_string(lstr),
			LuaValue::Table(tbl) => self.push_table(tbl),
			LuaValue::Function(func) => self.push_function(func),
			LuaValue::Userdata(ud) => self.push_userdata(ud),
			LuaValue::Coroutine(cor) => self.push_coroutine(cor),
			LuaValue::LightUserdata(ptr) => self.push_light_userdata(*ptr),
		}
	}

	/// Pushes a nil value on the stack.
	#[track_caller]
	pub fn push_nil(&self) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushnil(self.to_ptr());
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a boolean on the stack.
	#[track_caller]
	pub fn push_bool(&self, bl: bool) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushboolean(self.to_ptr(), bl as _);
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a number on the stack.
	#[track_caller]
	pub fn push_number(&self, num: f64) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushnumber(self.to_ptr(), num);
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a string on the stack.
	#[track_caller]
	pub fn push_string(&self, rstr: &str) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushlstring(self.to_ptr(), rstr.as_ptr().cast(), rstr.len());
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a C string on the stack.
	#[track_caller]
	pub fn push_c_string(&self, cstr: &CStr) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushstring(self.to_ptr(), cstr.as_ptr());
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes the location at the desired level on the stack.
	#[track_caller]
	pub fn push_location(&self, lvl: i32) {
		if self.check_size(1) {
			unsafe {
				ffi::luaL_where(self.to_ptr(), lvl.max(0));
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a new empty table on the stack.
	#[track_caller]
	pub fn push_new_table(&self) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_newtable(self.to_ptr());
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a raw Lua function on the stack.
	#[track_caller]
	pub fn push_c_function(&self, func: ffi::lua_CFunction) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushcfunction(self.to_ptr(), func);
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a raw Lua function on the stack with upvalues.
	#[track_caller]
	pub fn push_c_closure<T: ToLuaIter>(&self, func: ffi::lua_CFunction, upvalues: T) {
		let nvalues = self.push_many(upvalues);
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushcclosure(self.to_ptr(), func, nvalues);
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes a light userdata on the stack.
	#[track_caller]
	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	pub fn push_light_userdata(&self, ptr: LightUserdata) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushlightuserdata(self.to_ptr(), ptr);
			}
		} else {
			stack_overflow!();
		}
	}

	/// Pushes the value at the `idx` index on top of the stack.
	/// Returns whether the index was valid.
	#[track_caller]
	pub fn push_index(&self, idx: i32) -> bool {
		if self.is_valid(idx) {
			unsafe {
				self.push_index_unchecked(idx);
				true
			}
		} else {
			false
		}
	}

	/// Pushes the value at the `idx` index on top of the stack.
	///
	/// # Safety
	///
	/// You must ensure that the index is valid.
	#[track_caller]
	pub unsafe fn push_index_unchecked(&self, idx: i32) {
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushvalue(self.to_ptr(), idx);
			}
		} else {
			stack_overflow!();
		}
	}
}
