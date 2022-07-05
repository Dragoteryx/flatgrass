use super::*;

mod table; pub use table::*;
mod func; pub use func::*;

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
      let eq = self.state.lua_equal(-2, -1);
      self.state.lua_pop(2);
      eq != 0
    }
  }
}

impl<'l> PartialOrd for LuaValue<'l> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    use std::cmp::Ordering::*;

    if self == other { 
      Some(Equal)
    } else {
      let lt = unsafe {
        self.state.fg_checkstack(2);
        self.state.fg_pushvalue(self);
        self.state.fg_pushvalue(other);
        let lt = self.state.lua_lessthan(-2, -1);
        self.state.lua_pop(2);
        lt != 0
      };
      
      if lt {
        Some(Less)
      } else {
        Some(Greater)
      }
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

  pub fn state(&self) -> LuaState {
    self.state
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
}