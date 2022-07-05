use std::marker::PhantomData;
use crate::ffi::*;

pub mod traits; use traits::*;
pub mod value; use value::*;

mod globals; pub use globals::*;
mod typ; pub use typ::*;
mod gc; pub use gc::*;

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct Lua<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState
}

impl<'l> LuaArg for Lua<'l> {
  unsafe fn resolve(state: LuaState, _: &mut i32) -> Self {
    Self::from_state(state)
  }
}

impl<'l> Lua<'l> {
  pub unsafe fn from_state(state: LuaState) -> Self {
    Self { phantom: PhantomData, state }
  }

  pub fn state(&self) -> LuaState {
    self.state
  }

  pub fn globals(&self) -> LuaGlobals<'l> {
    unsafe { LuaGlobals::from_state(self.state) }
  }

  pub fn gc(&self) -> LuaGc<'l> {
    unsafe { LuaGc::from_state(self.state) }
  }

  pub fn print(&self, value: impl PushToLua) {
    unsafe { self.state.fg_print(value); }
  }

  pub fn error(&self, error: impl PushToLua) -> ! {
    unsafe { self.state.fg_error(error); }
  }

  pub fn realm(&self) -> LuaRealm {
    unsafe {
      self.state.fg_checkstack(3);
      self.state.lua_getglobal(crate::cstr!("SERVER"));
      self.state.lua_getglobal(crate::cstr!("CLIENT"));
      self.state.lua_getglobal(crate::cstr!("MENU"));
      let server = self.state.lua_toboolean(-3) != 0;
      let client = self.state.lua_toboolean(-2) != 0;
      let menu = self.state.lua_toboolean(-1) != 0;
      self.state.lua_pop(3);
      match (server, client, menu) {
        (true, false, false) => LuaRealm::Server,
        (false, true, false) => LuaRealm::Client,
        (false, false, true) => LuaRealm::Menu,
        _ => self.error("invalid realm")
      }
    }
  }

  pub fn curtime(&self) -> f64 {
    unsafe {
      self.state.fg_checkstack(1);
      self.state.lua_getglobal(crate::cstr!("CurTime"));
      self.state.lua_call(0, 1);
      let n = self.state.lua_tonumber(-1);
      self.state.lua_pop(1);
      n
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaRealm {
  Server,
  Client,
  Menu
}