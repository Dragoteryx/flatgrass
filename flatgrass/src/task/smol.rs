use async_channel::{Sender, bounded};
use std::cell::RefCell;
use std::thread::JoinHandle;

#[derive(Debug)]
pub struct SmolRuntime {
	thread: RefCell<Option<JoinHandle<()>>>,
	shutdown: Sender<()>,
}

impl SmolRuntime {
	pub fn new() -> Self {
		let (shutdown, receiver) = bounded(1);
		let thread = std::thread::spawn(move || {
			async_io::block_on(async move {
				let _ = receiver.recv().await;
			});
		});

		Self {
			thread: RefCell::new(Some(thread)),
			shutdown,
		}
	}

	pub fn shutdown(&self) {
		let _ = self.shutdown.try_send(());
		let thread = self.thread.borrow_mut().take();
		if let Some(thread) = thread {
			let _ = thread.join();
		}
	}
}
