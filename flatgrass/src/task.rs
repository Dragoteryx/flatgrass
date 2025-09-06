use crate::lua::Lua;
pub use crtn::executor::JoinHandle;
use std::time::Duration;

#[inline]
pub fn spawn<F: IntoFuture + 'static>(future: F) -> JoinHandle<F::Output> {
	Lua::get(|lua| lua.spawn(future))
}

#[inline]
pub fn spawn_blocking<F, T>(func: F) -> JoinHandle<T>
where
	F: FnOnce() -> T + Send + 'static,
	T: Send + 'static,
{
	Lua::get(|lua| lua.spawn_blocking(func))
}

#[inline]
pub async fn yield_now() {
	crtn::future::yield_now().await;
}

#[inline]
pub async fn sleep(duration: Duration) {
	spawn_blocking(move || std::thread::sleep(duration)).await;
}

#[inline]
pub async fn sleep_ms(ms: u64) {
	sleep(Duration::from_millis(ms)).await;
}
