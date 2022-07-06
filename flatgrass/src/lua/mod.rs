use std::marker::PhantomData;
use std::convert::Infallible;
use crate::ffi::*;

pub mod errors; use errors::*;
pub mod traits; use traits::*;
pub mod util; use util::*;

mod value; pub use value::*;
mod typ; pub use typ::*;

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

  pub fn globals(&self) -> Globals<'l> {
    unsafe { Globals::from_state(self.state) }
  }

  pub fn realm(&self) -> Realm {
    let globals = self.globals();
    let server = globals.get("SERVER").unwrap().try_as().unwrap();
    let client = globals.get("CLIENT").unwrap().try_as().unwrap();
    let menu = globals.get("MENU").unwrap().try_as().unwrap();
    match (server, client, menu) {
      (true, false, false) => Realm::Server,
      (false, true, false) => Realm::Client,
      (false, false, true) => Realm::Menu,
      _ => unreachable!()
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Realm {
  Server,
  Client,
  Menu
}