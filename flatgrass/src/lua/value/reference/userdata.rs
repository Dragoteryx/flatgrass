use super::*;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::rc::Rc;

#[repr(transparent)]
#[derive(Clone)]
pub struct Userdata {
	reference: Rc<Reference>,
}

impl LuaStack {
	pub fn push_userdata(&self, udata: &Userdata) {
		self.push_reference(&udata.reference);
	}

	pub fn pop_userdata(&self) -> Option<Userdata> {
		if self.get_type(-1) == Some(LuaType::Userdata) {
			unsafe { Some(self.pop_userdata_unchecked()) }
		} else {
			None
		}
	}

	pub unsafe fn pop_userdata_unchecked(&self) -> Userdata {
		Userdata {
			reference: Rc::new(self.pop_reference_unchecked()),
		}
	}

	pub fn get_userdata(&self, idx: i32) -> Option<Userdata> {
		if self.get_type(idx) == Some(LuaType::Userdata) {
			Some(unsafe { self.get_userdata_unchecked(idx) })
		} else {
			None
		}
	}

	pub unsafe fn get_userdata_unchecked(&self, idx: i32) -> Userdata {
		Userdata {
			reference: Rc::new(self.get_reference_unchecked(idx)),
		}
	}
}

impl Userdata {
	pub fn to_ptr(&self) -> *mut ffi::c_void {
		Lua::get(|lua| unsafe {
			let stack = lua.stack();
			stack.push_userdata(self);
			let ptr = ffi::lua_touserdata(lua.state(), -1);
			stack.pop_n(1);
			ptr
		})
	}
}

impl ToLua for Userdata {
	fn to_lua_by_ref(&self) -> LuaValue {
		self.clone().to_lua()
	}

	fn to_lua(self) -> LuaValue {
		LuaValue::Userdata(self)
	}
}

impl FromLua for Userdata {
	type Err = FromLuaError<'static>;

	fn from_lua(value: LuaValue) -> Result<Self, Self::Err> {
		if let LuaValue::Userdata(udata) = value {
			Ok(udata)
		} else {
			Err(FromLuaError::expected_and_got(
				LuaType::Userdata,
				value.get_type(),
			))
		}
	}

	fn no_value() -> Result<Self, Self::Err> {
		Err(FromLuaError::expected(LuaType::Userdata))
	}
}

impl Debug for Userdata {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Userdata[{:?}]", self.to_ptr())
	}
}

impl PartialEq for Userdata {
	fn eq(&self, other: &Self) -> bool {
		Lua::get(|lua| lua.equals(self, other).unwrap_or(false))
	}
}

impl PartialOrd for Userdata {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Lua::get(|lua| match lua.equals(self, other) {
			Some(true) => Some(Ordering::Equal),
			None => None,
			Some(false) => match lua.less_than(self, other) {
				Some(true) => Some(Ordering::Less),
				Some(false) => Some(Ordering::Greater),
				None => None,
			}
		})
	}
}