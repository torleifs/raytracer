
use std::rc::Rc;

use super::Sphere;

use super::PointLight;
use crate::math::Tuple;
use crate::math::Matrix;
use crate::color::Color;
use crate::raytracer::geometry::Shape;
pub struct World {
  pub shapes: Vec<Rc<dyn Shape>>,
  pub lights: Vec<PointLight>
}

impl World {
  pub fn new() -> World {
    let shapes: Vec<Rc<dyn Shape>> = Vec::new();
    let lights: Vec<PointLight> = Vec::new();
    World {
      shapes,
      lights
    }
  }
  pub fn default() -> World {
    let lights = vec![PointLight::new(&Tuple::point(-10., 10., -10.), &Color::new(1., 1., 1.))];
    
    let mut  s1 = Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.8);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    let mut s2 = Sphere::new();
    s2.transform = Matrix::scale(0.5, 0.5, 0.5);
    
    World {
      shapes: vec![Rc::new(s1), Rc::new(s2)],
      lights
    }
  }
}
