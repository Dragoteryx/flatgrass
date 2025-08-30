use crate::lua::value::{LuaString, LuaValue, Table};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{self, Serialize, SerializeMap, SerializeSeq, Serializer};
use std::fmt;

impl Serialize for LuaString {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_str().serialize(serializer)
	}
}

impl Serialize for Table {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.recurse(|depth| match depth > 0 {
			true => Err(ser::Error::custom("cannot serialize table with cycles")),
			false => {
				if self.is_sequential() {
					let mut seq = serializer.serialize_seq(Some(self.len()))?;
					self.ipairs()
						.try_for_each(|value| seq.serialize_element(&value))?;
					seq.end()
				} else {
					let mut map = serializer.serialize_map(Some(self.pairs().count()))?;
					self.pairs()
						.try_for_each(|(key, value)| map.serialize_entry(&key, &value))?;
					map.end()
				}
			}
		})
	}
}

impl Serialize for LuaValue {
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		match self {
			Self::Nil => ().serialize(serializer),
			Self::Bool(bl) => bl.serialize(serializer),
			Self::Number(num) => num.serialize(serializer),
			Self::String(lstr) => lstr.serialize(serializer),
			Self::Table(tbl) => tbl.serialize(serializer),
			Self::Function(_) => Err(ser::Error::custom("cannot serialize function")),
			Self::Userdata(_) => Err(ser::Error::custom("cannot serialize userdata")),
			Self::Coroutine(_) => Err(ser::Error::custom("cannot serialize coroutine")),
			Self::LightUserdata(_) => Err(ser::Error::custom("cannot serialize light userdata")),
		}
	}
}

impl<'de> Deserialize<'de> for LuaString {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		String::deserialize(deserializer).map(Self::from)
	}
}

impl<'de> Deserialize<'de> for Table {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_any(TableVisitor)
	}
}

impl<'de> Deserialize<'de> for LuaValue {
	fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		deserializer.deserialize_any(LuaValueVisitor)
	}
}

struct TableVisitor;
impl<'de> Visitor<'de> for TableVisitor {
	type Value = Table;

	fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "a Lua table")
	}

	fn visit_seq<A: de::SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
		let table = Table::new();
		while let Some(value) = seq.next_element::<LuaValue>()? {
			table.raw_push(value);
		}
		Ok(table)
	}

	fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
		let table = Table::new();
		while let Some((key, value)) = map.next_entry::<LuaValue, LuaValue>()? {
			table.raw_set(key, value);
		}
		Ok(table)
	}
}

struct LuaValueVisitor;
impl<'de> Visitor<'de> for LuaValueVisitor {
	type Value = LuaValue;

	fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "a Lua value")
	}

	fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
		Ok(LuaValue::Nil)
	}

	fn visit_bool<E: de::Error>(self, bl: bool) -> Result<Self::Value, E> {
		Ok(LuaValue::Bool(bl))
	}

	fn visit_i64<E: de::Error>(self, num: i64) -> Result<Self::Value, E> {
		Ok(LuaValue::Number(num as f64))
	}

	fn visit_u64<E: de::Error>(self, num: u64) -> Result<Self::Value, E> {
		Ok(LuaValue::Number(num as f64))
	}

	fn visit_f64<E: de::Error>(self, num: f64) -> Result<Self::Value, E> {
		Ok(LuaValue::Number(num))
	}

	fn visit_str<E: de::Error>(self, str: &str) -> Result<Self::Value, E> {
		Ok(LuaValue::String(str.into()))
	}

	fn visit_seq<A: de::SeqAccess<'de>>(self, seq: A) -> Result<Self::Value, A::Error> {
		TableVisitor.visit_seq(seq).map(LuaValue::Table)
	}

	fn visit_map<A: de::MapAccess<'de>>(self, map: A) -> Result<Self::Value, A::Error> {
		TableVisitor.visit_map(map).map(LuaValue::Table)
	}
}
