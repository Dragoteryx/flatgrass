#![forbid(unsafe_code)]

use crate::prelude::*;

#[doc(inline)]
pub use crate::printfg;
mod macros;

pub mod structs;

pub fn print<T: AsRef<str>>(msg: T) {
	if let LuaValue::Function(print) = Table::globals().raw_get("print") {
		let msg = msg.as_ref();
		for line in msg.lines() {
			if print.call(line).is_err() {
				break;
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Realm {
	Server,
	Client,
	Menu,
}

impl Realm {
	pub fn get() -> Self {
		let globals = Table::globals();
		let server = globals.raw_get("SERVER").truthy();
		let client = globals.raw_get("CLIENT").truthy();
		let menu = globals.raw_get("MENU_DLL").truthy();
		match (server, client, menu) {
			(true, false, false) => Self::Server,
			(false, true, false) => Self::Client,
			(false, false, true) => Self::Menu,
			_ => panic!("invalid realm"),
		}
	}

	pub fn is_server() -> bool {
		Self::get() == Self::Server
	}

	pub fn is_client() -> bool {
		Self::get() == Self::Client
	}

	pub fn is_menu() -> bool {
		Self::get() == Self::Menu
	}
}
