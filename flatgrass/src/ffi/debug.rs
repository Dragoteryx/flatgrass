use std::ffi::CStr;
use super::*;

/// See the Lua 5.1 manual: [`lua_Debug`](https://www.lua.org/manual/5.1/manual.html#lua_Debug)
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LuaDebug {
  pub event: c_int,
  name: *const c_char,
  namewhat: *const c_char,
  what: *const c_char,
  source: *const c_char,
  pub currentline: c_int,
  pub nups: c_int,
  pub linedefined: c_int,
  pub lastlinedefined: c_int,
  pub short_src: [c_char; 128],
  i_ci: c_int
}

impl Default for LuaDebug {
	fn default() -> Self {
		Self {
			event: 0,
      name: std::ptr::null(),
      namewhat: std::ptr::null(),
      what: std::ptr::null(),
      source: std::ptr::null(),
      currentline: 0, nups: 0,
      linedefined: 0,
      lastlinedefined: 0,
      short_src: [0; 128],
      i_ci: 0
		}
	}
}

impl LuaDebug {
  pub unsafe fn name(&self) -> &str {
    if !self.name.is_null() {
      CStr::from_ptr(self.name).to_str().unwrap()
    } else {
      ""
    }
  }

  pub unsafe fn namewhat(&self) -> &str {
    if !self.name.is_null() {
      CStr::from_ptr(self.name).to_str().unwrap()
    } else {
      ""
    }
  }

  pub unsafe fn what(&self) -> &str {
    if !self.name.is_null() {
      CStr::from_ptr(self.what).to_str().unwrap()
    } else {
      ""
    }
  }

  pub unsafe fn source(&self) -> &str {
    if !self.name.is_null() {
      CStr::from_ptr(self.source).to_str().unwrap()
    } else {
      ""
    }
  }
}