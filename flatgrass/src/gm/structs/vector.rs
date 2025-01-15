use super::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl FromLua for Vector {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		let userdata = Userdata::from_lua(value)?;
		unsafe {
			let raw_userdata = userdata.to_ptr();
			if (*raw_userdata).type_id != 10 {
				Err(FromLuaError::expected_and_got("vector", "userdata"))
			} else {
				Ok(*(*raw_userdata).data.cast())
			}
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected("vector"))
	}
}
