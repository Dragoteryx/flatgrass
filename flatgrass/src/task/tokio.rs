use futures_channel::oneshot::{Sender, channel};
use std::cell::RefCell;
use std::thread::JoinHandle;
use tokio::runtime::{Builder, Handle};

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
