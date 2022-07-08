use std::vec::IntoIter;
use super::*;

#[derive(Debug, Clone)]
pub struct Tuple<T>(pub Vec<T>);

// misc impls ------------------

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

// lua impls ------------------------

impl<T: PushToLua> PushManyToLua for Tuple<T> {
  unsafe fn push_many(state: LuaState, value: Self) {
    for value in value.0 {
      state.fg_pushvalue(value);
    }
  }
}

impl<'l, T: GetFromLua<'l>> LuaArg<'l> for Tuple<T> {
  type Error = BadArgError<'l, T>;

  unsafe fn resolve(state: LuaState<'l>, narg: &mut i32) -> Result<Self, Self::Error> {
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

// main impl -------------------------

impl<T> Tuple<T> {
  pub fn new() -> Self { Self(Vec::new()) }

  pub fn push(mut self, value: T) -> Self {
    self.0.push(value);
    self
  }
}