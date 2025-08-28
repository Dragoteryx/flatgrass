use crate::ffi;
use crate::lua::Lua;
use crate::lua::error::FromLuaError;
use crate::lua::stack::LuaStack;
use crate::lua::traits::{FromLua, ToLua, ToLuaIter};
use crate::lua::value::{LuaReference, LuaType, LuaValue};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[repr(transparent)]
#[derive(Clone)]
pub struct Function {
	reference: Rc<LuaReference>,
}

impl LuaStack<'_> {
	pub fn push_function(&self, func: &Function) {
		self.push_reference(&func.reference);
	}

	pub fn pop_function(&self) -> Option<Function> {
		if self.get_type(-1) == Some(LuaType::Function) {
			unsafe { Some(self.pop_function_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_function_unchecked(&self) -> Function {
		Function {
			reference: Rc::new(unsafe { self.pop_reference_unchecked() }),
		}
	}

	pub fn get_function(&self, idx: i32) -> Option<Function> {
		if self.get_type(idx) == Some(LuaType::Function) {
			Some(unsafe { self.get_function_unchecked(idx) })
		} else {
			None
		}
	}

	pub unsafe fn get_function_unchecked(&self, idx: i32) -> Function {
		Function {
			reference: Rc::new(unsafe { self.get_reference_unchecked(idx) }),
		}
	}
}

impl Function {
	pub fn new(func: ffi::lua_CFunction) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_c_function(func);
			stack.pop_function_unchecked()
		})
	}

	pub fn closure<T: ToLuaIter>(func: ffi::lua_CFunction, upvalues: T) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_c_closure(func, upvalues);
			stack.pop_function_unchecked()
		})
	}

	pub fn to_ptr(&self) -> *const ffi::libc::c_void {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_function(self);
			let ptr = ffi::lua_topointer(lua.to_ptr(), -1);
			stack.pop_n(1);
			ptr
		})
	}

	pub fn to_c_function(&self) -> Option<ffi::lua_CFunction> {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_function(self);
			let func = ffi::lua_tocfunction(lua.to_ptr(), -1);
			stack.pop_n(1);
			func
		})
	}

	pub fn call<T: ToLuaIter>(&self, args: T) -> Result<VecDeque<LuaValue>, LuaValue> {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			let size = stack.size();
			stack.push_function(self);
			let n = stack.push_many(args);
			let res = ffi::lua_pcall(lua.to_ptr(), n, ffi::LUA_MULTRET, 0);
			if res == 0 {
				let mut values = VecDeque::new();
				while stack.size() > size {
					values.push_front(stack.pop_value_unchecked());
				}

				Ok(values)
			} else {
				Err(stack.pop_value_unchecked())
			}
		})
	}
}

impl ToLua for Function {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		LuaValue::Function(self)
	}
}

impl FromLua for Function {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Function(func) = value {
			Ok(func)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Function,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Function))
	}
}

impl Default for Function {
	fn default() -> Self {
		Self::new(ffi::raw_function!(|_| ffi::LUA_MULTRET))
	}
}

impl Debug for Function {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Function[{:?}]", self.to_ptr())
	}
}

impl Eq for Function {}
impl PartialEq for Function {
	fn eq(&self, other: &Self) -> bool {
		self.to_ptr() == other.to_ptr()
	}
}

impl PartialOrd for Function {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Function {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_ptr().cmp(&other.to_ptr())
	}
}

impl Hash for Function {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_ptr().hash(state);
	}
}
