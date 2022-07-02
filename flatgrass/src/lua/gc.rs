use super::*;

#[derive(Clone, Copy)]
pub struct LuaGc<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState
}

impl<'l> LuaParam for LuaGc<'l> {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> LuaGc<'l> {
  pub const unsafe fn from_state(state: LuaState) -> Self {
    Self { phantom: PhantomData, state }
  }

  pub fn stop(&self) {
    unsafe { lua_gc(self.state, LUA_GCSTOP, 0); }
  }

  pub fn restart(&self) {
    unsafe { lua_gc(self.state, LUA_GCRESTART, 0); }
  }

  pub fn collect(&self) {
    unsafe { lua_gc(self.state, LUA_GCCOLLECT, 0); }
  }

  pub fn count_kilobytes(&self) -> usize {
    unsafe { lua_gc(self.state, LUA_GCCOUNT, 0) as _ }
  }

  pub fn count_remainder(&self) -> usize {
    unsafe { lua_gc(self.state, LUA_GCCOUNTB, 0) as _ }
  }
}