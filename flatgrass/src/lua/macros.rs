/// Create a new function from the given Rust function.
#[doc(hidden)]
#[macro_export]
macro_rules! func {
	($func:ident) => {
		$crate::lua::value::Function::new($func::to_lua)
	};
	($func:ident :: <$($ty:ty),* $(,)?>) => {
		$crate::lua::value::Function::new($func::to_lua::<$($ty),*>)
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
	($($value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_push($value); )*
		table
	}};
	($value:expr; $n:expr) => {{
		let table = $crate::lua::value::Table::new();
		let value = $crate::lua::traits::ToLua::to_lua($value);
		for _ in 0usize..$n { table.raw_push(&value); }
		table
	}};
	($($key:ident : $value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_set(::core::stringify!($key), $value); )*
		table
	}};
	($([$key:expr] : $value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_set($key, $value); )*
		table
	}};
}
