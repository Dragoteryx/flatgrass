use libc::c_void;
use std::fmt;
use super::*;

mod builder; pub use builder::*;

#[derive(Clone, PartialEq)]
pub struct Function<'l>(LuaValue<'l>);

// misc impls --------------------------

impl<'l> fmt::Debug for Function<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Function({:p})", self.pointer())
  }
}

// lua impls ---------------------------

impl<'l> PushToLua<'l> for &Function<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua<'l> for Function<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> GetFromLua<'l> for Function<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState<'l>, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Function {
      Err(GetFromLuaError::UnexpectedType(LuaType::Function, typ))
    } else {
      state.fg_checkstack(1);
      state.lua_pushvalue(idx);
      Ok(Self::pop(state))
    }
  }
}

// main impl ----------------------------

impl<'l> Function<'l> {
  unsafe fn pop(state: LuaState<'l>) -> Self {
    Self(LuaValue::pop(state))
  }

  pub unsafe fn builder_from_state(state: LuaState<'l>, func: LuaCFunction) -> FunctionBuilder<'l> {
    FunctionBuilder { upvalues: Vec::new(), func, state }
  }

  pub unsafe fn from_state(state: LuaState<'l>, func: LuaCFunction) -> Self {
    Self::builder_from_state(state, func).build()
  }

  pub fn builder(lua: &Lua<'l>, func: LuaCFunction) -> FunctionBuilder<'l> {
    unsafe { Self::builder_from_state(lua.0, func) }
  }

  pub fn new(lua: &Lua<'l>, func: LuaCFunction) -> Self {
    Self::builder(lua, func).build()
  }

  pub fn pointer(&self) -> *const c_void {
    self.0.pointer()
  }
}