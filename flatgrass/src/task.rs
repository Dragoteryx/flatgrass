use crate::lua::Lua;
use async_executor::LocalExecutor;
use std::future::IntoFuture;

#[doc(inline)]
pub use async_executor::{FallibleTask, Task};

#[cfg(feature = "tokio")]
mod tokio;

#[cfg(feature = "smol")]
mod smol;

pub fn spawn<F: IntoFuture + 'static>(future: F) -> Task<F::Output> {
	Lua::get(|lua| lua.async_runtime().spawn(future))
}

#[derive(Debug)]
pub struct Runtime {
	executor: LocalExecutor<'static>,
	#[cfg(feature = "tokio")]
	tokio: tokio::TokioRuntime,
	#[cfg(feature = "smol")]
	smol: smol::SmolRuntime,
}

impl Runtime {
	pub(crate) fn new() -> Self {
		Self {
			executor: LocalExecutor::new(),
			#[cfg(feature = "tokio")]
			tokio: tokio::TokioRuntime::new(),
			#[cfg(feature = "smol")]
			smol: smol::SmolRuntime::new(),
		}
	}

	pub fn spawn<F: IntoFuture + 'static>(&self, future: F) -> Task<F::Output> {
		let future = future.into_future();
		self.executor.spawn(future)
	}

	pub fn tick(&self) {
		#[cfg(feature = "tokio")]
		let _guard = self.tokio.handle().enter();
		while self.executor.try_tick() {}
	}

	pub fn shutdown(&self) {
		#[cfg(feature = "tokio")]
		self.tokio.shutdown();
		#[cfg(feature = "smol")]
		self.smol.shutdown();
	}
}
