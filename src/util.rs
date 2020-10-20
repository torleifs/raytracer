pub fn equal(a: f64, b: f64) -> bool {
  const EPSILON: f64 = 0.0001;
  if f64::abs(a - b) < EPSILON {
    return true;
  } else {
    return false;
  }
}