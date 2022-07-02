use super::*;

/// Types that implement this trait can be returned from a Lua function.
/// All types that implement [`ToLua`](ToLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaReturn {
  unsafe fn push(state: LuaState, value: Self);
}

impl<T: ToLua> LuaReturn for T {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value)
  }
}

impl<T: ToLua, E: ToLua> LuaReturn for Result<T, E> {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Ok(value) => ToLua::push(state, value),
      Err(err) => {
        ToLua::push(state, err);
        lua_error(state);
      }
    }
  }
}

impl<A: ToLua> LuaReturn for (A,) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
  }
}

impl<A: ToLua, B: ToLua> LuaReturn for (A, B) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
    ToLua::push(state, value.1);
  }
}

impl<A: ToLua, B: ToLua, C: ToLua> LuaReturn for (A, B, C) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
    ToLua::push(state, value.1);
    ToLua::push(state, value.2);
  }
}

impl<A: ToLua, B: ToLua, C: ToLua, D: ToLua> LuaReturn for (A, B, C, D) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
    ToLua::push(state, value.1);
    ToLua::push(state, value.2);
    ToLua::push(state, value.3);
  }
}

impl<A: ToLua, B: ToLua, C: ToLua, D: ToLua, E: ToLua> LuaReturn for (A, B, C, D, E) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
    ToLua::push(state, value.1);
    ToLua::push(state, value.2);
    ToLua::push(state, value.3);
    ToLua::push(state, value.4);
  }
}

impl<A: ToLua, B: ToLua, C: ToLua, D: ToLua, E: ToLua, F: ToLua> LuaReturn for (A, B, C, D, E, F) {
  unsafe fn push(state: LuaState, value: Self) {
    ToLua::push(state, value.0);
    ToLua::push(state, value.1);
    ToLua::push(state, value.2);
    ToLua::push(state, value.3);
    ToLua::push(state, value.4);
    ToLua::push(state, value.5);
  }
}