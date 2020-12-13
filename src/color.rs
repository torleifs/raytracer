use crate::math::{self, Tuple};
use std::cmp;
use std::ops;

use crate::util;

#[derive(Clone, Debug)]
pub struct Color {
  tuple: math::Tuple,
}
impl ops::Add<&Color> for Color {
  type Output = Color;
  fn add(self, rhs: &Color) -> Color {
    Color::from_tuple(self.tuple + &rhs.tuple)
  }
}
impl ops::Sub<Color> for Color {
  type Output = Color;
  fn sub(self, rhs: Color) -> Color {
    Color::from_tuple(self.tuple - &rhs.tuple)
  }
}
impl ops::Mul<f64> for Color {
  type Output = Color;
  fn mul(self, rhs: f64) -> Color {
    Color::from_tuple(self.tuple * rhs)
  }
}
impl ops::Mul<f64> for  &Color {
  type Output = Color;
  fn mul(self, rhs: f64) -> Color {
    Color::from_tuple(self.tuple.clone() * rhs)
  }
}

impl cmp::PartialEq for Color {
  fn eq(&self, other: &Self) -> bool {
    self.is_equal(other)
  }
}
// hadamard product
impl ops::Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
  }
}
impl<'a> ops::Mul for &'a Color {
  type Output = Color;
  fn mul(self, other: &'a Color) -> Color {
      Color::new(self.r() * other.r(), self.g() * other.g(), self.b() * other.b())
  }
}
impl<'a> ops::Mul<&Tuple> for &'a Color {
  type Output = Color;
  fn mul(self, other: & Tuple) -> Color {
      Color::new(self.r() * other.x, self.g() * other.y, self.b() * other.z)
  }
}
impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color::from_tuple(math::Tuple {
      x: r,
      y: g,
      z: b,
      w: 0.0,
    })
  }
  pub fn from_tuple(tuple: math::Tuple) -> Color {
    Color { tuple }
  }
  pub fn r(&self) -> f64 {
    self.tuple.x
  }
  pub fn g(&self) -> f64 {
    self.tuple.y
  }
  pub fn b(&self) -> f64 {
    self.tuple.z
  }
  pub fn is_equal(&self, other: &Color) -> bool {
    util::equal(self.r(), other.r())
      && util::equal(self.g(), other.g())
      && util::equal(self.b(), other.b())
  }
}
#[cfg(test)]
mod tests {
  use super::Color;
  #[test]
  fn add_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let answer = c1 + &c2;
    assert!(answer.is_equal(&Color::new(1.6, 0.7, 1.0)))
  }
  #[test]
  fn subtract_colors() {
    let c1 = Color::new(0.9, 0.6, 0.75);
    let c2 = Color::new(0.7, 0.1, 0.25);
    let answer = c1 - c2;
    assert!(answer.is_equal(&Color::new(0.2, 0.5, 0.5)))
  }

  #[test]
  fn multiply_color_with_scalar() {
    let c1 = Color::new(0.2, 0.3, 0.4);

    let answer = c1 * 2.0;
    assert!(answer.is_equal(&Color::new(0.4, 0.6, 0.8)))
  }

  #[test]
  fn multiply_colors() {
    let c1 = Color::new(1.0, 0.2, 0.4);
    let c2 = Color::new(0.9, 1.0, 0.1);
    let answer = c1 * c2;
    assert!(answer.is_equal(&Color::new(0.9, 0.2, 0.04)))
  }
}
