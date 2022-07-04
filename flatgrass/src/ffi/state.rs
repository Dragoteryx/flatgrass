use crate::lua::traits::PushToLua;
use super::*;

/// See the Lua 5.1 manual: [`lua_State`](https://www.lua.org/manual/5.1/manual.html#lua_State)
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct LuaState(NonNull<c_void>);

#[allow(clippy::missing_safety_doc)]
impl LuaState {
  pub unsafe fn fg_checkstack(self, size: c_int) {
    if self.lua_checkstack(size) == 0 {
      self.fg_error("stack overflow");
    }
  }

  pub unsafe fn fg_pushvalue(self, value: impl PushToLua) {
    PushToLua::push(self, value);
  }

  pub unsafe fn fg_print(&self, value: impl PushToLua) {
    self.fg_checkstack(2);
    self.lua_getglobal(cstr!("print"));
    self.fg_pushvalue(value);
    self.lua_call(1, 0);
  }

  pub unsafe fn fg_error(&self, error: impl PushToLua) -> ! {
    self.fg_checkstack(1);
    self.fg_pushvalue(error);
    self.lua_error();
    unreachable!();
  }
}