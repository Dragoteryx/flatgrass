use super::*;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector {
	pub const ZERO: Self = Self::splat(0.0);

	pub const fn new(x: f32, y: f32, z: f32) -> Self {
		Self { x, y, z }
	}

	pub const fn splat(n: f32) -> Self {
		Self::new(n, n, n)
	}

	pub fn is_zero(self) -> bool {
		self == Self::ZERO
	}

	pub fn not_zero(self) -> Option<Self> {
		(!self.is_zero()).then_some(self)
	}

	pub fn distance_sqr(self, other: Self) -> f32 {
		(self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2)
	}

	pub fn distance(self, other: Self) -> f32 {
		self.distance_sqr(other).sqrt()
	}

	pub fn len_sqr(self) -> f32 {
		self.distance_sqr(Self::ZERO)
	}

	pub fn len(self) -> f32 {
		self.len_sqr().sqrt()
	}

	pub fn dot(self, other: Self) -> f32 {
		(self.x * other.x) + (self.y * other.y) + (self.z * other.z)
	}
}
