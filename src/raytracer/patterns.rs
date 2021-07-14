use super::geometry::Shape;
use super::Sphere;
use crate::color::Color;
use crate::math;
use crate::math::Matrix;
use crate::math::Tuple;
use core::cell::RefCell;
use std::fmt;
use std::rc::Rc;

pub trait Pattern: fmt::Debug {
  fn stripe_at(&self, point: &Tuple) -> Color;
  fn stripe_at_object(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color;
}

#[derive(Clone, Debug)]
pub struct StripePattern {
  pub color_a: Color,
  pub color_b: Color,
  pub transform: math::Matrix,
}

impl StripePattern {
  pub fn new(color_a: Color, color_b: Color) -> StripePattern {
    Self {
      color_a,
      color_b,
      transform: math::Matrix::new_identity_matrix(4),
    }
  }
  pub fn new_with_transform(color_a: Color, color_b: Color, transform: Matrix) -> StripePattern {
    Self {
      color_a,
      color_b,
      transform: transform,
    }
  }
}
impl Pattern for StripePattern {
  fn stripe_at(&self, point: &Tuple) -> Color {
    match point.x.floor() as i64 % 2 {
      0 => self.color_a.clone(),
      _ => self.color_b.clone(),
    }
  }
  fn stripe_at_object(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color {
    let inverse_obj = object.get_transform().invert().unwrap();
    let object_space_point = &inverse_obj * point;
    let pattern_space_point = &self.transform.invert().unwrap() * &object_space_point;
    self.stripe_at(&pattern_space_point)
  }
}

fn black() -> Color {
  Color::new(0.0, 0.0, 0.0)
}

fn white() -> Color {
  Color::new(1.0, 1.0, 1.0)
}

#[test]
fn create_stripe_pattern() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.color_a, white());
  assert_eq!(pattern.color_b, black());
}

#[test]
fn stripe_pattern_constant_y() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 1.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 2.0, 0.0)), white());
}

#[test]
fn stripe_pattern_constant_z() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 0.0, 1.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 0.0, 2.0)), white());
}

#[test]
fn stripe_pattern_alternates_x() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(0.9, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(&Tuple::point(1.0, 0.0, 0.0)), black());

  assert_eq!(pattern.stripe_at(&Tuple::point(-0.1, 0.0, 0.0)), black());
  assert_eq!(pattern.stripe_at(&Tuple::point(-1.0, 0.0, 0.0)), black());
  assert_eq!(pattern.stripe_at(&Tuple::point(-1.1, 0.0, 0.0)), white());
}

#[test]
fn stripe_with_object_transform() {
  let mut object = Sphere::new();
  object.transform = RefCell::new(Matrix::scale(2.0, 2.0, 2.0));
  let pattern = StripePattern::new(white(), black());
  let c = pattern.stripe_at_object(Rc::new(object), &Tuple::point(1.5, 0., 0.));
  assert_eq!(c, white());
}

#[test]
fn stripe_with_pattern_transform() {
  let object = Rc::new(Sphere::new());
  let mut pattern = StripePattern::new(white(), black());
  pattern.transform = Matrix::scale(2.0, 2.0, 2.0);
  let c = pattern.stripe_at_object(object, &Tuple::point(1.5, 0., 0.));
  assert_eq!(c, white());
}

#[test]
fn stripe_with_object_and_pattern_transform() {
  let mut object = Sphere::new();
  object.transform = RefCell::new(Matrix::scale(2.0, 2.0, 2.0));
  let mut pattern = StripePattern::new(white(), black());
  pattern.transform = Matrix::translation(0.5, 0., 0.);
  let c = pattern.stripe_at_object(Rc::new(object), &Tuple::point(2.5, 0., 0.));
  assert_eq!(c, white());
}
