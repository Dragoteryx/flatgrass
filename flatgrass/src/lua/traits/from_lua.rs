use crate::lua::errors::FromLuaError;
use crate::lua::value::{LightUserdata, LuaString, LuaType, LuaValue};
use std::convert::Infallible;
use std::ffi::CString;
use std::rc::Rc;
use std::sync::Arc;
use super::FromLua;

#[cfg(feature = "either")]
use either::Either;

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
