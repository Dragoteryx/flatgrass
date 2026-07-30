use std::cell::RefCell;
use std::thread::JoinHandle;
use tokio::runtime::{Handle, Builder};
use async_channel::{Sender, bounded};

#[derive(Debug)]
pub struct TokioRuntime {
	thread: RefCell<Option<JoinHandle<()>>>,
	shutdown: Sender<()>,
	handle: Handle,
}

impl TokioRuntime {
	pub fn new() -> Self {
		let tokio = Builder::new_current_thread()
			.enable_all()
			.build()
			.expect("Failed to create Tokio runtime");

		let handle = tokio.handle().clone();
		let (shutdown, receiver) = bounded(1);
		let thread = std::thread::spawn(move || {
			tokio.block_on(async move {
				let _ = receiver.recv().await;
			});
		});

		Self {
			thread: RefCell::new(Some(thread)),
			shutdown,
			handle,
		}
	}

	pub fn handle(&self) -> &Handle {
		&self.handle
	}

	pub fn shutdown(&self) {
		let _ = self.shutdown.try_send(());
		let thread = self.thread.borrow_mut().take();
		if let Some(thread) = thread {
			let _ = thread.join();
		}
	}
}