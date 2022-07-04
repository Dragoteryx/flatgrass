use super::*;

#[derive(Clone, PartialEq, PartialOrd)]
pub struct LuaTable<'l>(LuaValue<'l>);

impl<'l> PushToLua for &LuaTable<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua for LuaTable<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> LuaTable<'l> {
  /// Pops the value at the top of the stack and returns a LuaTable.
  /// # Safety
  /// The stack must not be empty, and the value at the top needs to be a table.
  pub unsafe fn pop(state: LuaState) -> Self {
    Self(LuaValue::pop(state))
  }

  pub fn new(lua: &Lua<'l>) -> Self {
    unsafe {
      lua.state.fg_checkstack(1);
      lua.state.lua_newtable();
      Self::pop(lua.state)
    }
  }

  pub fn state(&self) -> LuaState {
    self.0.state
  }

  pub fn new_list<T: PushToLua>(lua: &Lua<'l>, iter: impl IntoIterator<Item = T>) -> Self {
    let tbl = Self::new(lua);
    for value in iter.into_iter() {
      tbl.push(value);
    }

    tbl
  }

  pub fn new_map<K: PushToLua, V: PushToLua>(lua: &Lua<'l>, iter: impl IntoIterator<Item = (K, V)>) -> Self {
    let tbl = Self::new(lua);
    for (key, value) in iter.into_iter() {
      tbl.set(key, value);
    }
    
    tbl
  }

  pub fn new_set<T: PushToLua>(lua: &Lua<'l>, iter: impl IntoIterator<Item = T>) -> Self {
    let tbl = Self::new(lua);
    for value in iter.into_iter() {
      tbl.set(value, true);
    }

    tbl
  }

  pub fn get(&self, key: impl PushToLua) -> LuaValue<'l> {
    unsafe {
      let state = self.state();
      state.fg_checkstack(2);
      state.fg_pushvalue(self);
      state.fg_pushvalue(key);
      state.lua_gettable(-2);
      let value = LuaValue::pop(state);
      state.lua_pop(1);
      value
    }
  }

  pub fn has(&self, key: impl PushToLua) -> bool {
    self.get(key).get_type() != LuaType::Nil
  }

  pub fn set(&self, key: impl PushToLua, value: impl PushToLua) {
    unsafe {
      let state = self.state();
      state.fg_checkstack(3);
      state.fg_pushvalue(self);
      state.fg_pushvalue(key);
      state.fg_pushvalue(value);
      state.lua_settable(-3);
      state.lua_pop(1);
    }
  }

  pub fn push(&self, value: impl PushToLua) {
    self.set(self.len() + 1, value);
  }

  pub fn len(&self) -> usize {
    unsafe {
      let state = self.state();
      state.fg_checkstack(1);
      state.fg_pushvalue(self);
      let len = state.lua_objlen(-1);
      state.lua_pop(1);
      len
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}