use crate::ffi;
use crate::lua::{Lua, ToLua, Type, Value};
use std::borrow::Cow;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Debug, Display};

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
			let location = stack.pop_lua_string_unchecked().to_string();
			Self { location, source }
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BadArgError<T> {
	name: Option<String>,
	arg: i32,
	source: T,
}

impl<T> BadArgError<T> {
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

impl<T: Error + 'static> Error for BadArgError<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.source)
	}
}

impl<T: ToString> Display for BadArgError<T> {
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FromLuaError<'a> {
	ExpectedAndGot(Cow<'a, str>, Cow<'a, str>),
	Expected(Cow<'a, str>),
	NoValue,
}

impl FromLuaError<'static> {
	pub const fn expected_and_got_type(expected: Type, got: Type) -> Self {
		Self::ExpectedAndGot(Cow::Borrowed(expected.name()), Cow::Borrowed(got.name()))
	}

	pub const fn expected_type(expected: Type) -> Self {
		Self::Expected(Cow::Borrowed(expected.name()))
	}
}

impl<'a> FromLuaError<'a> {
	pub fn expected_and_got<T, U>(expected: &'a T, got: &'a U) -> Self
	where
		T: ?Sized + AsRef<str>,
		U: ?Sized + AsRef<str>,
	{
		Self::ExpectedAndGot(
			Cow::Borrowed(expected.as_ref()),
			Cow::Borrowed(got.as_ref()),
		)
	}

	pub fn expected<T: ?Sized + AsRef<str>>(expected: &'a T) -> Self {
		Self::Expected(Cow::Borrowed(expected.as_ref()))
	}

	pub const fn no_value() -> Self {
		Self::NoValue
	}
}

impl Error for FromLuaError<'_> {}
impl Display for FromLuaError<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::ExpectedAndGot(expected, got) => write!(f, "{expected} expected, got {got}"),
			Self::Expected(expected) => write!(f, "{expected} expected, got no value"),
			Self::NoValue => write!(f, "got no value"),
		}
	}
}
