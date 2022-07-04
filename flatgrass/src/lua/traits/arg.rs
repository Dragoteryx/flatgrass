use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`FromLua`](FromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaArg: Sized {
  type Error: PushToLua;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error>;
}

impl LuaArg for LuaState {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(state)
  }
}

impl<T: LuaArg> LuaArg for Option<T> {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error> {
    Ok(T::resolve(state, idx).ok())
  }
}

/*impl<T: FromLua<Error = LuaError>> LuaArg for T {
  type Error = LuaError;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error> {
    if lua_isnone(state, *idx) != 0 {
      Err(LuaError::NoValue)
    } else {
      lua_pushvalue(state, *idx);
      *idx += 1;
      FromLua::pop(state)
    }
  }
}*/