use super::table::Table;
use libc::c_void;
use super::*;

pub trait PushToLua: Sized {
  unsafe fn push(state: LuaState, value: Self);
}

pub trait PushManyToLua: Sized {
  unsafe fn push_many(state: LuaState, values: Self);
}

impl<T: PushToLua> PushManyToLua for T {
  unsafe fn push_many(state: LuaState, values: Self) {
    state.fg_pushvalue(values);
  }
}

// primitive lua types ---------------------------

macro_rules! impl_push_primitive {
  (
    $typ:ty => ($state:ident, $value:ident) $body:block
  ) => {
    #[allow(unused_variables)]
    impl PushToLua for $typ {
      unsafe fn push($state: LuaState, $value: Self) $body
    }
    impl PushToLua for &$typ {
      unsafe fn push(state: LuaState, value: Self) {
        state.fg_pushvalue(*value);
      }
    }
  };
}

impl_push_primitive! {
  Nil => (state, value) {
    state.fg_checkstack(1);
    state.lua_pushnil();
  }
}

impl_push_primitive! {
  bool => (state, value) {
    state.fg_checkstack(1);
    state.lua_pushboolean({
      if value {
        1
      } else {
        0
      }
    });
  }
}

impl_push_primitive! {
  f64 => (state, value) {
    state.fg_checkstack(1);
    state.lua_pushnumber(value);
  }
}

impl PushToLua for &str {
  unsafe fn push(state: LuaState, value: Self) {
    let ptr = value.as_ptr() as *const i8;
    state.fg_checkstack(1);
    state.lua_pushlstring(ptr, value.len());
  }
}

impl_push_primitive! {
  LuaCFunction => (state, value) {
    state.fg_checkstack(1);
    state.lua_pushcfunction(value);
  }
}

impl_push_primitive! {
  *mut c_void => (state, value) {
    state.fg_checkstack(1);
    state.lua_pushlightuserdata(value);
  }
}

// other string types

impl PushToLua for &String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(value as &str);
  }
}

impl PushToLua for String {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(&value);
  }
}

// other numbers types --------------------------

macro_rules! impl_push_number {
  ($num:ty) => {
    impl PushToLua for $num {
      unsafe fn push(state: LuaState, value: Self) {
        state.fg_pushvalue(value as f64);
      }
    }
    impl PushToLua for &$num {
      unsafe fn push(state: LuaState, value: Self) {
        state.fg_pushvalue(*value);
      }
    }
  };
}

impl_push_number!(i8);
impl_push_number!(i16);
impl_push_number!(i32);
impl_push_number!(i64);
impl_push_number!(i128);
impl_push_number!(isize);
impl_push_number!(u8);
impl_push_number!(u16);
impl_push_number!(u32);
impl_push_number!(u64);
impl_push_number!(u128);
impl_push_number!(usize);
impl_push_number!(f32);

// option types ------------------------------

impl<'l, T: 'l> PushToLua for &'l Option<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Some(value) => state.fg_pushvalue(value),
      None => state.fg_pushvalue(Nil)
    }
  }
}

impl<T: PushToLua> PushToLua for Option<T> {
  unsafe fn push(state: LuaState, value: Self) {
    match value {
      Some(value) => state.fg_pushvalue(value),
      None => state.fg_pushvalue(Nil)
    }
  }
}

// list types ------------------------------------

use std::collections::VecDeque;
use std::collections::LinkedList;
use std::collections::BinaryHeap;

impl<'l, T: 'l> PushToLua for &'l [T] where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<const N: usize, T: PushToLua> PushToLua for [T; N] {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<'l, T: 'l> PushToLua for &'l Vec<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<T: PushToLua> PushToLua for Vec<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<'l, T: 'l> PushToLua for &'l VecDeque<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<T: PushToLua> PushToLua for VecDeque<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<'l, T: 'l> PushToLua for &'l LinkedList<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<T: PushToLua> PushToLua for LinkedList<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<'l, T: 'l> PushToLua for &'l BinaryHeap<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

impl<T: PushToLua> PushToLua for BinaryHeap<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::list_from_state(state, value))
  }
}

// hash types --------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl<'l, K: 'l, V: 'l> PushToLua for &'l HashMap<K, V>
where &'l K: PushToLua,
      &'l V: PushToLua {

  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::map_from_state(state, value))
  }    
}

impl<K: PushToLua, V: PushToLua> PushToLua for HashMap<K, V> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::map_from_state(state, value))
  }    
}

impl<'l, T: 'l> PushToLua for &'l HashSet<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::set_from_state(state, value))
  }    
}

impl<T: PushToLua> PushToLua for HashSet<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::set_from_state(state, value))
  }    
}

// btree types -------------------------------------

use std::collections::BTreeMap;
use std::collections::BTreeSet;

impl<'l, K: 'l, V: 'l> PushToLua for &'l BTreeMap<K, V>
where &'l K: PushToLua,
      &'l V: PushToLua {

  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::map_from_state(state, value))
  }    
}

impl<K: PushToLua, V: PushToLua> PushToLua for BTreeMap<K, V> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::map_from_state(state, value))
  }    
}

impl<'l, T: 'l> PushToLua for &'l BTreeSet<T> where &'l T: PushToLua {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::set_from_state(state, value))
  }    
}

impl<T: PushToLua> PushToLua for BTreeSet<T> {
  unsafe fn push(state: LuaState, value: Self) {
    state.fg_pushvalue(Table::set_from_state(state, value))
  }    
}

// tuple types -----------------------------------

macro_rules! impl_pushmany_tuple {
  () => {
    impl PushManyToLua for () {
      unsafe fn push_many(_: LuaState, _: Self) {}
    }
    impl PushManyToLua for &() {
      unsafe fn push_many(_: LuaState, _: Self) {}
    }
  };
  ($first:ident $($arg:ident)*) => {
    #[allow(non_camel_case_types)]
    impl<$first: PushToLua, $($arg: PushToLua),*> PushManyToLua for ($first, $($arg,)*) {
      unsafe fn push_many(state: LuaState, values: Self) {
        let ($first, $($arg),*) = values;
        state.fg_pushvalue($first);
        $(state.fg_pushvalue($arg);)*
      }
    }

    #[allow(non_camel_case_types)]
    impl<'l, $first:, $($arg),*> PushManyToLua for &'l ($first, $($arg,)*) where &'l $first: PushToLua, $(&'l $arg: PushToLua),* {
      unsafe fn push_many(state: LuaState, values: Self) {
        let ($first, $($arg),*) = values;
        state.fg_pushvalue($first);
        $(state.fg_pushvalue($arg);)*
      }
    }

    impl_pushmany_tuple! {
      $($arg)*
    }
  };
}

// this generates PushManyToLua impls
// for tuples of length 0 up to 26
// which is probably more than enough
impl_pushmany_tuple! {
  a b c d e f g h i j k
  l m n o p q r s t u v
  w x y z
}