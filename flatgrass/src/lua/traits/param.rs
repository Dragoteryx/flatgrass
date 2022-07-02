use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`FromLua`](FromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaParam: Sized {
  type Error: ToLua;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error>;
}

impl LuaParam for LuaState {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(state)
  }
}

impl<T: FromLua> LuaParam for T {
  type Error = T::Error;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error> {
    todo!()
  }
}

impl<T: FromLua> LuaParam for Option<T> {
  type Error = T::Error;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error> {
    todo!()
  }
}