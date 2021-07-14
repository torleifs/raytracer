use super::materials::Material;
use super::rays::Intersection;
use super::rays::Ray;
use crate::math;
use crate::math::Tuple;
use crate::util;
use std::cell::RefCell;
use std::rc::Rc;
use std::{
  cmp, fmt,
  sync::atomic::{AtomicUsize, Ordering},
};

static GLOBAL_GEOMETRY_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn normal_at(shape: Rc<dyn Shape>, point: &math::Tuple) -> math::Tuple {
  let transform = shape.get_transform();
  let inverted_transform = match transform.invert() {
    Some(i) => i,
    None => panic!(),
  };
  let point_in_object_space = &inverted_transform * point;
  let local_normal = shape.local_normal_at(&point_in_object_space);

  // convert the normal back to world coordinates
  let mut world_normal = &inverted_transform.transpose() * &local_normal;
  world_normal.w = 0.;

  world_normal.normalize()
}

pub trait Shape: fmt::Debug {
  fn get_id(&self) -> usize;
  fn get_transform(&self) -> math::Matrix;
  fn set_transform(&self, transform: math::Matrix);
  fn get_material(&self) -> Rc<Material>;
  fn set_material(&mut self, material: Material);
  fn local_normal_at(&self, p: &math::Tuple) -> math::Tuple;
  fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
  fn get_saved_ray(&self) -> Ray;
}

#[derive(Debug, Clone)]
pub struct Sphere {
  pub id: usize,
  pub transform: RefCell<math::Matrix>,
  pub material: Rc<Material>,
  saved_ray: Option<Ray>,
}

impl Sphere {
  pub fn new() -> Sphere {
    Sphere {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: RefCell::new(math::Matrix::new_identity_matrix(4)),
      material: Rc::new(Material::new()),
      saved_ray: None,
    }
  }
}

impl Shape for Sphere {
  fn get_id(&self) -> usize {
    self.id
  }
  fn get_transform(&self) -> math::Matrix {
    let a = self.transform.clone();
    a.into_inner()
  }
  fn set_transform(&self, transform: math::Matrix) {
    self.transform.replace(transform);
  }
  fn get_material(&self) -> Rc<Material> {
    self.material.clone()
  }
  fn set_material(&mut self, material: Material) {
    self.material = Rc::new(material);
  }
  fn local_normal_at(&self, point_in_object_space: &math::Tuple) -> math::Tuple {
    // The normal at a point on the unit-sphere is the vector from (0,0,0) to the point in
    // object coordinates, so we must convert the point to object space before subtracting (0,0,0):
    point_in_object_space - &math::Tuple::point(0., 0., 0.)
  }

  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let inverse_sphere_transform = match self.get_transform().invert() {
      Some(i) => i,
      None => panic!(),
    };

    let ray = ray.transform(&inverse_sphere_transform);
    let sphere_to_ray = &ray.origin - &Tuple::point(0., 0., 0.);

    // Solve the quadratic equation resulting from:
    // |X|^2 = R^2 (sphere), X are all points on sphere
    //  X = ray.origin + t* D, X are all points on Ray. D is ray direction
    let a = Tuple::dot(&ray.direction, &ray.direction);
    let b = 2. * Tuple::dot(&ray.direction, &sphere_to_ray);
    let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.;

    let discriminant = b * b - 4. * (a * c);
    let mut vec = Vec::with_capacity(2);
    if discriminant < 0. {
      return vec;
    }
    let t1 = (-b - discriminant.sqrt()) / (2. * a);
    let t2 = (-b + discriminant.sqrt()) / (2. * a);
    vec.push(Intersection {
      t: t1,
      shape: Rc::new(self.clone()),
    });
    vec.push(Intersection {
      t: t2,
      shape: Rc::new(self.clone()),
    });

    vec
  }
  fn get_saved_ray(&self) -> Ray {
    self.saved_ray.clone().unwrap()
  }
}

impl cmp::PartialEq for Sphere {
  fn eq(&self, other: &Self) -> bool {
    self.transform == other.transform && self.material == other.material
  }
}

#[derive(Debug, Clone)]
pub struct TestShape {
  pub id: usize,
  pub transform: RefCell<math::Matrix>,
  pub material: Rc<Material>,
  saved_ray: RefCell<Option<Ray>>,
}
impl TestShape {
  pub fn new() -> TestShape {
    TestShape {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: RefCell::new(math::Matrix::new_identity_matrix(4)),
      material: Rc::new(Material::new()),
      saved_ray: RefCell::new(None),
    }
  }
}
impl Shape for TestShape {
  fn get_id(&self) -> usize {
    self.id
  }

  fn get_transform(&self) -> math::Matrix {
    let a = self.transform.clone();
    a.into_inner()
  }
  fn set_transform(&self, transform: math::Matrix) {
    self.transform.replace(transform);
  }
  fn set_material(&mut self, material: Material) {
    self.material = Rc::new(material);
  }
  fn get_material(&self) -> Rc<Material> {
    self.material.clone()
  }

  fn local_normal_at(&self, point_in_object_space: &math::Tuple) -> math::Tuple {
    // The normal at a point on the unit-sphere is the vector from (0,0,0) to the point in
    // object coordinates, so we must convert the point to object space before subtracting (0,0,0):
    point_in_object_space - &math::Tuple::point(0., 0., 0.)
  }

  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let inverse_sphere_transform = match self.get_transform().invert() {
      Some(i) => i,
      None => panic!(),
    };

    let ray = ray.transform(&inverse_sphere_transform);
    self.saved_ray.replace(Some(ray.clone()));
    let vec = Vec::with_capacity(2);
    vec
  }
  fn get_saved_ray(&self) -> Ray {
    self.saved_ray.clone().into_inner().unwrap()
  }
}

#[derive(Debug, Clone)]
pub struct Plane {
  pub id: usize,
  pub transform: RefCell<math::Matrix>,
  pub material: Rc<Material>,
  saved_ray: RefCell<Option<Ray>>,
}
impl Plane {
  pub fn new() -> Plane {
    Plane {
      id: GLOBAL_GEOMETRY_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
      transform: RefCell::new(math::Matrix::new_identity_matrix(4)),
      material: Rc::new(Material::new()),
      saved_ray: RefCell::new(None),
    }
  }
}
impl Shape for Plane {
  fn get_id(&self) -> usize {
    self.id
  }

  fn get_transform(&self) -> math::Matrix {
    let a = self.transform.clone();
    a.into_inner()
  }
  fn set_transform(&self, transform: math::Matrix) {
    self.transform.replace(transform);
  }
  fn set_material(&mut self, material: Material) {
    self.material = Rc::new(material);
  }
  fn get_material(&self) -> Rc<Material> {
    self.material.clone()
  }

  fn local_normal_at(&self, _: &math::Tuple) -> math::Tuple {
    Tuple::vector(0.0, 1.0, 0.0)
  }

  fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
    let inverse_transform = match self.get_transform().invert() {
      Some(i) => i,
      None => panic!(),
    };
    let ray = ray.transform(&inverse_transform);
    self.saved_ray.replace(Some(ray.clone()));
    let mut vec = Vec::with_capacity(1);
    if f64::abs(ray.direction.y) < util::EPSILON {
      return vec;
    } else {
      vec.push(Intersection {
        t: -ray.origin.y / ray.direction.y,
        shape: Rc::new(self.clone()),
      });
      return vec;
    }
  }
  fn get_saved_ray(&self) -> Ray {
    self.saved_ray.clone().into_inner().unwrap()
  }
}
