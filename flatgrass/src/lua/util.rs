use crate::lua::Value;
use std::collections::vec_deque::{IntoIter, VecDeque};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Upvalue<T>(pub T);

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Yield<T>(pub T);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Return<T> {
	Values(T),
	Yield(T),
}

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple<T = Value> {
	deque: VecDeque<T>,
}

impl<T> Default for Tuple<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Tuple<T> {
	pub fn new() -> Self {
		Self {
			deque: VecDeque::new(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			deque: VecDeque::with_capacity(capacity),
		}
	}

	pub fn into_inner(self) -> VecDeque<T> {
		self.deque
	}
}

impl<T> Deref for Tuple<T> {
	type Target = VecDeque<T>;

	fn deref(&self) -> &Self::Target {
		&self.deque
	}
}

impl<T> DerefMut for Tuple<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.deque
	}
}

impl<T> FromIterator<T> for Tuple<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self {
			deque: VecDeque::from_iter(iter),
		}
	}
}

impl<T> Extend<T> for Tuple<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		self.deque.extend(iter);
	}
}

impl<T> IntoIterator for Tuple<T> {
	type IntoIter = IntoIter<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter {
		self.deque.into_iter()
	}
}
