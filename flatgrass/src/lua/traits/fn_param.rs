use crate::ffi;
use crate::ffi::lua_upvalueindex;
use crate::lua::Lua;
use crate::lua::error::LuaError;
use crate::lua::traits::{FromLua, ToLua};
use crate::lua::value::Tuple;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display};
use std::mem::replace;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Upvalue<T>(pub T);

pub trait LuaFnParam: Sized {
	type Err: ToLua;

	fn lua_fn_param(lua: &Lua, arg: &mut i32, upv: &mut i32) -> Result<Self, Self::Err>;
}

impl<T: FromLua<Err: ToString>> LuaFnParam for T {
	type Err = LuaError<BadArgumentError<T::Err>>;

	fn lua_fn_param(lua: &Lua, arg: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		let arg = replace(arg, *arg + 1);
		let res = match lua.stack().get_value(arg) {
			Some(value) => T::from_lua(value),
			None => T::no_value(),
		};

		res.map_err(|err| LuaError::new(BadArgumentError::new(arg, err)))
	}
}

impl<T: FromLua<Err: ToString>> LuaFnParam for Upvalue<T> {
	type Err = LuaError<T::Err>;

	fn lua_fn_param(lua: &Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
		let upv = replace(upv, *upv + 1);
		let idx = lua_upvalueindex(upv);
		let res = match lua.stack().get_value(idx) {
			Some(value) => T::from_lua(value),
			None => T::no_value(),
		};

		match res {
			Ok(value) => Ok(Self(value)),
			Err(err) => Err(LuaError::new(err)),
		}
	}
}

impl<T: FromLua<Err: ToString>> LuaFnParam for Tuple<T> {
	type Err = LuaError<BadArgumentError<T::Err>>;

	fn lua_fn_param(lua: &Lua, arg: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		let mut tuple = Self::new();
		while let Some(value) = lua.stack().get_value(*arg) {
			let value = T::from_lua(value)
				.map_err(|err| LuaError::new(BadArgumentError::new(*arg, err)))?;
			tuple.push_back(value);
			*arg += 1;
		}

		Ok(tuple)
	}
}

impl<T: FromLua<Err: ToString>> LuaFnParam for Tuple<Upvalue<T>> {
	type Err = LuaError<T::Err>;

	fn lua_fn_param(lua: &Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
		let mut tuple = Self::new();
		while let Some(value) = lua.stack().get_value(*upv) {
			let value = T::from_lua(value).map_err(LuaError::new)?;
			tuple.push_back(Upvalue(value));
			*upv += 1;
		}

		Ok(tuple)
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
