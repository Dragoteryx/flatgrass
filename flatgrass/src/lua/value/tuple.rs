use crate::lua::value::Value;
use std::collections::vec_deque::{IntoIter, VecDeque};
use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tuple<T = Value> {
	inner: VecDeque<T>,
}

impl<T> Default for Tuple<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Tuple<T> {
	pub fn new() -> Self {
		Self {
			inner: VecDeque::new(),
		}
	}

	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: VecDeque::with_capacity(capacity),
		}
	}

	pub fn into_inner(self) -> VecDeque<T> {
		self.inner
	}
}

impl<T: Debug> Debug for Tuple<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut dbg = f.debug_tuple("Tuple");
		for v in &self.inner {
			dbg.field(v);
		}

		dbg.finish()
	}
}

impl<T> Deref for Tuple<T> {
	type Target = VecDeque<T>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for Tuple<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

impl<T> FromIterator<T> for Tuple<T> {
	fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
		Self {
			inner: VecDeque::from_iter(iter),
		}
	}
}

impl<T> Extend<T> for Tuple<T> {
	fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
		self.inner.extend(iter);
	}
}

impl<T> IntoIterator for Tuple<T> {
	type IntoIter = IntoIter<T>;
	type Item = T;

	fn into_iter(self) -> Self::IntoIter {
		self.inner.into_iter()
	}
}
