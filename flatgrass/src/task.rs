use std::time::Duration;

pub use crtn::executor::JoinHandle;

#[inline]
pub fn poll() -> usize {
	crtn::executor::poll()
}

#[inline]
pub fn shutdown() {
	crtn::executor::drop_tasks();
}

#[inline]
pub fn spawn<F: IntoFuture + 'static>(future: F) -> JoinHandle<F::Output> {
	crtn::executor::spawn(future)
}

#[inline]
pub fn spawn_blocking<F, T>(func: F) -> JoinHandle<T>
where
	F: FnOnce() -> T + Send + 'static,
	T: Send + 'static,
{
	crtn::executor::spawn(crtn::future::blocking(func))
}

#[inline]
pub async fn sleep(duration: Duration) {
	// todo: call timer.Simple instead
	spawn_blocking(move || std::thread::sleep(duration)).await
}