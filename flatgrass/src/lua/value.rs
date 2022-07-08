use libc::c_void;
use std::fmt;
use super::*;

pub mod func; use func::*;
pub mod table; use table::*;
pub use table::Globals;

pub struct LuaValue<'l> {
  state: LuaState<'l>,
  lref: i32
}

// misc impls ------------------------------

impl<'l> Clone for LuaValue<'l> {
  fn clone(&self) -> Self {
    unsafe { Self::from_state(self.state, self) }
  }
}

impl<'l> Drop for LuaValue<'l> {
  fn drop(&mut self) {
    unsafe { self.state.luaL_unref(LUA_ENVIRONINDEX, self.lref); }
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
      LuaType::Userdata => write!(f, "Userdata ({:p})", self.pointer()),
      LuaType::Thread => write!(f, "Thread ({:p})", self.pointer()),
      LuaType::LightUserdata => write!(f, "LightUserdata ({:p})", self.pointer()),
      LuaType::None => unreachable!()
    }   
  }
}

// lua impls --------------------------------

impl<'l> PushToLua for &LuaValue<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(1);
    state.lua_rawgeti(LUA_ENVIRONINDEX, value.lref);
  }
}

impl<'l> PushToLua for LuaValue<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value);
  }
}

impl<'l> GetFromLua<'l> for LuaValue<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState<'l>, idx: i32) -> Result<Self, Self::Error> {
    if state.fg_type(idx) == LuaType::None {
      Err(GetFromLuaError::NoValue)
    } else {
      state.fg_pushindex(idx);
      Ok(Self::pop(state))
    }
  }
}

// main impl -------------------------

impl<'l> LuaValue<'l> {
  pub unsafe fn pop(state: LuaState<'l>) -> Self {
    Self { state, lref: state.luaL_ref(LUA_ENVIRONINDEX) }
  }

  pub unsafe fn from_state(state: LuaState<'l>, value: impl PushToLua) -> Self {
    state.fg_pushvalue(value);
    Self::pop(state)
  }

  pub fn new(lua: &Lua<'l>, value: impl PushToLua) -> Self {
    unsafe { Self::from_state(lua.0, value) }
  }

  pub fn pointer(&self) -> *const c_void {
    unsafe {
      let state = self.state;
      state.fg_pushvalue(self);
      let ptr = state.lua_topointer(-1);
      state.lua_pop(1);
      ptr
    }
  }

  pub fn get_type(&self) -> LuaType {
    unsafe {
      let state = self.state;
      state.fg_pushvalue(self);
      let t = state.fg_type(-1);
      state.lua_pop(1);
      t
    }
  }

  pub fn try_as<T: GetFromLua<'l>>(&self) -> Result<T, T::Error> {
    unsafe {
      let state = self.state;
      state.fg_pushvalue(self);
      let value = state.fg_getvalue(-1);
      state.lua_pop(1);
      value
    }
  }
}