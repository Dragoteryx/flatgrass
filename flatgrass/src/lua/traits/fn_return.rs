use crate::lua::util::{Return, Tuple, Yield};
use crate::lua::{Lua, ToLua, Value};
use std::convert::Infallible;

pub trait FnReturn: Sized {
	type Return: IntoIterator<Item: ToLua>;
	type Err: ToLua;

	fn fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err>;
}

impl<T: ToLua> FnReturn for T {
	type Return = [T; 1];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self]))
	}
}

impl FnReturn for () {
	type Return = [Value; 0];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([]))
	}
}

impl<T1: ToLua> FnReturn for (T1,) {
	type Return = [Value; 1];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self.0.to_lua()]))
	}
}

impl<T1: ToLua, T2: ToLua> FnReturn for (T1, T2) {
	type Return = [Value; 2];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([self.0.to_lua(), self.1.to_lua()]))
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua> FnReturn for (T1, T2, T3) {
	type Return = [Value; 3];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
		]))
	}
}

impl<T1: ToLua, T2: ToLua, T3: ToLua, T4: ToLua> FnReturn for (T1, T2, T3, T4) {
	type Return = [Value; 4];
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values([
			self.0.to_lua(),
			self.1.to_lua(),
			self.2.to_lua(),
			self.3.to_lua(),
		]))
	}
}

impl<T: ToLua> FnReturn for Tuple<T> {
	type Return = Self;
	type Err = Infallible;

	fn fn_return(self, _: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		Ok(Return::Values(self))
	}
}

impl<T: FnReturn> FnReturn for Yield<T> {
	type Return = T::Return;
	type Err = T::Err;

	fn fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		let (Return::Values(ret) | Return::Yield(ret)) = self.0.fn_return(lua)?;
		Ok(Return::Yield(ret))
	}
}

impl<T: FnReturn> FnReturn for Return<T> {
	type Return = T::Return;
	type Err = T::Err;

	fn fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, Self::Err> {
		let is_yield = matches!(self, Self::Yield(_));
		let (Self::Values(ret) | Self::Yield(ret)) = self;
		let (Return::Values(ret) | Return::Yield(ret)) = ret.fn_return(lua)?;
		if is_yield {
			Ok(Return::Yield(ret))
		} else {
			Ok(Return::Values(ret))
		}
	}
}

impl<T: FnReturn<Err = Infallible>, E: ToLua> FnReturn for Result<T, E> {
	type Return = T::Return;
	type Err = E;

	fn fn_return(self, lua: &Lua) -> Result<Return<Self::Return>, E> {
		self?.fn_return(lua).map_err(|err| match err {})
	}
}
