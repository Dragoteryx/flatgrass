use super::*;

pub fn print(msg: &str) {
	if Lua::is_valid() {
		let globals = Table::globals();
		let print = globals.raw_get("print");
		if let Value::Function(print) = print {
			for line in msg.lines() {
				if call!(print: line).is_err() {
					break;
				}
			}
		}
	}
}

pub fn curtime() -> f64 {
	if Lua::is_valid() {
		let globals = Table::globals();
		let curtime = globals.raw_get("CurTime");
		if let Value::Function(curtime) = curtime {
			if let Ok(mut res) = call!(curtime) {
				if let Some(Value::Number(time)) = res.pop_front() {
					return time;
				}
			}
		}
	}

	f64::NAN
}
