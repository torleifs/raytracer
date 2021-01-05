use std::cmp;

use crate::{math::Tuple, util::equal};
use crate::color::Color;

use super::PointLight;

#[derive(Clone, Debug)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
}

impl Material {
  pub fn new() -> Material {
   Material {
     color: Color::new(1., 1.,1.),
     ambient: 0.1,
     diffuse: 0.9,
     specular: 0.9,
     shininess: 200.0,
   }
  } 
  
  pub fn lighting(material: &Material, light: &PointLight, point: &Tuple, eye_v: &Tuple, normal_v: &Tuple, in_shadow: bool) -> Color{
    let effective_color = &material.color * &light.intensity;
    let light_vector = (&(light.position) - &point).normalize();
    let ambient = &effective_color *material.ambient;

    if in_shadow {
      return ambient;
    }
    let light_dot_normal = Tuple::dot(&light_vector, &normal_v);
    let mut diffuse = Color::new(0., 0., 0.);
    let mut specular = Color::new(0., 0., 0.);
    if light_dot_normal > 0. {
      diffuse = effective_color * material.diffuse * light_dot_normal;
      let reflect_vector = Tuple::reflect(&-light_vector, normal_v);
      let reflect_dot_eye = Tuple::dot(&reflect_vector, &eye_v);
      if reflect_dot_eye > 0. {
        let factor = reflect_dot_eye.powf(material.shininess);
        specular = &(light.intensity )* material.specular * factor;
      }
    }
    return ambient + &diffuse + &specular;
  }
}
impl cmp::PartialEq for Material {
  fn eq(&self, other: &Self) -> bool {
    self.color == other.color &&
    equal(self.ambient, other.ambient) &&
    equal(self.diffuse, other.diffuse) &&
    equal(self.specular, other.specular) &&
    equal(self.shininess, other.shininess)
  }
}