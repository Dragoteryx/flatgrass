use crate::lua::traits::ToLua;
use crate::lua::value::LuaValue;

pub trait ToLuaIter {
	type LuaIter: IntoIterator<Item: ToLua>;

	fn to_lua_iter(self) -> Self::LuaIter;
}

impl<T: ToLua> ToLuaIter for T {
	type LuaIter = [T; 1];

	fn to_lua_iter(self) -> Self::LuaIter {
		[self]
	}
}

impl ToLuaIter for () {
	type LuaIter = [LuaValue; 0];

	fn to_lua_iter(self) -> Self::LuaIter {
		[]
	}
}

impl<T: ToLua> ToLuaIter for (T,) {
	type LuaIter = [LuaValue; 1];

	fn to_lua_iter(self) -> Self::LuaIter {
		[self.0.to_lua()]
	}
}

impl<T1: ToLua, T2: ToLua> ToLuaIter for (T1, T2) {
	type LuaIter = [LuaValue; 2];

	fn to_lua_iter(self) -> Self::LuaIter {
		[self.0.to_lua(), self.1.to_lua()]
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua> ToLuaIter for (T1, T2, T3) {
	type LuaIter = [LuaValue; 3];

	fn to_lua_iter(self) -> Self::LuaIter {
		[self.0.to_lua(), self.1.to_lua(), self.2.to_lua()]
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua> ToLuaIter for (T1, T2, T3, T4) {
	type LuaIter = [LuaValue; 4];

	fn to_lua_iter(self) -> Self::LuaIter {
		[
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
		]
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua, T5: ToLua> ToLuaIter for (T1, T2, T3, T4, T5) {
	type LuaIter = [LuaValue; 5];

	fn to_lua_iter(self) -> Self::LuaIter {
		[
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
			self.4.to_lua(),
		]
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua, T5: ToLua, T6: ToLua> ToLuaIter
	for (T1, T2, T3, T4, T5, T6)
{
	type LuaIter = [LuaValue; 6];

	fn to_lua_iter(self) -> Self::LuaIter {
		[
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
			self.4.to_lua(),
			self.5.to_lua(),
		]
	}
}
