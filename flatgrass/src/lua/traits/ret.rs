use std::convert::Infallible;
use std::fmt::Display;
use super::*;

/// Types that implement this trait can be returned from a Lua function.
/// All types that implement [`PushToLua`](PushToLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaReturn<'l> {
  type Error: Display;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error>;
}

// misc types -------------------------

impl<'l, T: LuaReturn<'l, Error = Infallible>, E: Display> LuaReturn<'l> for Result<T, E> {
  type Error = E;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    value.map(|value| LuaReturn::push(state, value).unwrap())
  }
}

// tuple types ------------------------

impl<'l, T: PushToLua<'l>> LuaReturn<'l> for T {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(1);
    state.fg_pushvalue(value);
    Ok(1)
  }
}

impl<'l, A: PushToLua<'l>> LuaReturn<'l> for (A,) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(1);
    state.fg_pushvalue(value.0);
    Ok(1)
  }
}

impl<'l, A: PushToLua<'l>, B: PushToLua<'l>> LuaReturn<'l> for (A, B) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(2);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    Ok(2)
  }
}

impl<'l, A: PushToLua<'l>, B: PushToLua<'l>, C: PushToLua<'l>> LuaReturn<'l> for (A, B, C) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(3);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    Ok(3)
  }
}

impl<'l, A: PushToLua<'l>, B: PushToLua<'l>, C: PushToLua<'l>, D: PushToLua<'l>> LuaReturn<'l> for (A, B, C, D) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(4);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
    Ok(4)
  }
}

impl<'l, A: PushToLua<'l>, B: PushToLua<'l>, C: PushToLua<'l>, D: PushToLua<'l>, E: PushToLua<'l>> LuaReturn<'l> for (A, B, C, D, E) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(5);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
    state.fg_pushvalue(value.4);
    Ok(5)
  }
}

impl<'l, A: PushToLua<'l>, B: PushToLua<'l>, C: PushToLua<'l>, D: PushToLua<'l>, E: PushToLua<'l>, F: PushToLua<'l>> LuaReturn<'l> for (A, B, C, D, E, F) {
  type Error = Infallible;

  unsafe fn push(state: LuaState<'l>, value: Self) -> Result<i32, Self::Error> {
    state.fg_checkstack(6);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
    state.fg_pushvalue(value.4);
    state.fg_pushvalue(value.5);
    Ok(6)
  }
}