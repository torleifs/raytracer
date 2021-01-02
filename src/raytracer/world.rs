
use crate::raytracer::Intersection;
use std::rc::Rc;

use super::{Material, Ray, Sphere};

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
  pub fn default_world_with_ambient_materials(ambience: f64) -> World {
    let lights = vec![PointLight::new(&Tuple::point(-10., 10., -10.), &Color::new(1., 1., 1.))];
    
    let mut  s1 = Sphere::new();
    s1.material.color = Color::new(0.8, 1.0, 0.8);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    s1.material.ambient = ambience;
    let mut s2 = Sphere::new();
    s2.transform = Matrix::scale(0.5, 0.5, 0.5);
    s2.material.ambient = ambience;
    World {
      shapes: vec![Rc::new(s1), Rc::new(s2)],
      lights
    }
  }
  pub fn shade_hit(&self, comps: &super::rays::PreComputation) -> Color {
    Material::lighting(&comps.shape.get_material(), &self.lights[0], &comps.point, &comps.eye_vector, &comps.normal_vector)
  }
  pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
    let mut vec: Vec<Intersection> = Vec::new();
    for shape in &self.shapes {
      let mut intersections = Ray::intersects(&shape, ray);
      vec.append(&mut intersections);
    }
    vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    vec
  }
  pub fn color_at(&self, ray: &Ray) -> Color {
    let xs = self.intersect_world(ray);
    if xs.len() < 1 {
      return Color::new(0., 0., 0.);
    }
    let i = xs.iter().find(|&i| i.t > 0.).unwrap();
    let comps = Ray::precompute(&i, ray);
    return self.shade_hit(&comps);
  }
}
