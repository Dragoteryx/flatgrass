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
  type Error = Infallible;

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

  pub(crate) fn state(&self) -> LuaState {
    self.0.state()
  }

  pub fn get(&self, key: impl PushToLua) -> Option<LuaValue<'l>> {
    self.0.get(key)
  }

  pub fn set(&self, key: impl PushToLua, value: impl PushToLua) {
    self.0.set(key, value);
  }

  #[must_use = "check if the library has been properly initialized"]
  pub fn init_lib(&self, name: &str, func: impl FnOnce(&LuaTable<'l>)) -> bool {
    let tbl = match self.get(name) {
      Some(value) => value.try_as().ok(),
      None => unsafe {
        let state = self.state();
        state.fg_checkstack(1);
        state.lua_newtable();
        Some(LuaTable::pop(state))
      }
    };

    match tbl {
      None => false,
      Some(tbl) => {
        func(&tbl);
        self.set(name, tbl);
        true
      }
    }
  }
}