use std::sync::atomic::{AtomicUsize, Ordering};

use crate::math;

static GLOBAL_GEOMETRY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Sphere {
  pub id: usize,
  pub transform: math::Matrix,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: math::Matrix::new_identity_matrix(4),
    }
  }
}
