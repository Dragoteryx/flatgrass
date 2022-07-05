use super::*;

#[derive(Clone, PartialEq)]
pub struct LuaFunction<'l>(LuaValue<'l>);

impl<'l> PushToLua for &LuaFunction<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua for LuaFunction<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> LuaFunction<'l> {
  /// Pops the value at the top of the stack and returns a LuaFunction.
  /// # Safety
  /// The stack must not be empty, and the value at the top needs to be a function.
  pub unsafe fn pop(state: LuaState) -> Self {
    Self(LuaValue::pop(state))
  }

  pub fn new(lua: &Lua<'l>, func: LuaCFunction) -> Self {
    unsafe {
      lua.state.fg_checkstack(1);
      lua.state.lua_pushcfunction(func);
      Self::pop(lua.state)
    }
  }

  pub fn state(&self) -> LuaState {
    self.0.state
  }
}