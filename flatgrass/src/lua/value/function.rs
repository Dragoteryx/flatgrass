use crate::ffi;
use crate::lua::Lua;
use crate::lua::stack::Stack;
use crate::lua::traits::{FromLua, FromLuaError, ToLua};
use crate::lua::value::{Reference, Tuple, Type, Value};
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[repr(transparent)]
#[derive(Clone)]
pub struct Function {
	reference: Rc<Reference>,
}

impl Stack<'_> {
	pub fn push_function(&self, func: &Function) {
		self.push_reference(&func.reference);
	}

	pub fn pop_function(&self) -> Option<Function> {
		if self.get_type(-1) == Some(Type::Function) {
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
		if self.get_type(idx) == Some(Type::Function) {
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

	pub fn closure<T: IntoIterator<Item: ToLua>>(func: ffi::lua_CFunction, upvalues: T) -> Self {
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

	pub fn call<T: IntoIterator<Item: ToLua>>(&self, args: T) -> Result<Tuple, Value> {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			let size = stack.size();
			stack.push_function(self);
			let n_args = stack.push_many(args);
			let status = ffi::lua_pcall(lua.to_ptr(), n_args, ffi::LUA_MULTRET, 0);
			if status == 0 {
				let n_ret = (stack.size() - size) as usize;
				let mut values = Tuple::with_capacity(n_ret);
				for _ in 0..n_ret {
					values.push_front(stack.pop_value_unchecked());
				}

				Ok(values)
			} else {
				Err(stack.pop_value_unchecked())
			}
		})
	}

	pub fn call0(&self) -> Result<Tuple, Value> {
		self.call::<[u8; _]>([])
	}

	pub fn call1<T: ToLua>(&self, arg: T) -> Result<Tuple, Value> {
		self.call([arg])
	}

	pub fn call2<T1, T2>(&self, arg1: T1, arg2: T2) -> Result<Tuple, Value>
	where
		T1: ToLua,
		T2: ToLua,
	{
		self.call([arg1.to_lua(), arg2.to_lua()])
	}

	pub fn call3<T1, T2, T3>(&self, arg1: T1, arg2: T2, arg3: T3) -> Result<Tuple, Value>
	where
		T1: ToLua,
		T2: ToLua,
		T3: ToLua,
	{
		self.call([arg1.to_lua(), arg2.to_lua(), arg3.to_lua()])
	}

	pub fn call4<T1, T2, T3, T4>(
		&self,
		arg1: T1,
		arg2: T2,
		arg3: T3,
		arg4: T4,
	) -> Result<Tuple, Value>
	where
		T1: ToLua,
		T2: ToLua,
		T3: ToLua,
		T4: ToLua,
	{
		self.call([arg1.to_lua(), arg2.to_lua(), arg3.to_lua(), arg4.to_lua()])
	}

	pub fn call5<T1, T2, T3, T4, T5>(
		&self,
		arg1: T1,
		arg2: T2,
		arg3: T3,
		arg4: T4,
		arg5: T5,
	) -> Result<Tuple, Value>
	where
		T1: ToLua,
		T2: ToLua,
		T3: ToLua,
		T4: ToLua,
		T5: ToLua,
	{
		self.call([
			arg1.to_lua(),
			arg2.to_lua(),
			arg3.to_lua(),
			arg4.to_lua(),
			arg5.to_lua(),
		])
	}

	pub fn call6<T1, T2, T3, T4, T5, T6>(
		&self,
		arg1: T1,
		arg2: T2,
		arg3: T3,
		arg4: T4,
		arg5: T5,
		arg6: T6,
	) -> Result<Tuple, Value>
	where
		T1: ToLua,
		T2: ToLua,
		T3: ToLua,
		T4: ToLua,
		T5: ToLua,
		T6: ToLua,
	{
		self.call([
			arg1.to_lua(),
			arg2.to_lua(),
			arg3.to_lua(),
			arg4.to_lua(),
			arg5.to_lua(),
			arg6.to_lua(),
		])
	}
}

impl ToLua for Function {
	fn to_lua_by_ref(&self) -> Value {
		self.clone().to_lua()
	}

	fn to_lua(self) -> Value {
		Value::Function(self)
	}
}

impl ToLua for ffi::lua_CFunction {
	fn to_lua_by_ref(&self) -> Value {
		Function::new(*self).to_lua()
	}
}

impl FromLua for Function {
	type Err = FromLuaError<'static>;

	fn from_lua(value: Value) -> Result<Self, Self::Err> {
		if let Value::Function(func) = value {
			Ok(func)
		} else {
			Err(FromLuaError::expected_and_got_type(
				Type::Function,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(Type::Function))
	}
}

impl Default for Function {
	fn default() -> Self {
		Self::new(ffi::raw_function!(|_| ffi::LUA_MULTRET))
	}
}

impl Debug for Function {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Function <{:?}>", self.to_ptr())
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
