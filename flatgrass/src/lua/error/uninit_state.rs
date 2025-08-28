use std::any::type_name;
use std::error::Error;
use std::fmt::{self, Display};

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitializedStateError {
	type_name: &'static str,
}

impl UnitializedStateError {
	pub fn new<T: 'static>() -> Self {
		Self {
			type_name: type_name::<T>(),
		}
	}

	pub fn type_name(&self) -> &'static str {
		self.type_name
	}
}

impl Error for UnitializedStateError {}
impl Display for UnitializedStateError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "uninitialized state of type '{}'", self.type_name)
	}
}
