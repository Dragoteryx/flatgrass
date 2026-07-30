use crate::lua::Lua;
use async_channel::{Sender, bounded};
use async_executor::LocalExecutor;
use std::future::IntoFuture;

#[doc(inline)]
pub use async_executor::{FallibleTask, Task};

pub fn spawn<F: IntoFuture + 'static>(future: F) -> Task<F::Output> {
	Lua::get(|lua| lua.async_runtime().spawn(future))
}

#[derive(Debug)]
pub struct Runtime {
	executor: LocalExecutor<'static>,
	sender: Sender<()>,
}

impl Runtime {
	pub(crate) fn new() -> Self {
		let (sender, receiver) = bounded(1);
		std::thread::spawn(move || {
			async_io::block_on(async {
				let _ = receiver.recv().await;
			});
		});

		Self {
			executor: LocalExecutor::new(),
			sender,
		}
	}

	pub fn spawn<F: IntoFuture + 'static>(&self, future: F) -> Task<F::Output> {
		let future = future.into_future();
		self.executor.spawn(future)
	}

	pub fn tick(&self) {
		while self.executor.try_tick() {}
	}

	pub fn shutdown(&self) {
		let _ = self.sender.try_send(());
	}
}
