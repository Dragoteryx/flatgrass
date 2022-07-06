use super::*;

#[derive(Clone, PartialEq, Eq)]
pub struct LuaGc<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState
}

impl<'l> LuaArg for LuaGc<'l> {
  type Error = Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> LuaGc<'l> {
  pub unsafe fn from_state(state: LuaState) -> Self {
    Self { phantom: PhantomData, state }
  }

  pub fn state(&self) -> LuaState {
    self.state
  }

  pub fn stop(&self) {
    unsafe { self.state.lua_gc(LUA_GCSTOP, 0); }
  }

  pub fn restart(&self) {
    unsafe { self.state.lua_gc(LUA_GCRESTART, 0); }
  }

  pub fn collect(&self) {
    unsafe { self.state.lua_gc(LUA_GCCOLLECT, 0); }
  }

  pub fn count_kilobytes(&self) -> usize {
    unsafe { self.state.lua_gc(LUA_GCCOUNT, 0) as _ }
  }

  pub fn count_remainder(&self) -> usize {
    unsafe { self.state.lua_gc(LUA_GCCOUNTB, 0) as _ }
  }
}