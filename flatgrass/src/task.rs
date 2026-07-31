use crate::lua::Lua;
use avenir::{blocking, Executor};
use std::cell::Cell;
#[cfg(feature = "tokio")]
use ::tokio::runtime::Handle;

#[doc(inline)]
pub use avenir::Task;

#[cfg(feature = "tokio")]
mod tokio;

pub fn spawn<F: IntoFuture + 'static>(future: F) -> Task<F::Output> {
	Lua::get(|lua| lua.async_runtime().spawn(future))
}

pub fn spawn_blocking<F, T>(func: F) -> Task<T>
where
	F: FnOnce() -> T + Send + 'static,
	T: Send + 'static,
{
	Lua::get(|lua| lua.async_runtime().spawn_blocking(func))
}

#[derive(Debug)]
pub struct Runtime {
	executor: Executor<'static>,
	shutdown: Cell<bool>,
	#[cfg(feature = "tokio")]
	tokio: tokio::TokioRuntime,
}

impl Runtime {
	pub(crate) fn new() -> Self {
		Self {
			executor: Executor::new(),
			shutdown: Cell::new(false),
			#[cfg(feature = "tokio")]
			tokio: tokio::TokioRuntime::new(),
		}
	}

	pub(crate) fn tick(&self) {
		if !self.shutdown.get() {
			self.executor.tick();
		}
	}

	pub(crate) fn shutdown(&self) {
		if self.shutdown.replace(true) {
			self.executor.clear();
			#[cfg(feature = "tokio")]
			self.tokio.shutdown();
		}
	}

	pub fn spawn<F: IntoFuture + 'static>(&self, future: F) -> Task<F::Output> {
		if self.shutdown.get() {
			panic!("Cannot spawn tasks on a shutdown runtime");
		} else {
			self.executor.spawn(future)
		}
	}

	pub fn spawn_blocking<F, T>(&self, func: F) -> Task<T>
	where
		F: FnOnce() -> T + Send + 'static,
		T: Send + 'static,
	{
		let future = blocking(func);
		self.spawn(future)
	}

	#[cfg(feature = "tokio")]
	pub fn tokio_handle(&self) -> &Handle {
		self.tokio.handle()
	}
}
