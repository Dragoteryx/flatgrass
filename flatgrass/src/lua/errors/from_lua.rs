use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FromLuaError<'a> {
	ExpectedAndGot(Cow<'a, str>, Cow<'a, str>),
	Expected(Cow<'a, str>),
	NoValue,
}

impl<'a> FromLuaError<'a> {
	pub const fn expected_and_got(expected: LuaType, got: LuaType) -> Self {
		Self::ExpectedAndGot(Cow::Borrowed(expected.name()), Cow::Borrowed(got.name()))
	}

	pub const fn expected(expected: LuaType) -> Self {
		Self::Expected(Cow::Borrowed(expected.name()))
	}

	pub const fn no_value() -> Self {
		Self::NoValue
	}
}

impl<'a> Error for FromLuaError<'a> {}
impl<'a> Display for FromLuaError<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::ExpectedAndGot(expected, got) => write!(f, "{expected} expected, got {got}"),
			Self::Expected(expected) => write!(f, "{expected} expected, got no value"),
			Self::NoValue => write!(f, "got no value"),
		}
	}
}
