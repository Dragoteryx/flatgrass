use std::marker::PhantomData;
use crate::ffi::*;

pub mod traits; use traits::*;
pub mod value; use value::*;
pub mod error; use error::*;
mod realm; pub use realm::*;
mod gc; pub use gc::*;

#[derive(Clone, Copy)]
pub struct Lua<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState
}

impl<'l> LuaParam for Lua<'l> {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> Lua<'l> {
  pub const unsafe fn from_state(state: LuaState) -> Self {
    Self { phantom: PhantomData, state }
  }

  pub fn realm(&self) -> LuaRealm {
    LuaRealm::Server
  }

  pub fn print(&self, value: impl ToLua) {
    unsafe {
      lua_getglobal(self.state, crate::cstr!("print"));
      ToLua::push(self.state, value);
      lua_call(self.state, 1, 0);
    }
  }

  pub fn error(&self, value: impl ToLua) {
    unsafe {
      ToLua::push(self.state, value);
      lua_error(self.state);
    }
  }
}