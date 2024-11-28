use super::*;
use std::fmt::{self, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub const WHITE: Self = Self::new(255, 255, 255);
	pub const BLACK: Self = Self::new(0, 0, 0);
	pub const RED: Self = Self::new(255, 0, 0);
	pub const GREEN: Self = Self::new(0, 255, 0);
	pub const BLUE: Self = Self::new(0, 0, 255);
	pub const SERVER: Self = Self::new_alpha(156, 241, 255, 200);
	pub const CLIENT: Self = Self::new_alpha(255, 241, 122, 200);
	pub const MENU: Self = Self::new_alpha(100, 220, 100, 200);

	pub const fn new(r: u8, g: u8, b: u8) -> Self {
		Self::new_alpha(r, g, b, 255)
	}

	pub const fn new_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
		Self { r, g, b, a }
	}

	pub fn realm() -> Self {
		match Realm::get() {
			Realm::Server => Self::SERVER,
			Realm::Client => Self::CLIENT,
			Realm::Menu => Self::MENU,
		}
	}
}

impl Default for Color {
	fn default() -> Self {
		Self::BLACK
	}
}

impl Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match (self.a != 255, f.alternate()) {
			(false, false) => write!(f, "#{:02x}{:02x}{:02x}", self.r, self.g, self.b),
			(false, true) => write!(f, "#{:02X}{:02X}{:02X}", self.r, self.g, self.b),
			(true, false) => write!(
				f,
				"#{:02x}{:02x}{:02x}{:02x}",
				self.r, self.g, self.b, self.a
			),
			(true, true) => write!(
				f,
				"#{:02X}{:02X}{:02X}{:02X}",
				self.r, self.g, self.b, self.a
			),
		}
	}
}

/*impl FromLua for Color {
	type Error = FromLuaError;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Error> {
		let err = |_| FromLuaError::UnexpectedValue { expected: "Color", got: "table" };

		let table = Table::from_lua(value)?;
		let r = table.get("r").try_into().map_err(err)?;
		let g = table.get("g").try_into().map_err(err)?;
		let b = table.get("b").try_into().map_err(err)?;
		let a = table.get("a").try_into::<Option<u8>>().map_err(err)?;
		Ok(Self { r, g, b, a: a.unwrap_or(255) })
	}

	fn no_value() -> Result<Self, Self::Error> {
		Err(FromLuaError::UnexpectedValue {
			expected: "Color",
			got: "no value"
		})
	}
}*/

/*impl ToLua for Color {
	fn to_lua_by_ref(&self) -> LuaValue {
		let color = crate::table![
			r: self.r,
			g: self.g,
			b: self.b,
			a: self.a,
		];

		let color_mt = unsafe {
			Table::registry()
				.get("Color")
				.try_into()
				.expect("the 'Color' metatable is missing")
		};

		//color.set_metatable(&color_mt);
		color.to_lua()
	}
}*/
