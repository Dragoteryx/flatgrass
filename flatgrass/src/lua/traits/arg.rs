use std::fmt;
use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`GetFromLua`](GetFromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaArg: Sized {
  type Error: fmt::Display;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error>;
}

impl<T: GetFromLua> LuaArg for T {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    match state.fg_getvalue(*narg) {
      Err(err) => Err(BadArgError::new(state, *narg, err)),
      Ok(value) => {
        *narg += 1;
        Ok(value)
      }
    }
  }
}

impl<T: GetFromLua> LuaArg for Option<T> {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    match state.fg_getvalue(*narg) {
      Err(err) => match state.fg_type(*narg) {
        LuaType::None | LuaType::Nil => {
          *narg += 1;
          Ok(None)
        }
        _ => Err(BadArgError::new(state, *narg, err))
      }
      Ok(value) => {
        *narg += 1;
        Ok(Some(value))
      }
    }
  }
}

