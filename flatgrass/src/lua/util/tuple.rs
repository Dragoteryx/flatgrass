use std::vec::IntoIter;
use super::*;

#[derive(Debug, Clone)]
pub struct Tuple<T>(pub Vec<T>);

impl<T> Default for Tuple<T> {
  fn default() -> Self { Self::new() }
}

impl<T> FromIterator<T> for Tuple<T> {
  fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
    Self(iter.into_iter().collect())
  }
}

impl<T> IntoIterator for Tuple<T> {
  type IntoIter = IntoIter<T>;
  type Item = T;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<T> Tuple<T> {
  pub fn new() -> Self { Self(Vec::new()) }

  pub fn push(mut self, value: T) -> Self {
    self.0.push(value);
    self
  }
}

impl<T: GetFromLua> LuaArg for Tuple<T> {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    let mut values = Vec::new();
    loop {
      match state.fg_type(*narg) {
        LuaType::None => return Ok(Self(values)),
        _ => match state.fg_getvalue(*narg) {
          Err(err) => return Err(BadArgError::new(state, *narg, err)),
          Ok(value) => {
            *narg += 1;
            values.push(value);
          }
        }
      }
    }
  }
}

impl<T: PushToLua> LuaReturn for Tuple<T> {
  type Error = Infallible;

  unsafe fn push(state: LuaState, value: Self) -> Result<i32, Self::Error> {
    let nret = value.0.into_iter().map(|value| {
      state.fg_checkstack(1);
      state.fg_pushvalue(value);
    }).count().try_into().unwrap_or(i32::MAX);

    Ok(nret)
  }
}