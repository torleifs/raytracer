use super::geometry::Shape;
use super::PointLight;
use crate::color::Color;
use crate::raytracer::patterns::StripePattern;
use crate::raytracer::Sphere;
use crate::{math::Tuple, util::equal};
use std::cmp;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Material {
  pub color: Color,
  pub ambient: f64,
  pub diffuse: f64,
  pub specular: f64,
  pub shininess: f64,
  pub pattern: Option<StripePattern>,
}

impl Material {
  pub fn new() -> Material {
    Material {
      color: Color::new(1., 1., 1.),
      ambient: 0.1,
      diffuse: 0.9,
      specular: 0.9,
      shininess: 200.0,
      pattern: None,
    }
  }
  // Calculate the color of a point in 3D space using the Phong shading model
  pub fn lighting(
    material: &Material,
    object: Rc<dyn Shape>,
    light: &PointLight,
    point: &Tuple,
    eye_v: &Tuple,
    normal_v: &Tuple,
    in_shadow: bool,
  ) -> Color {
    let effective_color;
    if let Some(pattern) = &material.pattern {
      effective_color = &pattern.stripe_at_object(object.clone(), point) * &light.intensity;
    } else {
      effective_color = &material.color * &light.intensity;
    }

    let light_vector = (&(light.position) - &point).normalize();
    // The ambient component is constant in the Phong model
    let ambient = &effective_color * material.ambient;

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
        specular = &(light.intensity) * material.specular * factor;
      }
    }
    return ambient + &diffuse + &specular;
  }
}
impl cmp::PartialEq for Material {
  fn eq(&self, other: &Self) -> bool {
    self.color == other.color
      && equal(self.ambient, other.ambient)
      && equal(self.diffuse, other.diffuse)
      && equal(self.specular, other.specular)
      && equal(self.shininess, other.shininess)
  }
}

#[test]
fn light_with_pattern_applied() {
  let mut m = Material::new();
  m.pattern = Some(StripePattern::new(
    Color::new(1.0, 1.0, 1.0),
    Color::new(0.0, 0.0, 0.0),
  ));
  m.ambient = 1.0;
  m.diffuse = 0.0;
  m.specular = 0.0;
  let eye_vec = Tuple::vector(0.0, 0.0, -1.0);
  let normal_vec = Tuple::vector(0.0, 0.0, -1.0);
  let light = PointLight::new(&Tuple::point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
  let c1 = Material::lighting(
    &m,
    Rc::new(Sphere::new()),
    &light,
    &Tuple::point(0.9, 0.0, 0.0),
    &eye_vec,
    &normal_vec,
    false,
  );
  let c2 = Material::lighting(
    &m,
    Rc::new(Sphere::new()),
    &light,
    &Tuple::point(1.1, 0.0, 0.0),
    &eye_vec,
    &normal_vec,
    false,
  );

  assert_eq!(c1, Color::new(1.0, 1.0, 1.0));
  assert_eq!(c2, Color::new(0.0, 0.0, 0.0));
}
