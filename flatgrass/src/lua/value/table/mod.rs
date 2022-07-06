use libc::c_void;
use std::fmt;
use super::*;

mod globals; pub use globals::*;
mod ipairs; pub use ipairs::*;
mod pairs; pub use pairs::*;

#[derive(Clone, PartialEq)]
pub struct Table<'l>(LuaValue<'l>);

// misc impls --------------------------

impl<'l> fmt::Debug for Table<'l> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Table({:p}) ", self.pointer())?;

    if self.is_sequential() {
      f.debug_list()
        .entries(self.ipairs())
        .finish()
    } else {
      f.debug_map()
        .entries(self.pairs())
        .finish()
    }
  }
}

// lua impls ---------------------------

impl<'l> PushToLua<'l> for &Table<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value.0);
  }
}

impl<'l> PushToLua<'l> for Table<'l> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value.0);
  }
}

impl<'l> GetFromLua<'l> for Table<'l> {
  type Error = GetFromLuaError;

  unsafe fn try_get(state: LuaState<'l>, idx: i32) -> Result<Self, Self::Error> {
    let typ = state.fg_type(idx);
    if typ != LuaType::Table {
      Err(GetFromLuaError::UnexpectedType(LuaType::Table, typ))
    } else {
      state.fg_checkstack(1);
      state.lua_pushvalue(idx);
      Ok(Self::pop(state))
    }
  }
}

// main impl ----------------------------

impl<'l> Table<'l> {
  unsafe fn pop(state: LuaState<'l>) -> Self {
    Self(LuaValue::pop(state))
  }

  pub unsafe fn from_state(state: LuaState<'l>) -> Self {
    state.fg_checkstack(1);
    state.lua_newtable();
    Self::pop(state)
  }

  pub fn new(lua: &Lua<'l>) -> Self {
    unsafe { Self::from_state(lua.0) }
  }

  pub fn new_list<T: PushToLua<'l>>(lua: &Lua<'l>, iter: impl IntoIterator<Item = T>) -> Self {
    let tbl = Self::new(lua);
    for value in iter.into_iter() {
      tbl.push(value);
    }

    tbl
  }

  pub fn new_map<K: PushToLua<'l>, V: PushToLua<'l>>(lua: &Lua<'l>, iter: impl IntoIterator<Item = (K, V)>) -> Self {
    let tbl = Self::new(lua);
    for (key, value) in iter.into_iter() {
      tbl.set(key, value);
    }
    
    tbl
  }

  pub fn new_set<T: PushToLua<'l>>(lua: &Lua<'l>, iter: impl IntoIterator<Item = T>) -> Self {
    let tbl = Self::new(lua);
    for value in iter.into_iter() {
      tbl.add(value);
    }

    tbl
  }

  pub fn pointer(&self) -> *const c_void {
    self.0.pointer()
  }

  pub fn get(&self, key: impl PushToLua<'l>) -> Option<LuaValue<'l>> {
    unsafe {
      let state = self.0.state;
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

  pub fn has(&self, key: impl PushToLua<'l>) -> bool {
    self.get(key).is_some()
  }

  pub fn set(&self, key: impl PushToLua<'l>, value: impl PushToLua<'l>) {
    unsafe {
      let state = self.0.state;
      state.fg_checkstack(3);
      state.fg_pushvalue(self);
      state.fg_pushvalue(key);
      state.fg_pushvalue(value);
      state.lua_rawset(-3);
      state.lua_pop(1);
    }
  }

  pub fn push(&self, value: impl PushToLua<'l>) {
    self.set(self.len() + 1, value);
  }

  pub fn add(&self, value: impl PushToLua<'l>) {
    self.set(value, true);
  }

  pub fn len(&self) -> usize {
    unsafe {
      let state = self.0.state;
      state.fg_checkstack(1);
      state.fg_pushvalue(self);
      let len = state.lua_objlen(-1);
      state.lua_pop(1);
      len
    }
  }

  pub fn is_empty(&self) -> bool {
    self.pairs().count() == 0
  }

  pub fn ipairs(&self) -> Ipairs<'_, 'l> {
    Ipairs { range: (1..), table: self }
  }

  pub fn pairs(&self) -> Pairs<'_, 'l> {
    Pairs { prev: None, table: self }
  }

  pub fn is_sequential(&self) -> bool {
    self.len() == self.pairs().count()
  }
}