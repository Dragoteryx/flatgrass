use crate::lua::Lua;
use crate::lua::traits::ToLua;
use crate::lua::value::{Tuple, Value};
use std::convert::Infallible;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Yield<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Return<T> {
	Values(T),
	Yield(T),
}

pub trait LuaFnReturn: Sized {
	type Return: IntoIterator<Item: ToLua>;
	type Err: ToLua;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err>;
}

impl<T: ToLua> LuaFnReturn for T {
	type Return = [T; 1];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self]))
	}
}

impl<T: ToLua> LuaFnReturn for Tuple<T> {
	type Return = Self;
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values(self))
	}
}

impl LuaFnReturn for () {
	type Return = [Value; 0];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([]))
	}
}

impl<T1: ToLua> LuaFnReturn for (T1,) {
	type Return = [Value; 1];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self.0.to_lua()]))
	}
}

impl<T1: ToLua, T2: ToLua> LuaFnReturn for (T1, T2) {
	type Return = [Value; 2];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self.0.to_lua(), self.1.to_lua()]))
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua> LuaFnReturn for (T1, T2, T3) {
	type Return = [Value; 3];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
		]))
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua> LuaFnReturn for (T1, T2, T3, T4) {
	type Return = [Value; 4];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
		]))
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua, T5: ToLua> LuaFnReturn for (T1, T2, T3, T4, T5) {
	type Return = [Value; 5];
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
			self.4.to_lua(),
		]))
	}
}

impl<T: LuaFnReturn> LuaFnReturn for Yield<T> {
	type Return = T::Return;
	type Err = T::Err;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		let (Return::Values(ret) | Return::Yield(ret)) = self.0.lua_fn_return(lua)?;
		Ok(Return::Yield(ret))
	}
}

impl<T: LuaFnReturn> LuaFnReturn for Return<T> {
	type Return = T::Return;
	type Err = T::Err;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		let is_yield = matches!(self, Self::Yield(_));
		let (Self::Values(ret) | Self::Yield(ret)) = self;
		let (Return::Values(ret) | Return::Yield(ret)) = ret.lua_fn_return(lua)?;
		if is_yield {
			Ok(Return::Yield(ret))
		} else {
			Ok(Return::Values(ret))
		}
	}
}

impl<T: LuaFnReturn<Err = Infallible>, E: ToLua> LuaFnReturn for Result<T, E> {
	type Return = T::Return;
	type Err = E;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, E> {
		self?.lua_fn_return(lua).map_err(|err| match err {})
	}
}
