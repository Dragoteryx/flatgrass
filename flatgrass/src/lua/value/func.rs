use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use libc::c_void;
use std::fmt;
use super::*;

#[derive(Clone)]
pub struct Function<'l>(LuaValue<'l>);

// misc impls --------------------------

impl<'l> Eq for Function<'l> {}
impl<'l> PartialEq for Function<'l> {
  fn eq(&self, other: &Self) -> bool {
    self.pointer() == other.pointer() 
  }
}

impl<'l> Hash for Function<'l> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.pointer().hash(state);
  }
}

impl<'l> fmt::Debug for Function<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Function ({:p})", self.pointer())
  }
}

// lua impls ---------------------------

impl<'l> PushToLua for &Function<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua for Function<'l> {
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
      state.fg_pushindex(idx);
      Ok(Self::pop(state))
    }
  }
}

// main impl ----------------------------

impl<'l> Function<'l> {
  pub unsafe fn pop(state: LuaState<'l>) -> Self {
    Self(LuaValue::pop(state))
  }

  pub unsafe fn from_state(state: LuaState<'l>, func: LuaCFunction, upvalues: impl PushManyToLua) -> Self {
    state.fg_pushfunction(func, upvalues);
    Self::pop(state)
  }

  pub fn new(lua: &Lua<'l>, func: LuaCFunction, upvalues: impl PushManyToLua) -> Self {
    unsafe { Self::from_state(lua.0, func, upvalues) }
  }

  pub fn pointer(&self) -> *const c_void {
    self.0.pointer()
  }

  pub fn call(&self, args: impl PushManyToLua) -> Result<VecDeque<LuaValue<'l>>, LuaValue<'l>> {
    unsafe {
      let state = self.0.state;
      let top = state.lua_gettop();
      state.fg_pushvalue(self);
      let nargs = state.fg_pushmany(args);
      match state.lua_pcall(nargs, LUA_MULTRET, 0) {
        0 => {
          let mut res = VecDeque::new();
          while state.lua_gettop() > top {
            res.push_back(LuaValue::pop(state));
          }
          Ok(res)
        }
        _ => Err(LuaValue::pop(state))
      }
    }
  }
}