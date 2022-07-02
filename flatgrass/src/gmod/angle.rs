use std::ops::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Angle {
  pub p: f32,
  pub y: f32,
  pub r: f32
}