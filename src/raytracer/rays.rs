use super::geometry::normal_at;
use crate::math::Matrix;
use crate::math::Tuple;
use crate::raytracer::geometry::Shape;
use crate::util;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Ray {
  pub origin: Tuple,
  pub direction: Tuple,
}

impl Ray {
  pub fn new(origin: &Tuple, direction: &Tuple) -> Ray {
    assert!(origin.is_point());
    assert!(direction.is_vector());

    Ray {
      origin: origin.clone(),
      direction: direction.clone(),
    }
  }
  pub fn position(ray: &Ray, t: f64) -> Tuple {
    &ray.origin + &(&ray.direction * t)
  }

  pub fn transform(&self, m: &Matrix) -> Ray {
    Ray {
      origin: m * &self.origin,
      direction: m * &self.direction,
    }
  }

  pub fn precompute(i: &Intersection, r: &Ray) -> PreComputation {
    let pos = Ray::position(r, i.t);
    let eye_vector = -r.direction.clone();
    let mut normal_vector = normal_at(i.shape.clone(), &pos);
    let inside = Tuple::dot(&normal_vector, &eye_vector) < 0.;
    if inside {
      normal_vector = -normal_vector;
    }
    let reflectv = Tuple::reflect(&r.direction, &normal_vector);
    PreComputation {
      t: i.t,
      shape: i.shape.clone(),
      point: pos.clone(),
      over_point: pos + &(&normal_vector * util::EPSILON),
      eye_vector,
      normal_vector,
      inside,
      reflectv,
    }
  }
}
pub struct PreComputation {
  pub t: f64,
  pub shape: Rc<dyn Shape>,
  pub point: Tuple,
  pub over_point: Tuple,
  pub eye_vector: Tuple,
  pub normal_vector: Tuple,
  pub inside: bool,
  pub reflectv: Tuple,
}

#[derive(Debug, Clone)]
pub struct Intersection {
  pub shape: Rc<dyn Shape>,
  pub t: f64,
}

impl Intersection {
  pub fn new(shape: &Rc<dyn Shape>, t: f64) -> Intersection {
    Intersection {
      shape: Rc::clone(&shape),
      t,
    }
  }
  pub fn intersections(the_intersections: &[Intersection]) -> Vec<Intersection> {
    the_intersections.to_vec()
  }
  pub fn hit(intersections: &mut Vec<Intersection>) -> Option<Intersection> {
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    for intersection in intersections.iter() {
      if intersection.t > 0. {
        return Some((*intersection).clone());
      }
    }
    return None;
  }
}
