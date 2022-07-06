use super::*;

#[derive(Debug, Clone)]
pub struct Pairs<'a, 'l> {
  pub(super) prev: Option<LuaValue<'l>>,
  pub(super) table: &'a Table<'l>
}

impl<'a, 'l> Iterator for Pairs<'a, 'l> {
  type Item = (LuaValue<'l>, LuaValue<'l>);

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(prev) = &self.prev {
      if !self.table.has(prev) {
        self.prev = None;
        return None;
      }
    }

    unsafe {
      let state = self.table.0.state;
      state.fg_checkstack(3);
      state.fg_pushvalue(self.table);
      state.fg_pushvalue(&self.prev);
      if state.lua_next(-2) != 0 {
        let value = LuaValue::pop(state);
        let key = LuaValue::pop(state);
        self.prev = Some(key.clone());
        state.lua_pop(1);
        Some((key, value))
      } else {
        self.prev = None;
        state.lua_pop(1);
        None
      }
    }
  }
}