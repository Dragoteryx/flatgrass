use std::ops::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32
}