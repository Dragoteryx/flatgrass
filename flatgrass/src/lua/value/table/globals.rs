use super::*;

pub struct Globals<'l>(Table<'l>);

// lua impls -----------------------

impl<'l> PushToLua<'l> for &Globals<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua<'l> for Globals<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> LuaArg<'l> for Globals<'l> {
  type Error = Infallible;

  unsafe fn resolve(state: LuaState<'l>, _: &mut i32) -> Result<Self, Self::Error> {
    Ok(Self::from_state(state))
  }
}

// main impl ---------------------

impl<'l> Globals<'l> {
  pub unsafe fn from_state(state: LuaState<'l>) -> Self {
    state.fg_checkstack(1);
    state.lua_pushvalue(LUA_GLOBALSINDEX);
    Self(Table::pop(state))
  }

  pub fn get(&self, key: impl PushToLua<'l>) -> Option<LuaValue<'l>> {
    self.0.get(key)
  }

  pub fn set(&self, key: impl PushToLua<'l>, value: impl PushToLua<'l>) {
    self.0.set(key, value);
  }

  #[must_use = "check if the library has been properly initialized"]
  pub fn init_lib(&self, name: &str, func: impl FnOnce(&Table<'l>)) -> bool {
    let tbl = match self.get(name) {
      None => Some(unsafe { Table::from_state(self.0.0.state) }),
      Some(value) => value.try_as().ok()
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