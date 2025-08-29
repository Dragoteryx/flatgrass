use std::any::type_name;
use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{self, Display};
use std::ops::{Deref, DerefMut};

#[repr(transparent)]
#[derive(Default, Debug)]
pub struct StateManager {
	states: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl StateManager {
	pub fn set<T: 'static>(&mut self, state: T) {
		let state = RefCell::new(Box::new(state));
		self.states.insert(TypeId::of::<T>(), state);
	}

	pub fn get<T: 'static>(&self) -> Option<State<'_, T>> {
		self.states.get(&TypeId::of::<T>()).and_then(|cell| {
			let borrow = cell.try_borrow_mut().ok()?;
			let inner = RefMut::map(borrow, |value| value.downcast_mut::<T>().unwrap());
			Some(State { inner })
		})
	}

	pub fn get_ref<T: 'static>(&self) -> Option<StateRef<'_, T>> {
		self.states.get(&TypeId::of::<T>()).and_then(|cell| {
			let borrow = cell.try_borrow().ok()?;
			let inner = Ref::map(borrow, |value| value.downcast_ref::<T>().unwrap());
			Some(StateRef { inner })
		})
	}
}

#[repr(transparent)]
#[derive(Debug)]
pub struct State<'l, T> {
	inner: RefMut<'l, T>,
}

impl<T> Deref for State<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl<T> DerefMut for State<'_, T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

#[repr(transparent)]
#[derive(Debug)]
pub struct StateRef<'l, T> {
	inner: Ref<'l, T>,
}

impl<T> Deref for StateRef<'_, T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StateError {
	type_name: &'static str,
}

impl StateError {
	pub fn new<T: 'static>() -> Self {
		Self {
			type_name: type_name::<T>(),
		}
	}

	pub fn type_name(&self) -> &'static str {
		self.type_name
	}
}

impl Error for StateError {}
impl Display for StateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "cannot retrieve state of type '{}'", self.type_name)
	}
}
