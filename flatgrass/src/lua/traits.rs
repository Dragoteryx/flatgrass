use crate::lua::Lua;
use crate::lua::value::LuaValue;

mod from_lua;
mod to_iter;
mod to_lua;

pub mod function;

pub trait FromLua: Sized {
	type Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err>;

	fn no_value() -> Result<Self, Self::Err> {
		Self::from_lua(LuaValue::Nil)
	}
}

pub trait ToLuaIter {
	type LuaIter: IntoIterator<Item: ToLua>;

	fn to_lua_iter(self) -> Self::LuaIter;
}

pub trait ToLua {
	fn to_lua_by_ref(&self) -> LuaValue;

	fn to_lua(self) -> LuaValue
	where
		Self: Sized,
	{
		self.to_lua_by_ref()
	}
}
