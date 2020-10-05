use std::ops;
#[derive(Clone)]
pub struct Color {
  tuple: Tuple,
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
// hadamard product
impl ops::Mul<Color> for Color {
  type Output = Color;
  fn mul(self, rhs: Color) -> Color {
    Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
  }
}
impl Color {
  pub fn new(r: f64, g: f64, b: f64) -> Color {
    Color::from_tuple(Tuple {
      x: r,
      y: g,
      z: b,
      w: 0.0,
    })
  }
  pub fn from_tuple(tuple: Tuple) -> Color {
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
    equal(self.r(), other.r()) && equal(self.g(), other.g()) && equal(self.b(), other.b())
  }
}
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
    Tuple::tuple(
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
    Tuple::tuple(
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
    Tuple::tuple(
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
    Tuple::tuple(
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
    Tuple::tuple(-self.x, -self.y, -self.z, -self.w)
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
    Tuple::tuple(
      self.x / magnitude,
      self.y / magnitude,
      self.z / magnitude,
      self.w / magnitude,
    )
  }
  pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
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

fn equal(a: f64, b: f64) -> bool {
  const EPSILON: f64 = 0.00001;
  f64::abs(a - b) < EPSILON
}

#[cfg(test)]
mod tests {
  use super::equal;
  use super::Color;
  use super::Tuple;
  #[test]
  fn a_tuple_with_w_1_is_a_point() {
    let a = Tuple::tuple(4.3, -4.2, 3.1, 1.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 1.0);
    assert!(a.is_point())
  }

  #[test]
  fn a_tuple_with_w_0_is_a_vector() {
    let a = Tuple::tuple(4.3, -4.2, 3.1, 0.0);
    assert_eq!(a.x, 4.3);
    assert_eq!(a.y, -4.2);
    assert_eq!(a.z, 3.1);
    assert_eq!(a.w, 0.0);
    assert!(a.is_vector())
  }

  #[test]
  fn create_point_w_is_1() {
    let a = Tuple::point(4.0, -4.0, 3.0);
    assert_eq!(a.w, 1.0);
  }
  #[test]
  fn create_vector_w_is_0() {
    let a = Tuple::vector(4.0, -4.0, 3.0);
    assert_eq!(a.w, 0.0);
  }
  #[test]
  fn adding_two_tuples() {
    let a = Tuple::tuple(3.0, -2.0, 5.0, 1.0);
    let b = Tuple::tuple(-2.0, 3.0, 1.0, 0.0);
    let answer = a + &b;
    let correct_answer = Tuple {
      x: 1.0,
      y: 1.0,
      z: 6.0,
      w: 1.0,
    };
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  #[should_panic]
  fn adding_two_points_causes_panic() {
    let a = Tuple::point(3.0, -2.0, 5.0);
    let b = Tuple::point(-2.0, 3.0, 1.0);
    let _ = a + &b;
  }
  #[test]
  fn subtracting_two_points_results_in_vector() {
    let a = Tuple::point(3.0, 2.0, 1.0);
    let b = Tuple::point(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn subtracting_vector_from_point_results_in_point() {
    let a = Tuple::point(3.0, 2.0, 1.0);
    let b = Tuple::vector(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::point(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn subtracting_vector_from_vector_results_in_vector() {
    let a = Tuple::vector(3.0, 2.0, 1.0);
    let b = Tuple::vector(5.0, 6.0, 7.0);
    let answer = a - &b;
    let correct_answer = Tuple::vector(-2.0, -4.0, -6.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn subtract_vector_from_zero_vector() {
    let zero = Tuple::vector(0.0, 0.0, 0.0);
    let b = Tuple::vector(1.0, -2.0, 3.0);
    let answer = zero - &b;
    let correct_answer = Tuple::vector(-1.0, 2.0, -3.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn negate_tuple() {
    let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
    let negated_tuple = -a;
    let correct_answer = Tuple::tuple(-1.0, 2.0, -3.0, 4.0);
    assert!(negated_tuple.is_equal(&correct_answer))
  }
  #[test]
  fn multiply_tuple_by_scalar() {
    let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
    let answer = a * 3.5;
    let correct_answer = Tuple::tuple(3.5, -7.0, 10.5, -14.0);
    assert!(answer.is_equal(&correct_answer));
  }

  #[test]
  fn multiply_tuple_by_fraction() {
    let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
    let answer = a * 0.5;
    let correct_answer = Tuple::tuple(0.5, -1.0, 1.5, -2.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn divide_tuple_by_scalar() {
    let a = Tuple::tuple(1.0, -2.0, 3.0, -4.0);
    let answer = a / 2.0;
    let correct_answer = Tuple::tuple(0.5, -1.0, 1.5, -2.0);
    assert!(answer.is_equal(&correct_answer));
  }
  #[test]
  fn magnitude_of_vectors() {
    let a = Tuple::vector(1.0, 0.0, 0.0);
    assert!(equal(a.magnitude(), 1.0));
    let a = Tuple::vector(0.0, 1.0, 0.0);
    assert!(equal(a.magnitude(), 1.0));
    let a = Tuple::vector(0.0, 0.0, 1.0);
    assert!(equal(a.magnitude(), 1.0));
    let a = Tuple::vector(1.0, 2.0, 3.0);
    assert!(equal(a.magnitude(), (14.0 as f64).sqrt()));
    let a = Tuple::vector(-1.0, -2.0, -3.0);
    assert!(equal(a.magnitude(), (14 as f64).sqrt()));
  }
  #[test]
  fn normalizing_vectors() {
    let a = Tuple::vector(4.0, 0.0, 0.0);
    let b = a.normalize();
    assert!(b.is_equal(&Tuple::vector(1.0, 0.0, 0.0)));
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = a.normalize();
    assert!(b.is_equal(&Tuple::vector(
      1.0 / (14.0 as f64).sqrt(),
      2.0 / (14.0 as f64).sqrt(),
      3.0 / (14.0 as f64).sqrt()
    )));
  }
  #[test]
  fn magnitude_of_normalized_vector() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = a.normalize();
    assert!(equal(b.magnitude(), 1.0));
  }
  #[test]
  fn dot_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    assert!(equal(Tuple::dot(&a, &b), 20.0));
  }

  #[test]
  fn cross_product() {
    let a = Tuple::vector(1.0, 2.0, 3.0);
    let b = Tuple::vector(2.0, 3.0, 4.0);
    let answer = Tuple::cross(&a, &b);
    assert!(answer.is_equal(&Tuple::vector(-1.0, 2.0, -1.0)));
    let answer = Tuple::cross(&b, &a);
    assert!(answer.is_equal(&Tuple::vector(1.0, -2.0, 1.0)));
  }

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
