use libc::c_void;
use std::fmt;
use super::*;

mod func; pub use func::*;
mod table; pub use table::*;
mod globals; pub use globals::*;

pub struct LuaValue<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState,
  lref: i32
}

impl<'l> PushToLua for &LuaValue<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.lua_rawgeti(LUA_ENVIRONINDEX, value.lref);
  }
}

impl<'l> PushToLua for LuaValue<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value);
  }
}

impl<'l> Clone for LuaValue<'l> {
  fn clone(&self) -> Self {
    unsafe {
      self.state.fg_checkstack(1);
      self.state.fg_pushvalue(self);
      Self::pop(self.state)
    }
  }
}

impl<'l> Drop for LuaValue<'l> {
  fn drop(&mut self) {
    unsafe { self.state.luaL_unref(LUA_ENVIRONINDEX, self.lref); }
  }
}

impl<'l> PartialEq for LuaValue<'l> {
  fn eq(&self, other: &Self) -> bool {
    unsafe {
      
      self.state.fg_checkstack(2);
      self.state.fg_pushvalue(self);
      self.state.fg_pushvalue(other);
      let eq = self.state.lua_rawequal(-1, -2);
      self.state.lua_pop(2);
      eq != 0
    }
  }
}

impl<'l> fmt::Debug for LuaValue<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.get_type() {
      LuaType::Nil => write!(f, "nil"),
      LuaType::Boolean => self.try_as::<bool>().unwrap().fmt(f),
      LuaType::Number => self.try_as::<f64>().unwrap().fmt(f),
      LuaType::String => self.try_as::<String>().unwrap().fmt(f),
      LuaType::Table => self.try_as::<Table>().unwrap().fmt(f),
      LuaType::Function => self.try_as::<Function>().unwrap().fmt(f),
      LuaType::Userdata => write!(f, "userdata: {:p}", self.pointer()),
      LuaType::Thread => write!(f, "thread: {:p}", self.pointer()),
      LuaType::LightUserdata => write!(f, "lightuserdata: {:p}", self.pointer()),
      LuaType::None => unreachable!()
    }   
  }
}

impl<'l> LuaValue<'l> {
  /// Pops the value at the top of the stack and returns a LuaValue.
  /// # Safety
  /// The stack must not be empty.
  pub unsafe fn pop(state: LuaState) -> Self {
    Self {
      phantom: PhantomData, state,
      lref: state.luaL_ref(LUA_ENVIRONINDEX)
    }
  }

  pub fn new(lua: &Lua<'l>, value: impl PushToLua) -> Self {
    unsafe {
      lua.state.fg_checkstack(1);
      lua.state.fg_pushvalue(value);
      Self::pop(lua.state)
    }
  }

  pub fn pointer(&self) -> *const c_void {
    unsafe {
      self.state.fg_checkstack(1);
      self.state.fg_pushvalue(self);
      let ptr = self.state.lua_topointer(-1);
      self.state.lua_pop(1);
      ptr
    }
  }

  pub fn get_type(&self) -> LuaType {
    unsafe {
      self.state.fg_checkstack(1);
      self.state.fg_pushvalue(self);
      let t = self.state.fg_type(-1);
      self.state.lua_pop(1);
      t
    }
  }

  pub fn try_as<T: GetFromLua>(&self) -> Result<T, T::Error> {
    unsafe {
      self.state.fg_checkstack(1);
      self.state.fg_pushvalue(self);
      let value = self.state.fg_getvalue(-1);
      self.state.lua_pop(1);
      value
    }
  }
}