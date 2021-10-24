use super::geometry::Shape;
use super::Sphere;
use crate::color::Color;
use crate::math;
use crate::math::Matrix;
use crate::math::Tuple;

use std::fmt;
use std::rc::Rc;

pub trait Pattern: fmt::Debug {
  fn pattern_at(&self, point: &Tuple) -> Color;
  fn pattern_at_shape_from_transform(
    &self,
    object: Rc<dyn Shape>,
    point: &Tuple,
    pattern_transform: &Matrix,
  ) -> Color {
    let inverse_obj = object.get_transform().invert().unwrap();
    let object_space_point = &inverse_obj * point;
    let pattern_space_point = &pattern_transform.invert().unwrap() * &object_space_point;
    self.pattern_at(&pattern_space_point)
  }
  fn pattern_at_shape(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color;
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
  fn pattern_at(&self, point: &Tuple) -> Color {
    match point.x.floor() as i64 % 2 {
      0 => self.color_a.clone(),
      _ => self.color_b.clone(),
    }
  }
  fn pattern_at_shape(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color {
    self.pattern_at_shape_from_transform(object, point, &self.transform)
  }
}

#[derive(Clone, Debug)]
pub struct GradientPattern {
  pub color_a: Color,
  pub color_b: Color,
  pub transform: math::Matrix,
}
impl GradientPattern {
  pub fn new(color_a: Color, color_b: Color) -> GradientPattern {
    Self {
      color_a,
      color_b,
      transform: math::Matrix::new_identity_matrix(4),
    }
  }
  pub fn new_with_transform(color_a: Color, color_b: Color, transform: Matrix) -> GradientPattern {
    Self {
      color_a,
      color_b,
      transform: transform,
    }
  }
}
impl Pattern for GradientPattern {
  fn pattern_at(&self, point: &Tuple) -> Color {
    let distance = self.color_b.clone() - &self.color_a;
    let fraction = point.x - point.x.floor();
    self.color_a.clone() + &(distance * fraction)
  }
  fn pattern_at_shape(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color {
    self.pattern_at_shape_from_transform(object, point, &self.transform)
  }
}

#[derive(Clone, Debug)]
pub struct RingPattern {
  pub color_a: Color,
  pub color_b: Color,
  pub transform: math::Matrix,
}
impl RingPattern {
  pub fn new(color_a: Color, color_b: Color) -> RingPattern {
    Self {
      color_a,
      color_b,
      transform: math::Matrix::new_identity_matrix(4),
    }
  }
  pub fn new_with_transform(color_a: Color, color_b: Color, transform: Matrix) -> RingPattern {
    Self {
      color_a,
      color_b,
      transform: transform,
    }
  }
}

impl Pattern for RingPattern {
  fn pattern_at(&self, point: &Tuple) -> Color {
    if ((point.x * point.x + point.z * point.z).sqrt().floor() as i64) % 2 == 0 {
      return self.color_a.clone();
    } else {
      return self.color_b.clone();
    }
  }
  fn pattern_at_shape(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color {
    self.pattern_at_shape_from_transform(object, point, &self.transform)
  }
}

#[derive(Clone, Debug)]
pub struct CheckersPattern {
  pub color_a: Color,
  pub color_b: Color,
  pub transform: math::Matrix,
}
impl CheckersPattern {
  pub fn new(color_a: Color, color_b: Color) -> CheckersPattern {
    Self {
      color_a,
      color_b,
      transform: math::Matrix::new_identity_matrix(4),
    }
  }
  pub fn new_with_transform(color_a: Color, color_b: Color, transform: Matrix) -> CheckersPattern {
    Self {
      color_a,
      color_b,
      transform: transform,
    }
  }
}

impl Pattern for CheckersPattern {
  fn pattern_at(&self, point: &Tuple) -> Color {
    if ((point.x.floor() + point.y.floor() + point.z.floor()) as i64) % 2 == 0 {
      return self.color_a.clone();
    } else {
      return self.color_b.clone();
    }
  }
  fn pattern_at_shape(&self, object: Rc<dyn Shape>, point: &Tuple) -> Color {
    self.pattern_at_shape_from_transform(object, point, &self.transform)
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
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 1.0, 0.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 2.0, 0.0)), white());
}

#[test]
fn stripe_pattern_constant_z() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.0, 1.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.0, 2.0)), white());
}

#[test]
fn stripe_pattern_alternates_x() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.9, 0.0, 0.0)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(1.0, 0.0, 0.0)), black());

  assert_eq!(pattern.pattern_at(&Tuple::point(-0.1, 0.0, 0.0)), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(-1.0, 0.0, 0.0)), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(-1.1, 0.0, 0.0)), white());
}

#[test]
fn stripe_with_object_transform() {
  let mut object = Sphere::new();
  object.transform = Rc::new(Matrix::scale(2.0, 2.0, 2.0));
  let pattern = StripePattern::new(white(), black());
  let c = pattern.pattern_at_shape(Rc::new(object), &Tuple::point(1.5, 0., 0.));
  assert_eq!(c, white());
}

#[test]
fn stripe_with_pattern_transform() {
  let object = Rc::new(Sphere::new());
  let mut pattern = StripePattern::new(white(), black());
  pattern.transform = Matrix::scale(2.0, 2.0, 2.0);
  let c = pattern.pattern_at_shape(object, &Tuple::point(1.5, 0., 0.));
  assert_eq!(c, white());
}

#[test]
fn stripe_with_object_and_pattern_transform() {
  let mut object = Sphere::new();
  object.transform = Rc::new(Matrix::scale(2.0, 2.0, 2.0));
  let mut pattern = StripePattern::new(white(), black());
  pattern.transform = Matrix::translation(0.5, 0., 0.);
  let c = pattern.pattern_at_shape(Rc::new(object), &Tuple::point(2.5, 0., 0.));
  assert_eq!(c, white());
}

#[test]
fn gradient_linearly_interpolates_between_colors() {
  let pattern = GradientPattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), white());
  assert_eq!(
    pattern.pattern_at(&Tuple::point(0.25, 0., 0.)),
    Color::new(0.75, 0.75, 0.75)
  );
  assert_eq!(
    pattern.pattern_at(&Tuple::point(0.5, 0., 0.)),
    Color::new(0.5, 0.5, 0.5)
  );
  assert_eq!(
    pattern.pattern_at(&Tuple::point(0.75, 0., 0.)),
    Color::new(0.25, 0.25, 0.25)
  );
}

#[test]
fn ring_should_extend_in_both_x_and_z() {
  let pattern = RingPattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(1., 0., 0.)), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 1.)), black());
  // 0.708 just slightly more than sqrt(2)/2
  assert_eq!(pattern.pattern_at(&Tuple::point(0.708, 0., 0.708)), black());
}

#[test]
fn checkers_should_repeat_in_x() {
  let pattern = CheckersPattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.99, 0., 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(1.01, 0., 0.)), black());
}

#[test]
fn checkers_should_repeat_in_y() {
  let pattern = CheckersPattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0.99, 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 1.01, 0.)), black());
}
#[test]
fn checkers_should_repeat_in_z() {
  let pattern = CheckersPattern::new(white(), black());
  assert_eq!(pattern.pattern_at(&Tuple::point(0., 0., 0.)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0., 0.99)), white());
  assert_eq!(pattern.pattern_at(&Tuple::point(0.0, 0., 1.01)), black());
}
