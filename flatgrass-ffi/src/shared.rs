use libloading::Library;
use std::sync::LazyLock;

#[cfg(any(fg_win32, fg_win64))]
use libloading::os::windows::Library as WindowsLibrary;

#[allow(unused_macros)]
macro_rules! open_library {
	($path:literal) => {
		Library::new($path).ok()
	};
}

#[cfg(any(fg_win32, fg_win64))]
fn opened_lua_shared() -> Option<Library> {
	WindowsLibrary::open_already_loaded("lua_shared.dll")
		.map(Library::from)
		.ok()
}

#[cfg(fg_win32)]
fn find_lua_shared() -> Option<Library> {
	unsafe {
		opened_lua_shared()
			.or_else(|| open_library!("bin/lua_shared.dll"))
			.or_else(|| open_library!("garrysmod/bin/lua_shared.dll"))
	}
}

#[cfg(fg_win64)]
fn find_lua_shared() -> Option<Library> {
	unsafe { opened_lua_shared().or_else(|| open_library!("bin/win64/lua_shared.dll")) }
}

#[cfg(fg_linux32)]
fn find_lua_shared() -> Option<Library> {
	unsafe {
		open_library!("garrysmod/bin/lua_shared_srv.so")
			.or_else(|| open_library!("garrysmod/bin/lua_shared.so"))
			.or_else(|| open_library!("bin/linux32/lua_shared.so"))
			.or_else(|| open_library!("bin/linux32/lua_shared_client.so"))
	}
}

#[cfg(fg_linux64)]
fn find_lua_shared() -> Option<Library> {
	unsafe {
		open_library!("bin/linux64/lua_shared.so")
			.or_else(|| open_library!("bin/linux64/lua_shared_client.so"))
	}
}

#[cfg(fg_unsupported)]
fn find_lua_shared() -> Option<Library> {
	compile_error!("this platform isn't supported");
	None
}

/// The `lua_shared` library.
pub static LUA_SHARED: LazyLock<Library> =
	LazyLock::new(|| find_lua_shared().expect("failed to open lua_shared"));
