pub const EPSILON: f64 = 0.0001;
pub fn equal(a: f64, b: f64) -> bool {
  
  if f64::abs(a - b) < EPSILON {
    return true;
  } else {
    return false;
  }
}