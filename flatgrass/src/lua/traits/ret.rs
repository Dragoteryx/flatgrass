use std::convert::Infallible;
use std::fmt::Display;
use super::*;

/// Types that implement this trait can be returned from a Lua function.
/// All types that implement [`PushToLua`](PushToLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaReturn {
  type Error: Display;

  unsafe fn push_return(state: LuaState, value: Self) -> Result<i32, Self::Error>;
}

impl<T: PushManyToLua> LuaReturn for T {
  type Error = Infallible;

  unsafe fn push_return(state: LuaState, value: Self) -> Result<i32, Self::Error> {
    Ok(state.fg_pushmany(value))
  }
}

impl<T: LuaReturn<Error = Infallible>, E: Display> LuaReturn for Result<T, E> {
  type Error = E;

  unsafe fn push_return(state: LuaState, value: Self) -> Result<i32, Self::Error> {
    value.map(|value| LuaReturn::push_return(state, value).unwrap())
  }
}