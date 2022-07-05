use std::marker::PhantomData;
use std::convert::Infallible;
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
  type Error = Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> Lua<'l> {
  pub unsafe fn from_state(state: LuaState) -> Self {
    Self { phantom: PhantomData, state }
  }

  pub fn gc(&self) -> LuaGc<'l> {
    unsafe { LuaGc::from_state(self.state) }
  }

  pub fn globals(&self) -> LuaGlobals<'l> {
    unsafe { LuaGlobals::from_state(self.state) }
  }

  pub fn realm(&self) -> LuaRealm {
    let globals = self.globals();
    let server = globals.get("SERVER").unwrap().try_as().unwrap();
    let client = globals.get("CLIENT").unwrap().try_as().unwrap();
    let menu = globals.get("MENU").unwrap().try_as().unwrap();
    match (server, client, menu) {
      (true, false, false) => LuaRealm::Server,
      (false, true, false) => LuaRealm::Client,
      (false, false, true) => LuaRealm::Menu,
      _ => unreachable!()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaRealm {
  Server,
  Client,
  Menu
}