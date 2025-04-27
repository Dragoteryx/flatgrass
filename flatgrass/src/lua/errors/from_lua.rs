use super::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FromLuaError<'a> {
	ExpectedAndGot(Cow<'a, str>, Cow<'a, str>),
	Expected(Cow<'a, str>),
	NoValue,
}

impl FromLuaError<'static> {
	pub const fn expected_and_got_type(expected: LuaType, got: LuaType) -> Self {
		Self::ExpectedAndGot(Cow::Borrowed(expected.name()), Cow::Borrowed(got.name()))
	}

	pub const fn expected_type(expected: LuaType) -> Self {
		Self::Expected(Cow::Borrowed(expected.name()))
	}
}

impl<'a> FromLuaError<'a> {
	pub fn expected_and_got<T: ?Sized + AsRef<str>, U: ?Sized + AsRef<str>>(
		expected: &'a T,
		got: &'a U,
	) -> Self {
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
