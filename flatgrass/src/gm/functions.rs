use super::*;

pub fn print<T: AsRef<str>>(msg: T) {
	if Lua::is_initialized() {
		let globals = Table::globals();
		let print = globals.raw_get("print");
		if let LuaValue::Function(print) = print {
			let msg = msg.as_ref();
			for line in msg.lines() {
				if print.call(line).is_err() {
					break;
				}
			}
		}
	}
}

pub fn curtime() -> f64 {
	if Lua::is_initialized() {
		let globals = Table::globals();
		let curtime = globals.raw_get("CurTime");
		if let LuaValue::Function(curtime) = curtime {
			if let Ok(mut res) = curtime.call(()) {
				if let Some(LuaValue::Number(time)) = res.pop_front() {
					return time;
				}
			}
		}
	}

	f64::NAN
}
