use std::{fmt, cmp, sync::atomic::{AtomicUsize, Ordering}};

use crate::math;
use super::materials::Material;

static GLOBAL_GEOMETRY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);


pub trait Shape: fmt::Debug {
  fn get_id(&self) -> usize;
  fn get_transform(&self) -> math::Matrix;
  fn get_material(&self) -> Material; 
  fn normal_at(&self, p: &math::Tuple) -> math::Tuple;
  fn box_clone(&self) -> Box<dyn Shape>;
}

#[derive(Debug, Clone)]
pub struct Sphere {
  pub id: usize,
  pub transform: math::Matrix,
  pub material: Material
}

impl Clone for Box<dyn Shape>
{
    fn clone(&self) -> Box<dyn Shape> {
        self.box_clone()
    }
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: math::Matrix::new_identity_matrix(4),
      material: Material::new()
    }
  }
}

impl Shape for Sphere {
  fn get_id(&self) -> usize {
    self.id
  }
  fn get_transform(&self) -> math::Matrix {
    self.transform.clone()
  }
  fn get_material(&self) -> Material {
    self.material.clone()
  }

  fn normal_at(&self, p: &math::Tuple) -> math::Tuple {
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

  fn box_clone(&self) -> Box<dyn Shape> {
    Box::new((*self).clone())
}
}

impl cmp::PartialEq for Sphere {
  fn eq(&self, other: &Self) -> bool {
    self.transform == other.transform &&
    self.material == other.material
  }
}