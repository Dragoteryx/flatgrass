use libloading::Library;
use std::path::Path;
use std::sync::LazyLock;

#[allow(unused_macros)]
macro_rules! find_lua_path {
	($($path:literal),+ $(,)?) => {
		[ $($path),+ ]
			.into_iter()
			.map(Path::new)
			.find(|path| path.exists())
	};
}

#[cfg(fg_win32)]
fn lua_path() -> Option<&'static Path> {
	find_lua_path! {
		"garrysmod/bin/lua_shared.dll",
		"bin/lua_shared.dll",
	}
}

#[cfg(fg_win64)]
fn lua_path() -> Option<&'static Path> {
	find_lua_path! {
		"bin/win64/lua_shared.dll",
	}
}

#[cfg(fg_linux32)]
fn lua_path() -> Option<&'static Path> {
	find_lua_path! {
		"garrysmod/bin/lua_shared_srv.so",
		"garrysmod/bin/lua_shared.so",
		"bin/linux32/lua_shared.so",
		"bin/linux32/lua_shared_client.so",
	}
}

#[cfg(fg_linux64)]
fn lua_path() -> Option<&'static Path> {
	find_lua_path! {
		"bin/linux64/lua_shared.so",
		"bin/linux64/lua_shared_client.so",
	}
}

#[cfg(fg_unsupported)]
fn lua_path() -> Option<&'static Path> {
	compile_error!("this platform isn't supported");
	None
}

/// The `lua_shared` library.
pub static LUA_SHARED: LazyLock<Library> = LazyLock::new(|| {
	lua_path()
		.and_then(|path| unsafe { Library::new(path).ok() })
		.expect("failed to open lua_shared")
});
