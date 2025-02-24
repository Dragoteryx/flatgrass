/// Creates a raw Lua function using the closure syntax.
/// ```
/// let func: lua_CFunction = raw_function!(|state| {
///     let a = ffi::lua_tonumber(state, 1);
///     let b = ffi::lua_tonumber(state, 2);
///     ffi::lua_pushnumber(state, a + b);
///     1
/// });
/// ```
#[macro_export]
macro_rules! raw_function {
	(|_| $body:expr) => {{
		unsafe extern "C-unwind" fn func(_: *mut $crate::lua_State) -> $crate::libc::c_int {
			$body
		}

		func as $crate::lua_CFunction
	}};
	(|$state:ident| $body:expr) => {{
		unsafe extern "C-unwind" fn func(
			$state: *mut $crate::lua_State,
		) -> $crate::libc::c_int {
			$body
		}

		func as $crate::lua_CFunction
	}};
}

/// Used to import functions & macros from the Lua C API.
#[macro_export]
macro_rules! import_lua {
	() => {};
	(
		$(#[$meta:meta])*
		$vis:vis $(unsafe)? fn $name:ident($($arg:ident: $argty:ty),* $(,)?) $(-> $ret:ty)?;

		$($rest:tt)*
	) => {
		$(#[$meta])*
		#[allow(non_snake_case, clippy::missing_safety_doc)]
		$vis unsafe fn $name($($arg: $argty),*) $(-> $ret)? {
			use ::std::sync::LazyLock;

			static FUNC: LazyLock<unsafe extern "C-unwind" fn($($argty),*) $(-> $ret)?> = LazyLock::new(|| unsafe {
				*$crate::LUA_SHARED.get(stringify!($name).as_bytes()).expect(concat!("could not find '", stringify!($name), "'"))
			});

			unsafe {
				FUNC($($arg),*)
			}
		}

		$crate::import_lua! {
			$($rest)*
		}
	};
	(
		$(#[$meta:meta])*
		$vis:vis $(unsafe)? fn $name:ident($($arg:ident: $argty:ty),* $(,)?) $(-> $ret:ty)? $body:block

		$($rest:tt)*
	) => {
		$(#[$meta])*
		#[inline]
		#[allow(non_snake_case, clippy::missing_safety_doc)]
		$vis unsafe fn $name($($arg: $argty),*) $(-> $ret)? $body

		$crate::import_lua! {
			$($rest)*
		}
	};
}
