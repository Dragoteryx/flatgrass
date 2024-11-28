#[doc(hidden)]
#[macro_export]
macro_rules! table {
	($($value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_push($value); )*
		table
	}};
	($($key:ident : $value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_set(::std::stringify!($key), $value); )*
		table
	}};
	($([$key:expr] : $value:expr),* $(,)?) => {{
		let table = $crate::lua::value::Table::new();
		$( table.raw_set($key, $value); )*
		table
	}};
	($value:expr; $n:expr) => {{
		let value = &$value;
		let table = $crate::lua::value::Table::new();
		for _ in 0usize..$n { table.raw_push(value); }
		table
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! globals {
	($($key:ident : $value:expr),* $(,)?) => {{
		let globals = $crate::lua::value::Table::globals();
		$( globals.raw_set(::std::stringify!($key), $value); )*
		globals
	}};
}

#[doc(hidden)]
#[macro_export]
macro_rules! func {
	($func:ident) => {
		$crate::lua::value::Function::new($func::to_lua)
	};
	($func:ident :: <$($ty:ty),*> $(,)?) => {
		$crate::lua::value::Function::new($func::to_lua::<$($ty),*>)
	};
}
