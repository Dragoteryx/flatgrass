#[doc(hidden)]
#[macro_export]
macro_rules! call {
	($func:expr $(,)?) => {
		$crate::lua::Function::call(&$func, [] as [$crate::lua::Value; 0])
	};
	($func:expr, $($arg:expr),+ $(,)?) => {
		$crate::lua::Function::call(&$func, [$($crate::lua::ToLua::to_lua($arg)),+])
	};
}

#[doc(hidden)]
#[macro_export]
macro_rules! resume {
	($cor:expr $(,)?) => {
		$crate::lua::Coroutine::resume(&$cor, [] as [$crate::lua::Value; 0])
	};
	($cor:expr, $($arg:expr),+ $(,)?) => {
		$crate::lua::Coroutine::resume(&$cor, [$($crate::lua::ToLua::to_lua($arg)),+])
	};
}

/// Create a new table with the given values.
///
/// # Examples
///
/// This macro can be used to create a sequential table.
/// ```
/// let table = table![1, 2, 3];
/// ```
///
/// It can also be used to initialize a table of a specific length with a default value.
/// ```
/// let table = table![0; 10];
/// ```
///
/// A table can also be created as a map of key-value pairs.
/// ```
/// let table = table! {
///   key: "value",
///   key2: true,
/// };
/// ```
///
/// To use expressions as keys, surround them with square brackets.
/// ```
/// let table = table! {
///   ["key"]: "value",
///   [1 + 1]: "two",
/// };
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! table {
	($value:expr; $n:expr) => {{
		let table = $crate::lua::Table::new();
		let value = $crate::lua::ToLua::to_lua($value);
		for _ in 0usize..$n { table.raw_push(&value); }
		table
	}};
	($($value:expr),* $(,)?) => {{
		let table = $crate::lua::Table::new();
		$( table.raw_push($value); )*
		table
	}};
	($($key:ident : $value:expr),* $(,)?) => {{
		let table = $crate::lua::Table::new();
		$( table.raw_set(::core::stringify!($key), $value); )*
		table
	}};
	($([$key:expr] : $value:expr),* $(,)?) => {{
		let table = $crate::lua::Table::new();
		$( table.raw_set($key, $value); )*
		table
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! lua_function {
	(|_| $body:expr) => {
		$crate::lua::lua_function!(|lua| $body)
	};
	(|$lua:ident| $body:expr) => {
		$crate::ffi::raw_function!(|state| {
			let func = |$lua: &$crate::lua::Lua| {
				let res = $body;
				match $crate::lua::FnReturn::fn_return(res, $lua) {
					::core::result::Result::Ok($crate::lua::util::Return::Values(val)) => {
						::core::option::Option::Some($crate::lua::util::Return::Values(
							$lua.stack().push_many(val),
						))
					}
					::core::result::Result::Ok($crate::lua::util::Return::Yield(val)) => {
						::core::option::Option::Some($crate::lua::util::Return::Yield(
							$lua.stack().push_many(val),
						))
					}
					::core::result::Result::Err(err) => {
						$lua.stack().clear();
						$lua.stack().push_any(err);
						::core::option::Option::None
					}
				}
			};

			unsafe {
				match Lua::enter(state, func) {
					::core::option::Option::None => $crate::ffi::lua_error(state),
					::core::option::Option::Some(ret) => match ret {
						$crate::lua::util::Return::Yield(n) => $crate::ffi::lua_yield(state, n),
						$crate::lua::util::Return::Values(n) => n,
					},
				}
			}
		})
	};
}
