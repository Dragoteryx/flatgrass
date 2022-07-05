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
  pub fn name(&self) -> &str {
    if !self.name.is_null() {
      let cstr = unsafe { CStr::from_ptr(self.name) };
      cstr.to_str().unwrap_or_default()
    } else {
      ""
    }
  }

  pub fn namewhat(&self) -> &str {
    if !self.namewhat.is_null() {
      let cstr = unsafe { CStr::from_ptr(self.namewhat) };
      cstr.to_str().unwrap_or_default()
    } else {
      ""
    }
  }

  pub fn what(&self) -> &str {
    if !self.what.is_null() {
      let cstr = unsafe { CStr::from_ptr(self.what) };
      cstr.to_str().unwrap_or_default()
    } else {
      ""
    }
  }

  pub fn source(&self) -> &str {
    if !self.source.is_null() {
      let cstr = unsafe { CStr::from_ptr(self.source) };
      cstr.to_str().unwrap_or_default()
    } else {
      ""
    }
  }
}