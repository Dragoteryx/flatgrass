use crate::ffi;
use crate::lua::Lua;
use std::error::Error;
use std::ffi::CStr;
use std::fmt::{self, Display};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BadArgumentError<T> {
	name: Option<String>,
	arg: i32,
	source: T,
}

impl<T> BadArgumentError<T> {
	pub fn new(mut arg: i32, source: T) -> Self {
		Lua::get(|lua| unsafe {
			let mut debug = ffi::lua_Debug::default();
			let name = if ffi::lua_getstack(lua.to_ptr(), 0, &mut debug) == 0 {
				None
			} else {
				ffi::lua_getinfo(lua.to_ptr(), c"n".as_ptr(), &mut debug);
				if ffi::libc::strcmp(debug.namewhat, c"method".as_ptr()) == 0 {
					arg -= 1;
				}

				if !debug.name.is_null() {
					let name = CStr::from_ptr(debug.name);
					Some(name.to_string_lossy().to_string())
				} else {
					None
				}
			};

			Self { name, arg, source }
		})
	}

	pub fn name(&self) -> Option<&str> {
		self.name.as_deref()
	}

	pub fn arg(&self) -> i32 {
		self.arg
	}

	pub fn source(&self) -> &T {
		&self.source
	}
}

impl<T: Error + 'static> Error for BadArgumentError<T> {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		Some(&self.source)
	}
}

impl<T: ToString> Display for BadArgumentError<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = self.source.to_string();
		match (self.name(), self.arg) {
			(Some(name), 0) => write!(f, "calling '{name}' on bad self ({msg})"),
			(Some(name), arg) => write!(f, "bad argument #{arg} to '{name}' ({msg})"),
			(None, 0) => write!(f, "bad self ({msg})"),
			(None, arg) => write!(f, "bad argument #{arg} ({msg})"),
		}
	}
}
