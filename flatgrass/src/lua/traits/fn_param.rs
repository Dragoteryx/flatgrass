use crate::ffi::lua_upvalueindex;
use crate::lua::error::{BadArgError, LuaError};
use crate::lua::util::{Tuple, Upvalue};
use crate::lua::{FromLua, Lua, ToLua};
use std::mem::replace;

pub trait FnParam: Sized {
	type Err: ToLua;

	fn fn_param(lua: &Lua, arg: &mut i32, upv: &mut i32) -> Result<Self, Self::Err>;
}

impl<T: FromLua<Err: ToString>> FnParam for T {
	type Err = LuaError<BadArgError<T::Err>>;

	fn fn_param(lua: &Lua, arg: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		let arg = replace(arg, *arg + 1);
		let res = match lua.stack().get_value(arg) {
			Some(value) => T::from_lua(value),
			None => T::no_value(),
		};

		res.map_err(|err| LuaError::new(BadArgError::new(arg, err)))
	}
}

impl<T: FromLua<Err: ToString>> FnParam for Upvalue<T> {
	type Err = LuaError<T::Err>;

	fn fn_param(lua: &Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
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

impl<T: FromLua<Err: ToString>> FnParam for Tuple<T> {
	type Err = LuaError<BadArgError<T::Err>>;

	fn fn_param(lua: &Lua, arg: &mut i32, _: &mut i32) -> Result<Self, Self::Err> {
		let mut tuple = Self::new();
		while let Some(value) = lua.stack().get_value(*arg) {
			let value =
				T::from_lua(value).map_err(|err| LuaError::new(BadArgError::new(*arg, err)))?;
			tuple.push_back(value);
			*arg += 1;
		}

		Ok(tuple)
	}
}

impl<T: FromLua<Err: ToString>> FnParam for Tuple<Upvalue<T>> {
	type Err = LuaError<T::Err>;

	fn fn_param(lua: &Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
		let mut tuple = Self::new();
		let idx = lua_upvalueindex(*upv);
		while let Some(value) = lua.stack().get_value(idx) {
			let value = T::from_lua(value).map_err(LuaError::new)?;
			tuple.push_back(Upvalue(value));
			*upv += 1;
		}

		Ok(tuple)
	}
}

impl<T: FromLua<Err: ToString>> FnParam for Upvalue<Tuple<T>> {
	type Err = LuaError<T::Err>;

	fn fn_param(lua: &Lua, _: &mut i32, upv: &mut i32) -> Result<Self, Self::Err> {
		let mut tuple = Tuple::new();
		let idx = lua_upvalueindex(*upv);
		while let Some(value) = lua.stack().get_value(idx) {
			let value = T::from_lua(value).map_err(LuaError::new)?;
			tuple.push_back(value);
			*upv += 1;
		}

		Ok(Upvalue(tuple))
	}
}
