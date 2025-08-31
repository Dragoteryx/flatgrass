/// Creates a raw Lua function using the closure syntax.\
/// This isn't an actual closure, hence you cannot capture variables.
///
/// # Examples
///
/// ```
/// let func: lua_CFunction = raw_function!(|state| unsafe {
///     let a = ffi::lua_tonumber(state, 1);
///     let b = ffi::lua_tonumber(state, 2);
///     ffi::lua_pushnumber(state, a + b);
///     1
/// });
/// ```
#[macro_export]
macro_rules! raw_function {
	(|$state:ident| $body:expr) => {{
		unsafe extern "C-unwind" fn func($state: *mut $crate::lua_State) -> $crate::libc::c_int {
			$body
		}

		func as $crate::lua_CFunction
	}};
	(|_| $body:expr) => {{
		unsafe extern "C-unwind" fn func(_: *mut $crate::lua_State) -> $crate::libc::c_int {
			$body
		}

		func as $crate::lua_CFunction
	}};
}

/// Used to import functions from the Lua C API.
#[macro_export]
macro_rules! import_lua {
	($($tokens:tt)*) => {
		$crate::import_lua_inner! {
			$($tokens)*
		}
	};
}

/// Inner implementation of the import_lua macro.
#[doc(hidden)]
#[macro_export]
macro_rules! import_lua_inner {
	() => {};
	(
		$(#[$meta:meta])*
		$vis:vis $(unsafe)? fn $name:ident($($arg:ident: $argty:ty),* $(,)?) $(-> $ret:ty)? $body:block

		$($rest:tt)*
	) => {
		$(#[$meta])*
		#[inline]
		#[allow(non_snake_case, clippy::missing_safety_doc)]
		$vis unsafe fn $name($($arg: $argty),*) $(-> $ret)? $body

		$crate::import_lua_inner! {
			$($rest)*
		}
	};
	(
		$(#[$meta:meta])*
		$vis:vis $(unsafe)? fn $name:ident($($arg:ident: $argty:ty),* $(,)?) $(-> $ret:ty)?;

		$($rest:tt)*
	) => {
		$(#[$meta])*
		#[allow(non_snake_case, clippy::missing_safety_doc)]
		$vis unsafe fn $name($($arg: $argty),*) $(-> $ret)? {
			static FUNC: ::std::sync::LazyLock<unsafe extern "C-unwind" fn($($argty),*) $(-> $ret)?> = ::std::sync::LazyLock::new(|| unsafe {
				*$crate::LUA_SHARED.get(::std::stringify!($name).as_bytes()).expect(::std::concat!("could not find '", ::std::stringify!($name), "'"))
			});

			unsafe {
				FUNC($($arg),*)
			}
		}

		$crate::import_lua_inner! {
			$($rest)*
		}
	};
}
