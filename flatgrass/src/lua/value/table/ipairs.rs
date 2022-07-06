use std::ops::RangeFrom;
use super::*;

#[derive(Debug, Clone)]
pub struct Ipairs<'a, 'l> {
  pub(super) range: RangeFrom<isize>,
  pub(super) table: &'a Table<'l>
}

impl<'a, 'l> Iterator for Ipairs<'a, 'l> {
  type Item = LuaValue<'l>;

  fn next(&mut self) -> Option<Self::Item> {
    self.table.get(self.range.next())
  }
}