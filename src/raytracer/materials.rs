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
  
  // Calculate the color of a point in 3D space using the Phong shading model
  pub fn lighting(material: &Material, 
                  light: &PointLight,
                  point: &Tuple,
                  eye_v: &Tuple,
                  normal_v: &Tuple,
                  in_shadow: bool) -> Color {
    let effective_color = &material.color * &light.intensity;
    let light_vector = (&(light.position) - &point).normalize();
    
    // The ambient component is constant in the Phong model
    let ambient = &effective_color *material.ambient;

    if in_shadow {
      return ambient;
    }
    let light_dot_normal = Tuple::dot(&light_vector, &normal_v);
    let mut diffuse = Color::new(0., 0., 0.);
    let mut specular = Color::new(0., 0., 0.);

    // The diffuse and specular components should only contribute if the angle
    // between the light and the normal vector at that point is less than pi/2
    if light_dot_normal > 0. {
      // The diffuse component relies only on the angle between the light source
      // and the surface normal:
      diffuse = effective_color * material.diffuse * light_dot_normal;
      // The specular component relies only on the angle between the 
      // light reflection vector and the eye vector
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