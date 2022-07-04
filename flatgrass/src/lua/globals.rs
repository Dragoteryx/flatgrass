use super::*;

pub struct LuaGlobals<'l>(LuaTable<'l>);

impl<'l> PushToLua for &LuaGlobals<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua for LuaGlobals<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> LuaArg for LuaGlobals<'l> {
  type Error = std::convert::Infallible;

  unsafe fn resolve(state: LuaState, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

impl<'l> LuaGlobals<'l> {
  pub unsafe fn from_state(state: LuaState) -> Self {
    state.fg_checkstack(1);
    state.lua_pushvalue(LUA_GLOBALSINDEX);
    Self(LuaTable::pop(state))
  }

  pub fn state(&self) -> LuaState {
    self.0.state()
  }

  pub fn get(&self, key: impl PushToLua) -> LuaValue<'l> {
    self.0.get(key)
  }

  pub fn set(&self, key: impl PushToLua, value: impl PushToLua) {
    self.0.set(key, value);
  }

  pub fn add_lib(&self, name: &str, func: impl FnOnce(&LuaTable<'l>)) {
    unsafe {
      let state = self.state();
      state.fg_checkstack(1);
      state.lua_newtable();
      let tbl = LuaTable::pop(state);
      func(&tbl);
      self.set(name, tbl);
    }
  }
}