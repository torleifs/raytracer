use std::sync::atomic::{AtomicUsize, Ordering};

use crate::math;
use super::materials::Material;

static GLOBAL_GEOMETRY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub struct Sphere {
  pub id: usize,
  pub transform: math::Matrix,
  pub material: Material
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: math::Matrix::new_identity_matrix(4),
      material: Material::new()
    }
  }

  pub fn normal_at(&self, p: &math::Tuple) -> math::Tuple {
    let inverted_transform =  match self.transform.invert() {
      Some(i) => i,
      None => panic!(),
    };
    let point_in_object_space = &inverted_transform * p;
    let object_normal = point_in_object_space - &math::Tuple::point(0., 0., 0.);
    
    let mut world_normal = &inverted_transform.transpose() * &object_normal;
    world_normal.w = 0.;
    world_normal.normalize()
  }
}
