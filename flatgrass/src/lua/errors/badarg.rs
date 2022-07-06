use super::*;

#[derive(Debug, Clone)]
pub struct BadArgError<T: GetFromLua> {
  funcname: Option<String>,
  location: String,
  err: T::Error,
  narg: i32
}

impl<T: GetFromLua> Error for BadArgError<T>
where T: fmt::Debug, T::Error: Error {}

impl<T: GetFromLua> fmt::Display for BadArgError<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let funcname = &self.funcname;
    let location = &self.location;
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