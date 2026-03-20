use crate::lua::Lua;
pub use avenir::JoinHandle;

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
	avenir::yield_now().await;
}
