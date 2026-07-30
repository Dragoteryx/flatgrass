use crate::lua::Lua;
use async_channel::{Sender, bounded};
use async_executor::LocalExecutor;
use async_io::Timer;
use std::cell::RefCell;
use std::future::IntoFuture;
use std::thread::JoinHandle;
use std::time::Duration;

#[doc(inline)]
pub use async_executor::{FallibleTask, Task};

pub fn spawn<F: IntoFuture + 'static>(future: F) -> Task<F::Output> {
	Lua::get(|lua| lua.async_runtime().spawn(future))
}

pub async fn sleep(duration: Duration) {
	Timer::after(duration).await;
}

#[derive(Debug)]
pub struct Runtime {
	executor: LocalExecutor<'static>,
	thread: RefCell<Option<JoinHandle<()>>>,
	sender: Sender<()>,
}

impl Runtime {
	pub(crate) fn new() -> Self {
		let (sender, receiver) = bounded(1);
		let thread = std::thread::spawn(move || {
			async_io::block_on(async {
				let _ = receiver.recv().await;
			});
		});

		Self {
			executor: LocalExecutor::new(),
			thread: RefCell::new(Some(thread)),
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
		let thread = self.thread.borrow_mut().take();
		if let Some(thread) = thread {
			let _ = thread.join();
		}
	}
}
