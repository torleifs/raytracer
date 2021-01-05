use crate::raytracer::geometry::Shape;
use crate::math::Tuple;
use crate::math::Matrix;
use crate::util;
use std::rc::Rc;
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
  pub fn intersects(shape: &Rc<dyn Shape>, ray: &Ray) -> Vec<Intersection> {
    let inverse_sphere_transform = match shape.get_transform().invert() {
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
      shape: Rc::clone(&shape),
    });
    vec.push(Intersection {
      t: t2,
      shape:  Rc::clone(&shape),
    });

    vec
  }
 

  pub fn transform(&self, m: &Matrix) -> Ray {
    Ray {
      origin: m * &self.origin,
      direction: m * &self.direction,
    }
  }


  pub fn precompute(i: &Intersection, r: &Ray)-> PreComputation {
    let pos = Ray::position(r, i.t);
    let eye_vector = - r.direction.clone();
    let mut normal_vector= i.shape.normal_at(&pos);
    let inside = Tuple::dot(&normal_vector, &eye_vector) < 0.;
    if inside {
      normal_vector = - normal_vector;
    }
    PreComputation {
      t: i.t,
      shape: i.shape.clone(),
      point: pos.clone(),
      over_point: pos + &(&normal_vector * util::EPSILON),
      eye_vector,
      normal_vector,
      inside,
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
  pub inside: bool
}
#[derive( Debug, Clone)]
pub struct Intersection {
  pub shape: Rc<dyn Shape>,
  pub t: f64,
}

impl Intersection {
  pub fn new(shape: &Rc<dyn Shape>, t: f64) -> Intersection {
    Intersection { shape: Rc::clone(&shape), t }
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


