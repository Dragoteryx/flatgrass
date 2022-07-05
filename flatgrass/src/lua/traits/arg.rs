use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`GetFromLua`](GetFromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaArg: Sized {
  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Self;
}

impl LuaArg for LuaState {
  unsafe fn resolve(state: LuaState, _: &mut i32) -> Self {
    state
  }
}

impl<T: GetFromLua> LuaArg for T {
  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Self {
    match state.fg_getvalue(*narg) {
      Err(err) => state.fg_badarg_error(*narg, err),
      Ok(value) => {
        *narg += 1;
        value
      }
    }
  }
}

impl<T: GetFromLua> LuaArg for Option<T> {
  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Self {
    match state.fg_getvalue(*narg) {
      Err(err) => match state.fg_type(*narg) {
        LuaType::None | LuaType::Nil => {
          *narg += 1;
          None
        }
        _ => state.fg_badarg_error(*narg, err)
      }
      Ok(value) => {
        *narg += 1;
        Some(value)
      }
    }
  }
}