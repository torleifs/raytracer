use crate::color::Color;
use crate::math::Tuple;

pub struct StripePattern {
  pub color_a: Color,
  pub color_b: Color
}

impl StripePattern {
  pub fn new(color_a: Color, color_b: Color) -> StripePattern {
    Self  {
      color_a,
      color_b
    }
  }
  pub fn stripe_at(&self, point: Tuple) -> Color {
    match point.x.floor() as i64 % 2 {
      0 => self.color_a.clone(),
      _ => self.color_b.clone()
    }
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
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 1.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 2.0, 0.0)), white());
}


#[test]
fn stripe_pattern_constant_z() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 1.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 2.0)), white());
}

#[test]
fn stripe_pattern_alternates_x() {
  let pattern = StripePattern::new(white(), black());
  assert_eq!(pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(0.9, 0.0, 0.0)), white());
  assert_eq!(pattern.stripe_at(Tuple::point(1.0, 0.0, 0.0)), black());

  assert_eq!(pattern.stripe_at(Tuple::point(-0.1, 0.0, 0.0)), black());
  assert_eq!(pattern.stripe_at(Tuple::point(-1.0, 0.0, 0.0)), black());
  assert_eq!(pattern.stripe_at(Tuple::point(-1.1, 0.0, 0.0)), white());
}