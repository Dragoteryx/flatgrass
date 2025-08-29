use crate::lua::value::{LightUserdata, LuaString, LuaType, LuaValue};
use std::borrow::Cow;
use std::convert::Infallible;
use std::error::Error;
use std::ffi::CString;
use std::fmt::{self, Display};
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "either")]
use either::Either;

pub trait FromLua: Sized {
	type Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err>;

	fn no_value() -> Result<Self, Self::Err> {
		Self::from_lua(LuaValue::Nil)
	}
}

impl<T: FromLua> FromLua for Box<T> {
	type Err = T::Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		T::from_lua(value).map(Self::new)
	}

	fn no_value() -> Result<Self, Self::Err> {
		T::no_value().map(Self::new)
	}
}

impl<T: FromLua> FromLua for Rc<T> {
	type Err = T::Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		T::from_lua(value).map(Self::new)
	}

	fn no_value() -> Result<Self, Self::Err> {
		T::no_value().map(Self::new)
	}
}

impl<T: FromLua> FromLua for Arc<T> {
	type Err = T::Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		T::from_lua(value).map(Self::new)
	}

	fn no_value() -> Result<Self, Self::Err> {
		T::no_value().map(Self::new)
	}
}

impl<T: FromLua> FromLua for Option<T> {
	type Err = T::Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		match (value.is_nil(), T::from_lua(value)) {
			(false, Err(err)) => Err(err),
			(true, Err(_)) => Ok(None),
			(_, Ok(ok)) => Ok(Some(ok)),
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		match T::no_value() {
			Ok(ok) => Ok(Some(ok)),
			Err(_) => Ok(None),
		}
	}
}

#[cfg(feature = "either")]
impl<L: FromLua, R: FromLua> FromLua for Either<L, R> {
	type Err = R::Err;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		match L::from_lua(value.clone()) {
			Ok(ok) => Ok(Self::Left(ok)),
			Err(_) => match R::from_lua(value) {
				Ok(ok) => Ok(Self::Right(ok)),
				Err(err) => Err(err),
			},
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		match L::no_value() {
			Ok(ok) => Ok(Self::Left(ok)),
			Err(_) => match R::no_value() {
				Ok(ok) => Ok(Self::Right(ok)),
				Err(err) => Err(err),
			},
		}
	}
}

impl<T: FromLua> FromLua for Result<T, T::Err> {
	type Err = Infallible;

	fn from_lua(value: LuaValue) -> Result<Self, Infallible> {
		Ok(T::from_lua(value))
	}

	fn no_value() -> Result<Self, Infallible> {
		Ok(T::no_value())
	}
}

impl FromLua for bool {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Bool(bl) = value {
			Ok(bl)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Bool,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Bool))
	}
}

impl FromLua for f32 {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Number(num) = value {
			Ok(num as Self)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Number,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Number))
	}
}

impl FromLua for f64 {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Number(num) = value {
			Ok(num)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Number,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Number))
	}
}

impl FromLua for LightUserdata {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::LightUserdata(lud) = value {
			Ok(lud)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::LightUserdata,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::LightUserdata))
	}
}

impl FromLua for String {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		LuaString::from_lua(value).map(|lstr| lstr.to_string())
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::String))
	}
}

impl FromLua for CString {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		LuaString::from_lua(value).map(|lstr| lstr.to_c_str().to_owned())
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::String))
	}
}

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
