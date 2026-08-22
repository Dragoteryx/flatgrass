use crate::ffi;
use crate::lua::{Function, Lua, Table, Value, call};
use futures_channel::oneshot::{Receiver, channel};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

pub fn yield_now() -> Sleep {
	sleep(Duration::ZERO)
}

pub fn sleep_until(instant: Instant) -> Sleep {
	let now = Instant::now();
	if instant > now {
		sleep(instant - now)
	} else {
		yield_now()
	}
}

pub fn sleep(duration: Duration) -> Sleep {
	static WAKE_TIMER: ffi::lua_CFunction = ffi::raw_function!(|state| unsafe {
		Lua::enter(state, |lua| {
			let ptr = lua.stack().get_light_userdata(ffi::lua_upvalueindex(1))?;
			lua.async_runtime().timers.wake(ptr, ());
			Some(())
		});
		0
	});

	let (sender, receiver) = channel();
	Lua::try_get(|lua| {
		if let Some(lua) = lua {
			if let Value::Table(timer) = Table::globals().raw_get("timer") {
				if let Value::Function(timer_simple) = timer.raw_get("Simple") {
					let ptr = lua.async_runtime().timers.insert(sender);
					let func = Function::closure(WAKE_TIMER, [ptr]);
					let _ = call!(timer_simple, duration.as_secs_f64(), func);
				}
			}
		}
	});

	Sleep { receiver }
}

#[derive(Debug)]
#[repr(transparent)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Sleep {
	receiver: Receiver<()>,
}

impl Future for Sleep {
	type Output = ();

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
		match Pin::new(&mut self.receiver).poll(cx) {
			Poll::Ready(Err(_)) => panic!("sleep channel closed unexpectedly"),
			Poll::Ready(Ok(_)) => Poll::Ready(()),
			Poll::Pending => Poll::Pending,
		}
	}
}
