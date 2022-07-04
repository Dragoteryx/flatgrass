/// Creates a null-terminated string at compile time.
#[macro_export]
macro_rules! cstr {
	($str:literal) => {
		concat!($str, '\0').as_ptr() as *const i8
	}
}

/// Prints output to the Garry's Mod console.
/// 
/// Uses the [`print`](https://wiki.facepunch.com/gmod/Global.print) function.
#[macro_export]
macro_rules! printfg {
	($luaprint:expr, $($arg:expr),*) => {
		$crate::lua::Lua::print(&$luaprint, format!($($arg),*))
	};
}

#[macro_export]
macro_rules! errorfg {
	($luaprint:expr, $($arg:expr),*) => {
		$crate::lua::Lua::error(&$luaprint, format!($($arg),*))
	};
}