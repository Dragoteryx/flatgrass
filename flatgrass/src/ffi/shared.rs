use libloading::Library;
use once_cell::sync::Lazy;
use std::path::PathBuf;

#[cfg(all(target_os = "windows", target_arch = "x86"))]
pub static LUA_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
  [ "garrysmod/bin/lua_shared.dll",
    "bin/lua_shared.dll"].into_iter()
    .map(|str| PathBuf::from(str))
    .find(|path| path.exists())
});

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
pub static LUA_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
  Some(PathBuf::from("bin/win64/lua_shared.dll"))
});

#[cfg(all(target_os = "linux", target_arch = "x86"))]
pub static LUA_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
  [ "garrysmod/bin/lua_shared_srv.so",
    "garrysmod/bin/lua_shared.so",
    "bin/linux32/lua_shared.so",
    "bin/linux32/lua_shared_client.so"].into_iter()
    .map(|str| PathBuf::from(str))
    .find(|path| path.exists())
});

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
pub static LUA_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
  [ "bin/linux64/lua_shared.so",
    "bin/linux64/lua_shared_client.so"].into_iter()
    .map(|str| PathBuf::from(str))
    .find(|path| path.exists())
});

#[cfg(all(target_os = "macos"))]
pub static LUA_PATH: Lazy<Option<PathBuf>> =Lazy::new(|| {
  Some(PathBuf::from("garrysmod/bin/lua_shared.dylib"))
});

pub static LUA_SHARED: Lazy<Library> = Lazy::new(|| {
  LUA_PATH.as_ref()
    .and_then(|path| unsafe { Library::new(path).ok() })
    .expect("Could not open lua_shared.dll")
});