use crate::prelude::*;

#[doc(inline)]
pub use crate::printfg;
mod macros;

mod functions;
pub use functions::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Realm {
	Server,
	Client,
	Menu,
}

impl Realm {
	pub fn get() -> Option<Self> {
		if Lua::is_initialized() {
			let globals = Table::globals();
			let server = globals.raw_get("SERVER").truthy();
			let client = globals.raw_get("CLIENT").truthy();
			let menu = globals.raw_get("MENU_DLL").truthy();
			match (server, client, menu) {
				(true, false, false) => Some(Self::Server),
				(false, true, false) => Some(Self::Client),
				(false, false, true) => Some(Self::Menu),
				_ => None,
			}
		} else {
			None
		}
	}

	pub fn is_server() -> bool {
		Self::get() == Some(Self::Server)
	}

	pub fn is_client() -> bool {
		Self::get() == Some(Self::Client)
	}

	pub fn is_menu() -> bool {
		Self::get() == Some(Self::Menu)
	}
}
