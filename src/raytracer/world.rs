use crate::raytracer::Intersection;
use std::rc::Rc;

use super::PointLight;
use super::{Material, Ray, Sphere};
use crate::color::Color;
use crate::math::Matrix;
use crate::math::Tuple;
use crate::raytracer::geometry::Shape;
use crate::util;
pub struct World {
  pub shapes: Vec<Rc<dyn Shape>>,
  pub lights: Vec<PointLight>,
}

impl World {
  pub fn new() -> World {
    let shapes: Vec<Rc<dyn Shape>> = Vec::new();
    let lights: Vec<PointLight> = Vec::new();
    World { shapes, lights }
  }
  pub fn default() -> World {
    let lights = vec![PointLight::new(
      &Tuple::point(-10., 10., -10.),
      &Color::new(1., 1., 1.),
    )];

    let mut s1 = Sphere::new();
    let mut mat = Material::new();
    mat.color = Color::new(0.8, 1.0, 0.6);
    mat.diffuse = 0.7;
    mat.specular = 0.2;
    s1.material = Rc::new(mat);
    let mut s2 = Sphere::new();
    s2.transform = Rc::new(Matrix::scale(0.5, 0.5, 0.5));
    World {
      shapes: vec![Rc::new(s1), Rc::new(s2)],
      lights,
    }
  }
  pub fn default_world_with_ambient_materials(ambience: f64) -> World {
    let lights = vec![PointLight::new(
      &Tuple::point(-10., 10., -10.),
      &Color::new(1., 1., 1.),
    )];
    let mut s1 = Sphere::new();
    let mut mat = Material::new();
    mat.color = Color::new(0.8, 1.0, 0.8);
    mat.diffuse = 0.7;
    mat.specular = 0.2;
    mat.ambient = ambience;
    s1.material = Rc::new(mat);
    let mut s2 = Sphere::new();
    s2.transform = Rc::new(Matrix::scale(0.5, 0.5, 0.5));
    mat = Material::new();
    mat.ambient = ambience;
    s2.material = Rc::new(mat);
    World {
      shapes: vec![Rc::new(s1), Rc::new(s2)],
      lights,
    }
  }
  pub fn shade_hit(&self, comps: &super::rays::PreComputation, remaining: u8) -> Color {
    let is_shadow = self.is_shadowed(&comps.over_point);
    let surface_color = Material::lighting(
      &comps.shape.get_material(),
      comps.shape.clone(),
      &self.lights[0],
      &comps.over_point,
      &comps.eye_vector,
      &comps.normal_vector,
      is_shadow,
    );
    let reflected_color = self.reflected_color(&comps, remaining - 1);
    return surface_color + &reflected_color;
  }
  pub fn intersect_world(&self, ray: &Ray) -> Vec<Intersection> {
    // Traverse all shapes, find intersections for all shapes
    // return a vector of intersections sorted on low t
    let mut vec: Vec<Intersection> = Vec::new();
    for shape in &self.shapes {
      let mut intersections = shape.intersect(ray);
      vec.append(&mut intersections);
    }
    vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    vec
  }
  pub fn color_at(&self, ray: &Ray, remaining: u8) -> Color {
    let xs = self.intersect_world(ray);
    if xs.len() < 1 {
      return Color::new(0., 0., 0.);
    }
    // use the intersection nearest to camera and find color at this point
    let maybe_t = xs.iter().find(|&i| i.t > 0.);
    if let Some(i) = maybe_t {
      let comps = Ray::precompute(&i, ray);
      return self.shade_hit(&comps, remaining);
    } else {
      return Color::new(0.0, 0.0, 0.0);
    }
  }

  // Determine if a point in 3D space is in shadow:
  // TODO: only considers the first light source for now.
  pub fn is_shadowed(&self, point: &Tuple) -> bool {
    let light_pos = &self.lights[0].position;
    let point_to_light = light_pos - point;
    let distance = point_to_light.magnitude();
    let point_to_light_normalized = point_to_light.normalize();

    // Determine if a point is in shadow by casting a ray *from* the point
    // *to* the light-source. A point will be in shadow if the ray intersects
    // at least one object for t E[0, distance>
    let point_to_light_ray = Ray::new(&point, &point_to_light_normalized);
    let mut intersections = self.intersect_world(&point_to_light_ray);
    let h = Intersection::hit(&mut intersections);
    let t = match h {
      Some(i) => i.t,
      None => -1.,
    };
    if t > 0. && t < distance {
      return true;
    } else {
      return false;
    }
  }

  pub fn reflected_color(
    &self,
    precomputation: &super::rays::PreComputation,
    remaining: u8,
  ) -> Color {
    if remaining < 1 {
      return Color::new(0.0, 0.0, 0.0);
    }
    if util::equal(precomputation.shape.get_material().reflective, 0.0) {
      return Color::new(0.0, 0.0, 0.0);
    }
    let reflect_ray = Ray::new(&precomputation.over_point, &precomputation.reflectv);
    let color = self.color_at(&reflect_ray, remaining);

    return color * precomputation.shape.get_material().reflective;
  }
}
