/// Creates a null-terminated string at compile time.
#[macro_export]
macro_rules! cstr {
	($str:literal) => {
		concat!($str, '\0').as_ptr() as *const i8
	}
}

/// Creates a new table and add fields to it.
#[macro_export]
macro_rules! table {
	($lua:expr) => {
		$crate::lua::LuaTable::new(&$lua)
	};
	($lua:expr, [$($value:expr),* $(,)?]) => {
		{
			let __tbl = $crate::lua::LuaTable::new(&$lua);
			$(__tbl.push($value);)*
			__tbl
		}
	};
	($lua:expr, [$($key:expr => $value:expr),* $(,)?]) => {
		{
			let __tbl = $crate::lua::LuaTable::new(&$lua);
			$(__tbl.insert($key, $value);)*
			__tbl
		}
	};
}

/// Prints output to the Garry's Mod console.
/// 
/// Uses the [`print`](https://wiki.facepunch.com/gmod/Global.print) function.
#[macro_export]
macro_rules! printfg {
	($lua:expr, $($arg:expr),*) => {
		$crate::lua::Lua::print(&$lua, &format!($($arg),*));
	};
}

#[macro_export]
macro_rules! errorfg {
	($lua:expr, $($arg:expr),*) => {
		$crate::lua::Lua::error(&$lua, &format!($($arg),*));
	};
}