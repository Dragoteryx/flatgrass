use crate::lua::error::FromLuaError;
use crate::lua::util::Tuple;
use crate::lua::value::Reference;
use crate::lua::{FromLua, Lua, Stack, ToLua, Type, Value};
use crate::{ffi, resume};
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Resume {
	Yield(Tuple),
	Return(Tuple),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
	Suspended,
	Running,
	Normal,
	Dead,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct Coroutine {
	reference: Rc<Reference>,
}

impl Stack<'_> {
	pub fn push_coroutine(&self, func: &Coroutine) {
		self.push_reference(&func.reference);
	}

	pub fn pop_coroutine(&self) -> Option<Coroutine> {
		if self.get_type(-1) == Some(Type::Coroutine) {
			unsafe { Some(self.pop_coroutine_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_coroutine_unchecked(&self) -> Coroutine {
		let reference = unsafe { self.pop_reference_unchecked() };
		Coroutine {
			reference: Rc::new(reference),
		}
	}

	pub fn get_coroutine(&self, idx: i32) -> Option<Coroutine> {
		if self.get_type(idx) == Some(Type::Coroutine) {
			unsafe { Some(self.get_coroutine_unchecked(idx)) }
		} else {
			None
		}
	}

	pub unsafe fn get_coroutine_unchecked(&self, idx: i32) -> Coroutine {
		let reference = unsafe { self.get_reference_unchecked(idx) };
		Coroutine {
			reference: Rc::new(reference),
		}
	}
}

impl Coroutine {
	pub fn new(func: ffi::lua_CFunction) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_new_coroutine(func);
			stack.pop_coroutine_unchecked()
		})
	}

	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_coroutine(self);
			let state = ffi::lua_tothread(lua.to_ptr(), -1);
			stack.pop_n(1);
			state
		})
	}

	pub fn is_suspended(&self) -> bool {
		self.status() == Status::Suspended
	}

	pub fn is_running(&self) -> bool {
		self.status() == Status::Running
	}

	pub fn is_normal(&self) -> bool {
		self.status() == Status::Normal
	}

	pub fn is_dead(&self) -> bool {
		self.status() == Status::Dead
	}

	pub fn status(&self) -> Status {
		Lua::get(|lua| unsafe {
			let ptr = self.to_ptr();
			if lua.to_ptr() == ptr {
				Status::Running
			} else {
				let stack = lua.stack();
				stack.push_coroutine(self);
				let status = ffi::lua_status(ptr);
				stack.pop_n(1);

				if status == ffi::LUA_YIELD {
					Status::Suspended
				} else if status == 0 {
					let mut dbg = std::mem::zeroed();
					if ffi::lua_getstack(ptr, 0, &mut dbg) != 0 {
						Status::Normal
					} else if ffi::lua_gettop(ptr) == 0 {
						Status::Dead
					} else {
						Status::Suspended
					}
				} else {
					Status::Dead
				}
			}
		})
	}

	pub fn resume<T: IntoIterator<Item: ToLua>>(&self, args: T) -> Result<Resume, Value> {
		unsafe {
			let stack = Stack::new(self.to_ptr());
			let n_args = stack.push_many(args);
			let status = ffi::lua_resume(stack.to_ptr(), n_args);
			if !matches!(status, ffi::LUA_YIELD | 0) {
				Err(stack.pop_value_unchecked())
			} else {
				let n_ret = stack.size() as usize;
				let mut values = Tuple::with_capacity(n_ret);
				for _ in 0..n_ret {
					values.push_front(stack.pop_value_unchecked());
				}

				if status == ffi::LUA_YIELD {
					Ok(Resume::Yield(values))
				} else {
					Ok(Resume::Return(values))
				}
			}
		}
	}
}

impl ToLua for Coroutine {
	fn to_lua_by_ref(&self) -> Value {
		self.clone().to_lua()
	}

	fn to_lua(self) -> Value {
		Value::Coroutine(self)
	}
}

impl FromLua for Coroutine {
	type Err = FromLuaError<'static>;

	fn from_lua(value: Value) -> Result<Self, Self::Err> {
		if let Value::Coroutine(cor) = value {
			Ok(cor)
		} else {
			Err(FromLuaError::expected_and_got_type(
				Type::Coroutine,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(Type::Coroutine))
	}
}

impl Debug for Coroutine {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct(&format!("Coroutine <{:?}>", self.to_ptr()))
			.field("status", &self.status())
			.finish()
	}
}

impl Eq for Coroutine {}
impl PartialEq for Coroutine {
	fn eq(&self, other: &Self) -> bool {
		self.to_ptr() == other.to_ptr()
	}
}

impl PartialOrd for Coroutine {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Coroutine {
	fn cmp(&self, other: &Self) -> Ordering {
		self.to_ptr().cmp(&other.to_ptr())
	}
}

impl Hash for Coroutine {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.to_ptr().hash(state);
	}
}

impl Iterator for Coroutine {
	type Item = Result<Resume, Value>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.is_dead() {
			false => Some(resume!(self)),
			true => None,
		}
	}
}

impl Future for Coroutine {
	type Output = Result<Tuple, Value>;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		match resume!(self) {
			Err(err) => Poll::Ready(Err(err)),
			Ok(Resume::Return(values)) => Poll::Ready(Ok(values)),
			Ok(Resume::Yield(_)) => {
				cx.waker().wake_by_ref();
				Poll::Pending
			}
		}
	}
}
