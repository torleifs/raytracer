use std::cmp;
use std::ops;

use crate::util::equal;
#[derive(Debug, Clone)]
pub struct Tuple {
  pub x: f64,
  pub y: f64,
  pub z: f64,
  pub w: f64,
}

impl ops::Add<&Tuple> for Tuple {
  type Output = Tuple;
  fn add(self, other: &Tuple) -> Tuple {
    if self.is_point() && other.is_point() {
      panic!("Adding two points does not make sense");
    }
    Tuple::new(
      self.x + other.x,
      self.y + other.y,
      self.z + other.z,
      self.w + other.w,
    )
  }
}
impl ops::Sub<&Tuple> for Tuple {
  type Output = Tuple;
  fn sub(self, other: &Tuple) -> Tuple {
    Tuple::new(
      self.x - other.x,
      self.y - other.y,
      self.z - other.z,
      self.w - other.w,
    )
  }
}
impl ops::Mul<f64> for Tuple {
  type Output = Tuple;
  fn mul(self, scalar: f64) -> Tuple {
    Tuple::new(
      scalar * self.x,
      scalar * self.y,
      scalar * self.z,
      scalar * self.w,
    )
  }
}
impl ops::Div<f64> for Tuple {
  type Output = Tuple;
  fn div(self, scalar: f64) -> Tuple {
    Tuple::new(
      self.x / scalar,
      self.y / scalar,
      self.z / scalar,
      self.w / scalar,
    )
  }
}
impl ops::Neg for Tuple {
  type Output = Tuple;
  fn neg(self) -> Tuple {
    Tuple::new(-self.x, -self.y, -self.z, -self.w)
  }
}
impl cmp::PartialEq for Tuple {
  fn eq(&self, other: &Self) -> bool {
    self.is_equal(other)
  }
}
impl Tuple {
  pub fn is_point(&self) -> bool {
    equal(self.w, 1.0)
  }
  pub fn is_vector(&self) -> bool {
    equal(self.w, 0.0)
  }
  pub fn is_equal(&self, other: &Tuple) -> bool {
    equal(self.x, other.x)
      && equal(self.y, other.y)
      && equal(self.z, other.z)
      && equal(self.w, other.w)
  }
  pub fn magnitude(&self) -> f64 {
    (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
  }
  pub fn normalize(&self) -> Tuple {
    let magnitude = self.magnitude();
    Tuple::new(
      self.x / magnitude,
      self.y / magnitude,
      self.z / magnitude,
      self.w / magnitude,
    )
  }
  pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
  }
  pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
  }
  pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
  }
  pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
  }
  pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    Tuple::vector(
      a.y * b.z - a.z * b.y,
      a.z * b.x - a.x * b.z,
      a.x * b.y - a.y * b.x,
    )
  }
}