use std::cmp;

use crate::math::Tuple;
use crate::color::Color;

#[derive(Debug)]
pub struct PointLight {
  pub position: Tuple,
  pub intensity: Color,
}

impl PointLight {
  pub fn new(position: &Tuple, intensity: &Color) -> PointLight {
   PointLight {
     position: position.clone(),
     intensity: intensity.clone()
   }
  } 
}

impl cmp::PartialEq for PointLight {
  fn eq(&self, other: &Self) -> bool {
    self.position == other.position &&
    self.intensity == other.intensity
  }
}