use super::*;

/// Types that implement this trait can be returned from a Lua function.
/// All types that implement [`PushToLua`](PushToLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaReturn {
  unsafe fn push(state: LuaState, value: Self);
}

impl<T: PushToLua> LuaReturn for T {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(1);
    state.fg_pushvalue(value);
  }
}

impl<T: LuaReturn, E: PushToLua> LuaReturn for Result<T, E> {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Ok(value) => LuaReturn::push(state, value),
      Err(error) => state.fg_error(error)
    }
  }
}

impl<A: PushToLua> LuaReturn for (A,) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(1);
    state.fg_pushvalue(value.0);
  }
}

impl<A: PushToLua, B: PushToLua> LuaReturn for (A, B) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(2);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
  }
}

impl<A: PushToLua, B: PushToLua, C: PushToLua> LuaReturn for (A, B, C) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(3);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
  }
}

impl<A: PushToLua, B: PushToLua, C: PushToLua, D: PushToLua> LuaReturn for (A, B, C, D) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(4);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
  }
}

impl<A: PushToLua, B: PushToLua, C: PushToLua, D: PushToLua, E: PushToLua> LuaReturn for (A, B, C, D, E) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(5);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
    state.fg_pushvalue(value.4);
  }
}

impl<A: PushToLua, B: PushToLua, C: PushToLua, D: PushToLua, E: PushToLua, F: PushToLua> LuaReturn for (A, B, C, D, E, F) {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_checkstack(6);
    state.fg_pushvalue(value.0);
    state.fg_pushvalue(value.1);
    state.fg_pushvalue(value.2);
    state.fg_pushvalue(value.3);
    state.fg_pushvalue(value.4);
    state.fg_pushvalue(value.5);
  }
}