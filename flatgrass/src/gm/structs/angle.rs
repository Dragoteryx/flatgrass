use super::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Angle {
	pub p: f32,
	pub y: f32,
	pub r: f32,
}

impl Angle {
	pub const ZERO: Self = Self::splat(0.0);

	pub const fn new(p: f32, y: f32, r: f32) -> Self {
		Self { p, y, r }
	}

	pub const fn splat(n: f32) -> Self {
		Self::new(n, n, n)
	}
}
