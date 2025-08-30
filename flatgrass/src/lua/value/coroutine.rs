use crate::ffi;
use crate::lua::Lua;
use crate::lua::stack::LuaStack;
use crate::lua::traits::{FromLua, FromLuaError, ToLua, ToLuaIter};
use crate::lua::value::{LuaReference, LuaType, LuaValue};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::{self, Debug};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
	Suspended,
	Running,
	Normal,
	Dead,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Resume {
	Return(VecDeque<LuaValue>),
	Yield(VecDeque<LuaValue>),
}

#[repr(transparent)]
#[derive(Clone)]
pub struct Coroutine {
	reference: Rc<LuaReference>,
}

impl LuaStack<'_> {
	pub fn push_coroutine(&self, func: &Coroutine) {
		self.push_reference(&func.reference);
	}

	pub fn pop_coroutine(&self) -> Option<Coroutine> {
		if self.get_type(-1) == Some(LuaType::Coroutine) {
			unsafe { Some(self.pop_coroutine_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_coroutine_unchecked(&self) -> Coroutine {
		Coroutine {
			reference: Rc::new(unsafe { self.pop_reference_unchecked() }),
		}
	}

	pub fn get_coroutine(&self, idx: i32) -> Option<Coroutine> {
		if self.get_type(idx) == Some(LuaType::Coroutine) {
			unsafe { Some(self.get_coroutine_unchecked(idx)) }
		} else {
			None
		}
	}

	pub unsafe fn get_coroutine_unchecked(&self, idx: i32) -> Coroutine {
		Coroutine {
			reference: Rc::new(unsafe { self.get_reference_unchecked(idx) }),
		}
	}
}

impl Coroutine {
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

	pub fn to_ptr(&self) -> *mut ffi::lua_State {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_coroutine(self);
			let state = ffi::lua_tothread(lua.to_ptr(), -1);
			stack.pop_n(1);
			state
		})
	}

	pub fn status(&self) -> Status {
		Lua::get(|lua| unsafe {
			if lua.to_ptr() == self.to_ptr() {
				Status::Running
			} else {
				let stack = lua.stack();
				stack.push_coroutine(self);
				let status = ffi::lua_status(self.to_ptr());
				stack.pop_n(1);

				match status {
					ffi::LUA_YIELD => Status::Suspended,
					0 => {
						let mut dbg = std::mem::zeroed();
						if ffi::lua_getstack(self.to_ptr(), 0, &mut dbg) != 0	{
							Status::Normal
						} else if ffi::lua_gettop(self.to_ptr()) == 0 {
							Status::Dead
						} else {
							Status::Suspended
						}
					}
					_ => Status::Dead
				}
			}
		})
	}

	pub fn resume<T: ToLuaIter>(&self, args: T) -> Result<Resume, LuaValue> {
		unsafe {
			let stack = LuaStack::new(self.to_ptr());
			let n = stack.push_many(args);
			match ffi::lua_resume(stack.to_ptr(), n) {
				status @ (0 | ffi::LUA_YIELD) => {
					let mut values = VecDeque::new();
					while stack.size() > 0 {
						values.push_front(stack.pop_value_unchecked());
					}

					if status == 0 {
						Ok(Resume::Return(values))
					} else {
						Ok(Resume::Yield(values))
					}
				}
				_ => Err(stack.pop_value_unchecked()),
			}
		}
	}
}

impl ToLua for Coroutine {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		LuaValue::Coroutine(self)
	}
}

impl FromLua for Coroutine {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Coroutine(cor) = value {
			Ok(cor)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Coroutine,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Coroutine))
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
