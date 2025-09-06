use crate::lua::Lua;
use crate::lua::traits::ToLua;
use crate::lua::value::Value;
use std::error::Error;
use std::fmt::{self, Display};

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LuaError<T> {
	location: String,
	source: T,
}

impl<T> LuaError<T> {
	pub fn new(source: T) -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_location(1);
			Self {
				location: stack.pop_lua_string_unchecked().to_string(),
				source,
			}
		})
	}

	pub fn location(&self) -> &str {
		&self.location
	}

	pub fn source(&self) -> &T {
		&self.source
	}
}

impl<T: Error + 'static> Error for LuaError<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.source)
	}
}

impl<T: ToString> Display for LuaError<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.location, self.source.to_string())
	}
}

impl<T: ToString> ToLua for LuaError<T> {
	fn to_lua_by_ref(&self) -> Value {
		self.to_string().to_lua()
	}
}
