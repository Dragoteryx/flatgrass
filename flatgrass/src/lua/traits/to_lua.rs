use crate::lua::value::{LuaString, LuaValue, Table};
use std::collections::*;
use std::convert::Infallible;
use std::ffi::*;
use std::marker::{PhantomData, PhantomPinned};
use std::num::NonZero;
use std::rc::Rc;
use std::sync::Arc;

#[cfg(feature = "either")]
use either::Either;

#[cfg(feature = "macros")]
pub use flatgrass_macros::ToLua;

pub trait ToLua {
	fn to_lua_by_ref(&self) -> LuaValue;

	fn to_lua(self) -> LuaValue
	where
		Self: Sized,
	{
		self.to_lua_by_ref()
	}
}

/// Implements the ToLua trait for number types.
macro_rules! impl_tolua_num {
	($num:ty) => {
		impl ToLua for $num {
			fn to_lua_by_ref(&self) -> LuaValue {
				LuaValue::Number(*self as f64)
			}
		}
	};
}

/// Implements the ToLua trait for integer types.
macro_rules! impl_tolua_int {
	($int:ty) => {
		impl_tolua_num!($int);

		impl ToLua for NonZero<$int> {
			fn to_lua_by_ref(&self) -> LuaValue {
				self.get().to_lua()
			}
		}
	};
}

impl_tolua_num!(f32);
impl_tolua_num!(f64);
impl_tolua_int!(i8);
impl_tolua_int!(i16);
impl_tolua_int!(i32);
impl_tolua_int!(i64);
impl_tolua_int!(i128);
impl_tolua_int!(isize);
impl_tolua_int!(u8);
impl_tolua_int!(u16);
impl_tolua_int!(u32);
impl_tolua_int!(u64);
impl_tolua_int!(u128);
impl_tolua_int!(usize);

impl ToLua for Infallible {
	fn to_lua_by_ref(&self) -> LuaValue {
		match *self {}
	}
}

impl ToLua for PhantomPinned {
	fn to_lua_by_ref(&self) -> LuaValue {
		LuaValue::Nil
	}
}

impl<T> ToLua for PhantomData<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		LuaValue::Nil
	}
}

impl<T: ?Sized + ToLua> ToLua for &T {
	fn to_lua_by_ref(&self) -> LuaValue {
		T::to_lua_by_ref(self)
	}
}

impl<T: ?Sized + ToLua> ToLua for &mut T {
	fn to_lua_by_ref(&self) -> LuaValue {
		T::to_lua_by_ref(self)
	}
}

impl<T: ToLua> ToLua for Box<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		T::to_lua_by_ref(self)
	}

	fn to_lua(self) -> LuaValue {
		T::to_lua(*self)
	}
}

impl<T: ToLua> ToLua for Rc<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		T::to_lua_by_ref(self)
	}

	fn to_lua(self) -> LuaValue {
		match Self::try_unwrap(self) {
			Ok(value) => value.to_lua(),
			Err(rc) => rc.to_lua_by_ref(),
		}
	}
}

impl<T: ToLua> ToLua for Arc<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		T::to_lua_by_ref(self)
	}

	fn to_lua(self) -> LuaValue {
		match Self::try_unwrap(self) {
			Ok(value) => value.to_lua(),
			Err(arc) => arc.to_lua_by_ref(),
		}
	}
}

impl<T: ToLua> ToLua for Option<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_ref().map(T::to_lua_by_ref).unwrap_or_default()
	}

	fn to_lua(self) -> LuaValue {
		self.map(T::to_lua).unwrap_or_default()
	}
}

#[cfg(feature = "either")]
impl<L: ToLua, R: ToLua> ToLua for Either<L, R> {
	fn to_lua_by_ref(&self) -> LuaValue {
		match self {
			Self::Left(left) => left.to_lua_by_ref(),
			Self::Right(right) => right.to_lua_by_ref(),
		}
	}

	fn to_lua(self) -> LuaValue {
		match self {
			Self::Left(left) => left.to_lua(),
			Self::Right(right) => right.to_lua(),
		}
	}
}

impl ToLua for bool {
	fn to_lua_by_ref(&self) -> LuaValue {
		LuaValue::Bool(*self)
	}
}

impl ToLua for str {
	fn to_lua_by_ref(&self) -> LuaValue {
		LuaString::from(self).to_lua()
	}
}

impl ToLua for String {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_str().to_lua()
	}
}

impl ToLua for CStr {
	fn to_lua_by_ref(&self) -> LuaValue {
		LuaString::from(self).to_lua()
	}
}

impl ToLua for CString {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_c_str().to_lua()
	}
}

impl<T: ToLua> ToLua for [T] {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter().collect::<Table>().to_lua()
	}
}

impl<const N: usize, T: ToLua> ToLua for [T; N] {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_slice().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<T: ToLua> ToLua for Vec<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_slice().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<T: ToLua> ToLua for VecDeque<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter().collect::<Table>().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<T: ToLua> ToLua for LinkedList<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter().collect::<Table>().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<T: ToLua> ToLua for HashSet<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter()
			.map(|key| (key, true))
			.collect::<Table>()
			.to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter()
			.map(|key| (key, true))
			.collect::<Table>()
			.to_lua()
	}
}

impl<T: ToLua> ToLua for BTreeSet<T> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter()
			.map(|key| (key, true))
			.collect::<Table>()
			.to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter()
			.map(|key| (key, true))
			.collect::<Table>()
			.to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for [(K, V)] {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter()
			.map(|(key, value)| (key, value))
			.collect::<Table>()
			.to_lua()
	}
}

impl<const N: usize, K: ToLua, V: ToLua> ToLua for [(K, V); N] {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_slice().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for Vec<(K, V)> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.as_slice().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for VecDeque<(K, V)> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter()
			.map(|(key, value)| (key, value))
			.collect::<Table>()
			.to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for LinkedList<(K, V)> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter()
			.map(|(key, value)| (key, value))
			.collect::<Table>()
			.to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for HashMap<K, V> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter().collect::<Table>().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}

impl<K: ToLua, V: ToLua> ToLua for BTreeMap<K, V> {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.iter().collect::<Table>().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		self.into_iter().collect::<Table>().to_lua()
	}
}
