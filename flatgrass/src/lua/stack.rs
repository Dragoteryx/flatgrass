use crate::ffi;
use crate::lua::traits::{ToLua, ToLuaMany};
use crate::lua::value::userdata::LightUserdata;
use crate::lua::value::{Type, Value};
use std::ffi::CStr;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Provides a safe interface to the Lua stack.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Stack<'l> {
	ptr: NonNull<ffi::lua_State>,
	lua: PhantomData<&'l ()>,
}

/// An iterator over the values on the stack.
#[derive(Debug, Clone, Copy)]
pub struct StackIter<'l> {
	stack: Stack<'l>,
	idx: i32,
}

impl Iterator for StackIter<'_> {
	type Item = Value;

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
	pub unsafe fn new(ptr: *mut ffi::lua_State) -> Self {
		unsafe {
			Self {
				ptr: NonNull::new_unchecked(ptr),
				lua: PhantomData,
			}
		}
	}

	/// The raw Lua state associated with this stack.
	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		self.ptr.as_ptr()
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
	pub fn get_type(&self, idx: i32) -> Option<Type> {
		unsafe {
			match ffi::lua_type(self.to_ptr(), idx) {
				ffi::LUA_TNONE => None,
				ffi::LUA_TNIL => Some(Type::Nil),
				ffi::LUA_TNUMBER => Some(Type::Number),
				ffi::LUA_TBOOLEAN => Some(Type::Bool),
				ffi::LUA_TSTRING => Some(Type::String),
				ffi::LUA_TTABLE => Some(Type::Table),
				ffi::LUA_TFUNCTION => Some(Type::Function),
				ffi::LUA_TUSERDATA => Some(Type::Userdata),
				ffi::LUA_TTHREAD => Some(Type::Coroutine),
				ffi::LUA_TLIGHTUSERDATA => Some(Type::LightUserdata),
				_ => unreachable!(),
			}
		}
	}

	/// Returns the type of the value at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index is valid.
	pub unsafe fn get_type_unchecked(&self, idx: i32) -> Type {
		unsafe { self.get_type(idx).unwrap_unchecked() }
	}

	/// Returns the type of the value at the `idx` index, or `None` if the index isn't valid.
	pub fn get_value(&self, idx: i32) -> Option<Value> {
		self.get_type(idx).map(|ty| unsafe {
			match ty {
				Type::Nil => Value::Nil,
				Type::Bool => self.get_bool_unchecked(idx).to_lua(),
				Type::Number => self.get_number_unchecked(idx).to_lua(),
				Type::String => self.get_lua_string_unchecked(idx).to_lua(),
				Type::Table => self.get_table_unchecked(idx).to_lua(),
				Type::Function => self.get_function_unchecked(idx).to_lua(),
				Type::Userdata => self.get_userdata_unchecked(idx).to_lua(),
				Type::Coroutine => self.get_coroutine_unchecked(idx).to_lua(),
				Type::LightUserdata => self.get_light_userdata_unchecked(idx).to_lua(),
			}
		})
	}

	/// Returns the value at the `idx` index.
	///
	/// # Safety
	///
	/// You must ensure that the index is valid.
	pub unsafe fn get_value_unchecked(&self, idx: i32) -> Value {
		unsafe { self.get_value(idx).unwrap_unchecked() }
	}

	/// Returns the boolean at the `idx` index, or `None` if the value at that index isn't a boolean.
	pub fn get_bool(&self, idx: i32) -> Option<bool> {
		if self.get_type(idx) == Some(Type::Bool) {
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
		if self.get_type(idx) == Some(Type::Number) {
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
		if self.get_type(idx) == Some(Type::LightUserdata) {
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
	pub fn pop_value(&self) -> Option<Value> {
		self.get_type(-1).map(|ty| unsafe {
			match ty {
				Type::Bool => self.pop_bool_unchecked().to_lua(),
				Type::Number => self.pop_number_unchecked().to_lua(),
				Type::String => self.pop_lua_string_unchecked().to_lua(),
				Type::Table => self.pop_table_unchecked().to_lua(),
				Type::Function => self.pop_function_unchecked().to_lua(),
				Type::Userdata => self.pop_userdata_unchecked().to_lua(),
				Type::Coroutine => self.pop_coroutine_unchecked().to_lua(),
				Type::LightUserdata => self.pop_light_userdata_unchecked().to_lua(),
				Type::Nil => {
					self.pop_n(1);
					Value::Nil
				}
			}
		})
	}

	/// Pops the value at the top of the stack, returning it.
	///
	/// # Safety
	///
	/// You must ensure that the stack is not empty.
	pub unsafe fn pop_value_unchecked(&self) -> Value {
		unsafe { self.pop_value().unwrap_unchecked() }
	}

	/// Pops the boolean at the top of the stack, returning it, or `None` if the stack is empty.
	pub fn pop_bool(&self) -> Option<bool> {
		if self.get_type(-1) == Some(Type::Bool) {
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
		if self.get_type(-1) == Some(Type::Number) {
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
		if self.get_type(-1) == Some(Type::LightUserdata) {
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
	pub fn push_many<T: ToLuaMany>(&self, values: T) -> i32 {
		values.to_lua_many().fold(0, |n, value| {
			self.push_any(value);
			n + 1
		})
	}

	/// Pushes the value on the stack.
	#[track_caller]
	pub fn push_value(&self, value: &Value) {
		match value {
			Value::Nil => self.push_nil(),
			Value::Bool(bl) => self.push_bool(*bl),
			Value::Number(num) => self.push_number(*num),
			Value::String(lstr) => self.push_lua_string(lstr),
			Value::Table(tbl) => self.push_table(tbl),
			Value::Function(func) => self.push_function(func),
			Value::Userdata(ud) => self.push_userdata(ud),
			Value::Coroutine(cor) => self.push_coroutine(cor),
			Value::LightUserdata(ptr) => self.push_light_userdata(*ptr),
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

	/// Pushes a new coroutine on the stack and initializes it.
	#[track_caller]
	pub fn push_new_coroutine(&self, func: ffi::lua_CFunction) -> *mut ffi::lua_State {
		if self.check_size(1) {
			unsafe {
				let ptr = ffi::lua_newthread(self.to_ptr());
				Self::new(ptr).push_c_function(func);
				ptr
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
	pub fn push_c_closure<T: ToLuaMany>(&self, func: ffi::lua_CFunction, upvalues: T) {
		let n = self.push_many(upvalues);
		if self.check_size(1) {
			unsafe {
				ffi::lua_pushcclosure(self.to_ptr(), func, n);
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

impl Debug for Stack<'_> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Stack <{:?}> ", self.to_ptr())?;
		f.debug_list().entries(self.iter()).finish()
	}
}
