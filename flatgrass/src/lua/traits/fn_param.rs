use crate::ffi;
use crate::ffi::lua_upvalueindex;
use crate::lua::Lua;
use crate::lua::error::LuaError;
use crate::lua::stack::LuaStack;
use crate::lua::state::{State, StateError, StateRef};
use crate::lua::traits::{FromLua, ToLua};
use std::convert::Infallible;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display};
use std::mem::replace;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Upvalue<T>(pub T);

pub trait LuaFnParam<'l>: Sized {
	type Err: ToLua;

	fn lua_fn_param(lua: &'l Lua, arg: &mut i32, upv: &mut i32) -> Result<Self, Self::Err>;
}

impl<'l> LuaFnParam<'l> for &'l Lua {
	type Err = Infallible;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		Ok(lua)
	}
}

impl<'l> LuaFnParam<'l> for LuaStack<'l> {
	type Err = Infallible;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		Ok(lua.stack())
	}
}

impl<'l, T: 'static> LuaFnParam<'l> for State<'l, T> {
	type Err = LuaError<StateError>;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		lua.state()
			.ok_or_else(|| LuaError::new(StateError::new::<T>()))
	}
}

impl<'l, T: 'static> LuaFnParam<'l> for StateRef<'l, T> {
	type Err = LuaError<StateError>;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		lua.state_ref()
			.ok_or_else(|| LuaError::new(StateError::new::<T>()))
	}
}

impl<'l, T: FromLua<Err: ToString>> LuaFnParam<'l> for T {
	type Err = LuaError<BadArgumentError<T::Err>>;

	fn lua_fn_param(lua: &'l Lua, arg: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		let arg = replace(arg, *arg + 1);
		let res = match lua.stack().get_value(arg) {
			Some(value) => T::from_lua(value),
			None => T::no_value(),
		};

		res.map_err(|err| LuaError::new(BadArgumentError::new(arg, err)))
	}
}

impl<'l, T: FromLua<Err: ToString>> LuaFnParam<'l> for Upvalue<T> {
	type Err = LuaError<T::Err>;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
		let upv = replace(upv, *upv + 1);
		let res = match lua.stack().get_value(lua_upvalueindex(upv)) {
			Some(value) => T::from_lua(value),
			None => T::no_value(),
		};

		match res {
			Ok(value) => Ok(Self(value)),
			Err(err) => Err(LuaError::new(err)),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BadArgumentError<T> {
	name: Option<String>,
	arg: i32,
	source: T,
}

impl<T> BadArgumentError<T> {
	pub fn new(mut arg: i32, source: T) -> Self {
		Lua::get(|lua| unsafe {
			let mut dbg = std::mem::zeroed();
			let name = match ffi::lua_getstack(lua.to_ptr(), 0, &mut dbg) == 0 {
				true => None,
				false => {
					ffi::lua_getinfo(lua.to_ptr(), c"n".as_ptr(), &mut dbg);
					if ffi::libc::strcmp(dbg.namewhat, c"method".as_ptr()) == 0 {
						arg -= 1;
					}

					if !dbg.name.is_null() {
						let name = CStr::from_ptr(dbg.name);
						Some(name.to_string_lossy().to_string())
					} else {
						None
					}
				}
			};

			Self { name, arg, source }
		})
	}

	pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	pub fn arg(&self) -> i32 {
		self.arg
	}

	pub fn source(&self) -> &T {
		&self.source
	}
}

impl<T: Error + 'static> Error for BadArgumentError<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.source)
	}
}

impl<T: ToString> Display for BadArgumentError<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = self.source.to_string();
		match (self.name(), self.arg) {
			(Some(name), 0) => write!(f, "calling '{name}' on bad self ({msg})"),
			(Some(name), arg) => write!(f, "bad argument #{arg} to '{name}' ({msg})"),
			(None, 0) => write!(f, "bad self ({msg})"),
			(None, arg) => write!(f, "bad argument #{arg} ({msg})"),
		}
	}
}
