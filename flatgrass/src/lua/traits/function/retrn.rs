use super::{Lua, ToLua, ToLuaIter};
use std::convert::Infallible;

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Yield<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Return<T, Y = T> {
	Values(T),
	Yield(Y),
}

#[cfg_attr(docsrs, doc(notable_trait))]
pub trait LuaFnReturn: Sized {
	type Values: ToLuaIter;
	type Yield: ToLuaIter;
	type Err: ToLua;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Values, Self::Yield>, Self::Err>;
}

impl<T: ToLuaIter> LuaFnReturn for T {
	type Values = Self;
	type Yield = Infallible;
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Values, Self::Yield>, Self::Err> {
		Ok(Return::Values(self))
	}
}

impl<T: ToLuaIter> LuaFnReturn for Yield<T> {
	type Values = Infallible;
	type Yield = T;
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Return<Self::Values, Self::Yield>, Self::Err> {
		Ok(Return::Yield(self.0))
	}
}

impl<T: ToLuaIter, Y: ToLuaIter> LuaFnReturn for Return<T, Y> {
	type Values = T;
	type Yield = Y;
	type Err = Infallible;

	fn lua_fn_return(self, _: &Lua) -> Result<Self, Self::Err> {
		Ok(self)
	}
}

impl<T: LuaFnReturn<Err = Infallible>, E: ToLua> LuaFnReturn for Result<T, E> {
	type Values = T::Values;
	type Yield = T::Yield;
	type Err = E;

	fn lua_fn_return(self, lua: &Lua) -> Result<Return<Self::Values, Self::Yield>, E> {
		self?.lua_fn_return(lua).map_err(|err| match err {})
	}
}
