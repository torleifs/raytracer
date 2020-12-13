use crate::math::Tuple;
use crate::color::Color;

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
