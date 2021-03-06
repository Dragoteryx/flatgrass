use std::fmt::Display;
use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`GetFromLua`](GetFromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaArg<'l>: Sized {
  type Error: Display;

  unsafe fn resolve(state: LuaState<'l>, narg: &mut i32) -> Result<Self, Self::Error>;
}

impl<'l, T: GetFromLua<'l>> LuaArg<'l> for T {
  type Error = BadArgError<'l, T>;

  unsafe fn resolve(state: LuaState<'l>, narg: &mut i32) -> Result<Self, Self::Error> {
    match state.fg_getvalue(*narg) {
      Err(err) => Err(BadArgError::new(state, *narg, err)),
      Ok(value) => {
        *narg += 1;
        Ok(value)
      }
    }
  }
}

impl<'l, T: GetFromLua<'l>> LuaArg<'l> for Option<T> {
  type Error = BadArgError<'l, T>;

  unsafe fn resolve(state: LuaState<'l>, narg: &mut i32) -> Result<Self, Self::Error> {
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

