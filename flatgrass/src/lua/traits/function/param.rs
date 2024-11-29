use super::{FromLua, Lua, ToLua};
use crate::ffi::lua_upvalueindex;
use crate::lua::errors::{BadArgumentError, LuaError};
use crate::lua::LuaStack;
use std::convert::Infallible;
use std::mem::replace;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Upvalue<T>(pub T);

#[cfg_attr(docsrs, doc(notable_trait))]
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

impl<'l> LuaFnParam<'l> for &'l LuaStack {
	type Err = Infallible;

	fn lua_fn_param(lua: &'l Lua, _: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		Ok(lua.stack())
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
