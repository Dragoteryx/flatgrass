use super::*;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::rc::Rc;

#[repr(transparent)]
#[derive(Clone)]
pub struct Table {
	reference: Rc<Reference>,
}

impl LuaStack {
	pub fn push_table(&self, tbl: &Table) {
		self.push_reference(&tbl.reference);
	}

	pub fn pop_table(&self) -> Option<Table> {
		if self.get_type(-1) == Some(LuaType::Table) {
			unsafe { Some(self.pop_table_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_table_unchecked(&self) -> Table {
		Table {
			reference: Rc::new(self.pop_reference_unchecked()),
		}
	}

	pub fn get_table(&self, idx: i32) -> Option<Table> {
		if self.get_type(idx) == Some(LuaType::Table) {
			Some(unsafe { self.get_table_unchecked(idx) })
		} else {
			None
		}
	}

	pub unsafe fn get_table_unchecked(&self, idx: i32) -> Table {
		Table {
			reference: Rc::new(self.get_reference_unchecked(idx)),
		}
	}
}

impl Table {
	pub fn new() -> Self {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_new_table();
			stack.pop_table_unchecked()
		})
	}

	pub fn globals() -> Self {
		Lua::get(|lua| unsafe { lua.stack().get_table_unchecked(ffi::LUA_GLOBALSINDEX) })
	}

	pub fn environment() -> Self {
		Lua::get(|lua| unsafe { lua.stack().get_table_unchecked(ffi::LUA_ENVIRONINDEX) })
	}

	pub unsafe fn registry() -> Self {
		Lua::get(|lua| lua.stack().get_table_unchecked(ffi::LUA_REGISTRYINDEX))
	}

	pub fn to_ptr(&self) -> *const ffi::c_void {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_table(self);
			let ptr = ffi::lua_topointer(lua.state(), -1);
			stack.pop_n(1);
			ptr
		})
	}

	pub fn raw_get<K: ToLua>(&self, key: K) -> LuaValue {
		let key = key.to_lua();
		if key.is_nil() {
			LuaValue::Nil
		} else {
			Lua::get(|lua| unsafe {
				let stack = lua.stack();
				stack.push_table(self);
				stack.push_any(key);
				ffi::lua_rawget(lua.state(), -2);
				let value = stack.pop_value_unchecked();
				stack.pop_n(1);
				value
			})
		}
	}

	pub fn raw_has<K: ToLua>(&self, key: K) -> bool {
		!self.raw_get(key).is_nil()
	}

	pub fn raw_set<K: ToLua, V: ToLua>(&self, key: K, value: V) {
		let key = key.to_lua();
		if !key.is_nil() {
			Lua::get(|lua| unsafe {
				let stack = lua.stack();
				stack.push_table(self);
				stack.push_any(key);
				stack.push_any(value);
				ffi::lua_rawset(lua.state(), -3);
				stack.pop_n(1);
			});
		}
	}

	pub fn raw_push<V: ToLua>(&self, value: V) {
		self.raw_set(self.len() + 1, value);
	}

	pub fn raw_remove<K: ToLua>(&self, key: K) -> LuaValue {
		let key = key.to_lua();
		let value = self.raw_get(&key);
		self.raw_set(key, LuaValue::Nil);
		value
	}

	pub fn raw_pop(&self) -> LuaValue {
		self.raw_remove(self.len())
	}

	pub fn get<K: ToLua>(&self, key: K) -> Result<LuaValue, LuaValue> {
		static GET: ffi::lua_CFunction = ffi::raw_function!(|state| {
			ffi::lua_gettable(state, 1);
			1
		});

		let key = key.to_lua();
		if key.is_nil() {
			Ok(LuaValue::Nil)
		} else {
			Lua::get(|lua| {
				let stack = lua.stack();
				stack.push_c_function(GET);
				stack.push_table(self);
				stack.push_any(key);
				unsafe {
					match ffi::lua_pcall(lua.state(), 2, 1, 0) {
						0 => Ok(stack.pop_value_unchecked()),
						_ => Err(stack.pop_value_unchecked()),
					}
				}
			})
		}
	}

	pub fn has<K: ToLua>(&self, key: K) -> Result<bool, LuaValue> {
		self.get(key).map(|value| !value.is_nil())
	}

	pub fn set<K: ToLua, V: ToLua>(&self, key: K, value: V) -> Result<(), LuaValue> {
		const SET: ffi::lua_CFunction = ffi::raw_function!(|state| {
			ffi::lua_settable(state, 1);
			0
		});

		let key = key.to_lua();
		if key.is_nil() {
			Ok(())
		} else {
			Lua::get(|lua| {
				let stack = lua.stack();
				stack.push_c_function(SET);
				stack.push_table(self);
				stack.push_any(key);
				stack.push_any(value);
				unsafe {
					match ffi::lua_pcall(lua.state(), 3, 0, 0) {
						0 => Ok(()),
						_ => Err(stack.pop_value_unchecked()),
					}
				}
			})
		}
	}

	pub fn push<V: ToLua>(&self, value: V) -> Result<(), LuaValue> {
		self.set(self.len() + 1, value)
	}

	pub fn remove<K: ToLua>(&self, key: K) -> Result<LuaValue, LuaValue> {
		let value = self.get(&key)?;
		self.set(key, LuaValue::Nil)?;
		Ok(value)
	}

	pub fn pop(&self) -> Result<LuaValue, LuaValue> {
		self.remove(self.len())
	}

	pub fn len(&self) -> usize {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_table(self);
			let len = ffi::lua_objlen(lua.state(), -1);
			stack.pop_n(1);
			len
		})
	}

	pub fn next<K: ToLua>(&self, key: K) -> Option<(LuaValue, LuaValue)> {
		let key = key.to_lua();
		if !key.is_nil() && !self.raw_has(&key) {
			None
		} else {
			Lua::get(|lua| unsafe {
				let stack = lua.stack();
				stack.push_table(self);
				stack.push_any(key);
				match ffi::lua_next(lua.state(), -2) {
					0 => {
						stack.pop_n(1);
						None
					}
					_ => {
						let value = stack.pop_value_unchecked();
						let key = stack.pop_value_unchecked();
						stack.pop_n(1);
						Some((key, value))
					}
				}
			})
		}
	}

	pub fn is_empty(&self) -> bool {
		self.next(LuaValue::Nil).is_none()
	}

	pub fn is_sequential(&self) -> bool {
		self.pairs().enumerate().all(|(i, _)| self.raw_has(i + 1))
	}

	pub fn ipairs(&self) -> Ipairs {
		Ipairs {
			table: self,
			key: 1,
		}
	}

	pub fn pairs(&self) -> Pairs {
		Pairs {
			table: self,
			key: LuaValue::Nil,
		}
	}

	pub fn recurse<T>(&self, func: impl FnOnce(usize) -> T) -> T {
		thread_local! {
			static VISITED: RefCell<HashMap<*const ffi::c_void, usize>> = RefCell::default();
		}

		VISITED.with(|visited| {
			let ptr = self.to_ptr();
			let depth = {
				let mut visited = visited.borrow_mut();
				let depth = match visited.entry(ptr) {
					Entry::Occupied(entry) => entry.into_mut(),
					Entry::Vacant(entry) => entry.insert(0),
				};

				let old_depth = *depth;
				*depth += 1;
				old_depth
			};

			let res = func(depth);
			let mut visited = visited.borrow_mut();
			let depth = visited.get_mut(&ptr).unwrap();
			*depth -= 1;
			if *depth == 0 {
				visited.remove(&ptr);
			}

			res
		})
	}
}

impl ToLua for Table {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		LuaValue::Table(self)
	}
}

impl FromLua for Table {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Table(tbl) = value {
			Ok(tbl)
		} else {
			Err(FromLuaError::expected_and_got_type(
				LuaType::Table,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected_type(LuaType::Table))
	}
}

impl Default for Table {
	fn default() -> Self {
		Self::new()
	}
}

impl Debug for Table {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Table[{:?}] ", self.to_ptr())?;
		self.recurse(|depth| match (depth > 0, self.is_sequential()) {
			(false, false) => f.debug_map().entries(self.pairs()).finish(),
			(false, true) => f.debug_list().entries(self.ipairs().map(|(_, v)| v)).finish(),
			(true, false) => write!(f, "{{..}}"),
			(true, true) => write!(f, "[..]"),
		})
	}
}

impl PartialEq for Table {
	fn eq(&self, other: &Self) -> bool {
		Lua::get(|lua| lua.equals(self, other).unwrap_or(false))
	}
}

impl PartialOrd for Table {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Lua::get(|lua| match lua.equals(self, other) {
			None => None,
			Some(true) => Some(Ordering::Equal),
			Some(false) => match lua.less_than(self, other) {
				Some(false) => Some(Ordering::Greater),
				Some(true) => Some(Ordering::Less),
				None => None,
			},
		})
	}
}

impl<T: ToLua> FromIterator<T> for Table {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		iter.into_iter().fold(Self::new(), |tbl, value| {
			tbl.raw_push(value);
			tbl
		})
	}
}

impl<K: ToLua, V: ToLua> FromIterator<(K, V)> for Table {
	fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
		iter.into_iter().fold(Self::new(), |tbl, (key, value)| {
			tbl.raw_set(key, value);
			tbl
		})
	}
}

#[derive(Debug)]
pub struct Ipairs<'a> {
	table: &'a Table,
	key: usize,
}

impl Iterator for Ipairs<'_> {
	type Item = (usize, LuaValue);

	fn next(&mut self) -> Option<Self::Item> {
		match self.table.raw_get(self.key).not_nil() {
			Some(value) => {
				let key = self.key;
				self.key += 1;
				Some((key, value))
			}
			None => {
				self.key = 1;
				None
			}
		}
	}
}

#[derive(Debug)]
pub struct Pairs<'a> {
	table: &'a Table,
	key: LuaValue,
}

impl Iterator for Pairs<'_> {
	type Item = (LuaValue, LuaValue);

	fn next(&mut self) -> Option<Self::Item> {
		let key = std::mem::take(&mut self.key);
		self.table.next(key).map(|(key, value)| {
			self.key = key.clone();
			(key, value)
		})
	}
}
