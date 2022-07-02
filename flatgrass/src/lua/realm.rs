use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LuaRealm {
  Server,
  Client,
  Menu
}