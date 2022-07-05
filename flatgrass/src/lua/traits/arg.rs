use std::fmt;
use super::*;

/// Types that implement this trait can be used as parameter for a Lua function.
/// All types that implement [`GetFromLua`](GetFromLua) will automatically
/// implement this trait via a blanket implementation,
/// so you should implement it instead whenever possible.
pub trait LuaArg: Sized {
  type Error: fmt::Display;

  unsafe fn resolve(state: LuaState, idx: &mut i32) -> Result<Self, Self::Error>;
}

impl<T: GetFromLua> LuaArg for T {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    match state.fg_getvalue(*narg) {
      Err(err) => Err(BadArgError::new(state, *narg, err)),
      Ok(value) => {
        *narg += 1;
        Ok(value)
      }
    }
  }
}

impl<T: GetFromLua> LuaArg for Option<T> {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    match state.fg_getvalue(*narg) {
      Err(err) => match state.fg_type(*narg) {
        LuaType::None | LuaType::Nil => {
          *narg += 1;
          Ok(None)
        }
        _ => Err(BadArgError::new(state, *narg, err))
      }
      Ok(value) => {
        *narg += 1;
        Ok(Some(value))
      }
    }
  }
}

pub struct BadArgError<T: GetFromLua> {
  funcname: Option<String>,
  location: String,
  err: T::Error,
  narg: i32
}

impl<T: GetFromLua> fmt::Display for BadArgError<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let location = &self.location;
    let funcname = &self.funcname;
    let narg = self.narg;
    let err = &self.err;
    match funcname {
      None => write!(f, "{location}bad argument #{narg} ({err})"),
      Some(funcname) => {
        match narg == 0 {
          true => write!(f, "{location}calling '{funcname}' on bad self ({err})"),
          false => write!(f, "{location}bad argument #{narg} to '{funcname}' ({err})")
        }
      }
    }
  }
}

impl<T: GetFromLua> BadArgError<T> {
  pub unsafe fn new(state: LuaState, mut narg: i32, err: T::Error) -> Self {
    let location = state.fg_where(1);
    match state.fg_getdebug(cstr!("n")) {
      None => Self {
        funcname: None,
        location, narg,
        err
      },
      Some(debug) => {
        let mut funcname = debug.name();
        if funcname.is_empty() { funcname = "?"; }
        if debug.namewhat() == "method" { narg -= 1; }
        Self {
          funcname: Some(funcname.into()),
          location, narg, err
        }
      }
    }
  }
}