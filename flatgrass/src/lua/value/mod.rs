use super::*;

pub struct LuaValue<'l> {
  phantom: PhantomData<&'l ()>,
  state: LuaState
}