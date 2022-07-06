use super::*;

#[derive(Clone)]
pub struct FunctionBuilder<'l> {
  pub(super) upvalues: Vec<LuaValue<'l>>,
  pub(super) func: LuaCFunction,
  pub(super) state: LuaState<'l>
}

impl<'l> FunctionBuilder<'l> {
  pub fn add_upvalue(mut self, upvalue: impl PushToLua<'l>) -> Self {
    unsafe {
      let state = self.state;
      state.fg_checkstack(1);
      state.fg_pushvalue(upvalue);
      self.upvalues.push(LuaValue::pop(state));
    }
    self
  }

  pub fn build(self) -> Function<'l> {
    unsafe {
      let state = self.state;
      let n = self.upvalues.into_iter().map(|upvalue| {
        state.fg_checkstack(1);
        state.fg_pushvalue(upvalue);
      }).count().try_into().unwrap();

      state.fg_checkstack(1);
      state.lua_pushcclosure(self.func, n);
      Function::pop(state)
    }
  }
}