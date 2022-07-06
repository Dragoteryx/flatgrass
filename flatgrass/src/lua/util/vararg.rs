use std::vec::IntoIter;
use super::*;

/// This type is an [Iterator] that mimics the behavior of the ... parameter in Lua.
/// It can be used when you are expecting an unknown number of arguments.
/// ```
/// #[flatgrass:function(lua_sum)]
/// pub fn sum(values: Vararg<f64>) -> f64 {
///   values.sum()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Vararg<T: GetFromLua>(IntoIter<T>);

impl<T: GetFromLua> LuaArg for Vararg<T> {
  type Error = BadArgError<T>;

  unsafe fn resolve(state: LuaState, narg: &mut i32) -> Result<Self, Self::Error> {
    let mut values = Vec::new();
    loop {
      match state.fg_type(*narg) {
        LuaType::None => return Ok(Self(values.into_iter())),
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

impl<T: GetFromLua> Iterator for Vararg<T> {
  type Item = T;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<T: GetFromLua> DoubleEndedIterator for Vararg<T> {
  fn next_back(&mut self) -> Option<Self::Item> {
    self.0.next_back()
  }
}