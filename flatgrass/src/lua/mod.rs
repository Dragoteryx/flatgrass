use std::convert::Infallible;
use crate::ffi::*;

mod value; pub use value::*;
pub mod traits; use traits::*;
pub mod errors; use errors::*;
pub mod misc; use misc::*;

#[repr(transparent)]
#[derive(Clone)]
pub struct Lua<'l>(LuaState<'l>);

// lua impls --------------------

impl<'l> LuaArg<'l> for Lua<'l> {
  type Error = Infallible;

  unsafe fn resolve(state: LuaState<'l>, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> LuaReturn for Lua<'l> {
  type Error = Infallible;

  unsafe fn push_return(state: LuaState, _: Self) -> Result<i32, Self::Error> {
    Ok(state.lua_gettop())
  }
}

// main impl ---------------------

impl<'l> Lua<'l> {
  pub unsafe fn from_state(state: LuaState<'l>) -> Self {
    Self(state)
  }

  pub fn push(&self, value: impl PushToLua) {
    unsafe { self.0.fg_pushvalue(value); }
  }

  pub fn gc(&self) -> LuaGc<'l> {
    unsafe { LuaGc::from_state(self.0) }
  }

  pub fn globals(&self) -> Globals<'l> {
    unsafe { Globals::from_state(self.0) }
  }

  pub fn realm(&self) -> Option<Realm> {
    unsafe { Realm::from_state(self.0) }
  }

  pub fn print(&self, values: impl PushManyToLua) -> bool {
    self.globals().get("print")
      .and_then(|pt| pt.try_as::<func::Function>().ok())
      .map(|pt| pt.call(values).is_ok())
      .unwrap_or_default()
  }

  pub fn curtime(&self) -> Option<f64> {
    self.globals().get("CurTime")
      .and_then(|ct| ct.try_as::<func::Function>().ok())
      .and_then(|ct| ct.call(()).ok())
      .and_then(|mut ct| ct.pop_front())
      .and_then(|ct| ct.try_as().ok())
  }
}

