use crate::lua::traits::ToLua;
use crate::lua::value::Value;
use std::iter::Chain;
use std::iter::{Empty, empty};
use std::iter::{Once, once};

pub trait ToLuaMany {
	type LuaMany: Iterator<Item = Value>;

	fn to_lua_many(self) -> Self::LuaMany;
}

impl<T: ToLua> ToLuaMany for T {
	type LuaMany = Once<Value>;

	fn to_lua_many(self) -> Self::LuaMany {
		once(self.to_lua())
	}
}

impl ToLuaMany for () {
	type LuaMany = Empty<Value>;

	fn to_lua_many(self) -> Self::LuaMany {
		empty()
	}
}

impl<T: ToLuaMany> ToLuaMany for (T,) {
	type LuaMany = T::LuaMany;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0.to_lua_many()
	}
}

impl<T1: ToLuaMany, T2: ToLuaMany> ToLuaMany for (T1, T2) {
	type LuaMany = Chain<T1::LuaMany, T2::LuaMany>;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0.to_lua_many().chain(self.1.to_lua_many())
	}
}

impl<T1: ToLuaMany, T2: ToLuaMany, T3: ToLuaMany> ToLuaMany for (T1, T2, T3) {
	type LuaMany = Chain<Chain<T1::LuaMany, T2::LuaMany>, T3::LuaMany>;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0
			.to_lua_many()
			.chain(self.1.to_lua_many())
			.chain(self.2.to_lua_many())
	}
}

impl<T1, T2, T3, T4> ToLuaMany for (T1, T2, T3, T4)
where
	T1: ToLuaMany,
	T2: ToLuaMany,
	T3: ToLuaMany,
	T4: ToLuaMany,
{
	type LuaMany = Chain<Chain<Chain<T1::LuaMany, T2::LuaMany>, T3::LuaMany>, T4::LuaMany>;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0
			.to_lua_many()
			.chain(self.1.to_lua_many())
			.chain(self.2.to_lua_many())
			.chain(self.3.to_lua_many())
	}
}

impl<T1, T2, T3, T4, T5> ToLuaMany for (T1, T2, T3, T4, T5)
where
	T1: ToLuaMany,
	T2: ToLuaMany,
	T3: ToLuaMany,
	T4: ToLuaMany,
	T5: ToLuaMany,
{
	type LuaMany =
		Chain<Chain<Chain<Chain<T1::LuaMany, T2::LuaMany>, T3::LuaMany>, T4::LuaMany>, T5::LuaMany>;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0
			.to_lua_many()
			.chain(self.1.to_lua_many())
			.chain(self.2.to_lua_many())
			.chain(self.3.to_lua_many())
			.chain(self.4.to_lua_many())
	}
}

impl<T1, T2, T3, T4, T5, T6> ToLuaMany for (T1, T2, T3, T4, T5, T6)
where
	T1: ToLuaMany,
	T2: ToLuaMany,
	T3: ToLuaMany,
	T4: ToLuaMany,
	T5: ToLuaMany,
	T6: ToLuaMany,
{
	type LuaMany = Chain<
		Chain<Chain<Chain<Chain<T1::LuaMany, T2::LuaMany>, T3::LuaMany>, T4::LuaMany>, T5::LuaMany>,
		T6::LuaMany,
	>;

	fn to_lua_many(self) -> Self::LuaMany {
		self.0
			.to_lua_many()
			.chain(self.1.to_lua_many())
			.chain(self.2.to_lua_many())
			.chain(self.3.to_lua_many())
			.chain(self.4.to_lua_many())
			.chain(self.5.to_lua_many())
	}
}
