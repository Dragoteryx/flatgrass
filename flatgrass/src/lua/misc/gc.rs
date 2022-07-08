use super::*;

#[derive(Clone)]
pub struct LuaGc<'l>(LuaState<'l>);

// lua impls -----------------------

impl<'l> LuaArg<'l> for LuaGc<'l> {
  type Error = Infallible;

  unsafe fn resolve(state: LuaState<'l>, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

// main impl ----------------------

impl<'l> LuaGc<'l> {
  pub unsafe fn from_state(state: LuaState<'l>) -> Self {
    Self(state)
  }

  pub fn stop(&self) {
    unsafe { self.0.lua_gc(LUA_GCSTOP, 0); }
  }

  pub fn restart(&self) {
    unsafe { self.0.lua_gc(LUA_GCRESTART, 0); }
  }

  pub fn collect(&self) {
    unsafe { self.0.lua_gc(LUA_GCCOLLECT, 0); }
  }

  pub fn count_kilobytes(&self) -> usize {
    unsafe { self.0.lua_gc(LUA_GCCOUNT, 0) as _ }
  }

  pub fn count_remainder(&self) -> usize {
    unsafe { self.0.lua_gc(LUA_GCCOUNTB, 0) as _ }
  }
}