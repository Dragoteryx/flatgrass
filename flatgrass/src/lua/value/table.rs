use super::*;

#[derive(Clone, PartialEq)]
pub struct Table<'l>(LuaValue<'l>);

impl<'l> PushToLua for &Table<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua for Table<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> fmt::Debug for Table<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "table: {:p}", self.pointer())
  }
}

impl<'l> Table<'l> {
  /// Pops the value at the top of the stack and returns a Table.
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
      tbl.add(value);
    }

    tbl
  }

  pub(crate) fn state(&self) -> LuaState {
    self.0.state
  }

  pub fn pointer(&self) -> *const c_void {
    self.0.pointer()
  }

  pub fn get(&self, key: impl PushToLua) -> Option<LuaValue<'l>> {
    unsafe {
      let state = self.state();
      state.fg_checkstack(2);
      state.fg_pushvalue(self);
      state.fg_pushvalue(key);
      state.lua_rawget(-2);
      let value = LuaValue::pop(state);
      state.lua_pop(1);
      match value.get_type() {
        LuaType::Nil => None,
        _ => Some(value)
      }
    }
  }

  pub fn has(&self, key: impl PushToLua) -> bool {
    self.get(key).is_some()
  }

  pub fn set(&self, key: impl PushToLua, value: impl PushToLua) {
    unsafe {
      let state = self.state();
      state.fg_checkstack(3);
      state.fg_pushvalue(self);
      state.fg_pushvalue(key);
      state.fg_pushvalue(value);
      state.lua_rawset(-3);
      state.lua_pop(1);
    }
  }

  pub fn push(&self, value: impl PushToLua) {
    self.set(self.len() + 1, value);
  }

  pub fn add(&self, value: impl PushToLua) {
    self.set(value, true);
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