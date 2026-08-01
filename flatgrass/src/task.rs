use crate::lua::Lua;
use avenir::{Executor, blocking};
use std::cell::Cell;

#[doc(inline)]
pub use avenir::Task;

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
	tokio: TokioRuntime,
}

impl Runtime {
	pub(crate) fn new() -> Self {
		Self {
			executor: Executor::new(),
			shutdown: Cell::new(false),
			#[cfg(feature = "tokio")]
			tokio: TokioRuntime::new(),
		}
	}

	pub(crate) fn tick(&self) {
		if !self.shutdown.get() {
			self.executor.tick();
		}
	}

	pub(crate) fn shutdown(&self) {
		if !self.shutdown.replace(true) {
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
		self.spawn(future).detach()
	}

	#[cfg(feature = "tokio")]
	pub fn tokio_handle(&self) -> &Handle {
		self.tokio.handle()
	}
}

#[cfg(feature = "tokio")]
use tokio_runtime::*;

#[cfg(feature = "tokio")]
mod tokio_runtime {
	use futures_channel::oneshot::{Sender, channel};
	use std::cell::RefCell;
	use std::thread::JoinHandle;
	use tokio::runtime::Builder;
	pub use tokio::runtime::Handle;

	#[derive(Debug)]
	pub struct TokioRuntime {
		shutdown: RefCell<Option<(Sender<()>, JoinHandle<()>)>>,
		handle: Handle,
	}

	impl TokioRuntime {
		pub fn new() -> Self {
			let tokio = Builder::new_current_thread()
				.enable_all()
				.build()
				.expect("Failed to create Tokio runtime");

			let (sender, receiver) = channel();
			let handle = tokio.handle().clone();
			let thread = std::thread::spawn(move || {
				tokio.block_on(async move {
					let _ = receiver.await;
				});
			});

			let shutdown = (sender, thread);
			Self {
				shutdown: RefCell::new(Some(shutdown)),
				handle,
			}
		}

		pub fn handle(&self) -> &Handle {
			&self.handle
		}

		pub fn shutdown(&self) {
			let shutdown = self.shutdown.borrow_mut().take();
			if let Some((sender, thread)) = shutdown {
				let _ = sender.send(());
				let _ = thread.join();
			}
		}
	}
}
