use super::Sphere;
use crate::math;

pub struct Ray {
  pub origin: math::Tuple,
  pub direction: math::Tuple,
}

impl Ray {
  pub fn new(origin: &math::Tuple, direction: &math::Tuple) -> Ray {
    assert!(origin.is_point());
    assert!(direction.is_vector());

    Ray {
      origin: origin.clone(),
      direction: direction.clone(),
    }
  }
  pub fn position(ray: &Ray, t: f64) -> math::Tuple {
    &ray.origin + &(&ray.direction * t)
  }
  pub fn intersects(sphere: &Sphere, ray: &Ray) -> Vec<Intersection> {
    let inverse_sphere_transform = match sphere.transform.invert() {
      Some(i) => i,
      None => panic!(),
    };

    let ray = ray.transform(&inverse_sphere_transform);
    let sphere_to_ray = &ray.origin - &math::Tuple::point(0., 0., 0.);

    // Solve the quadratic equation resulting from:
    // |X|^2 = R^2 (sphere), X are all points on sphere
    //  X = ray.origin + t* D, X are all points on Ray. D is ray direction
    let a = math::Tuple::dot(&ray.direction, &ray.direction);
    let b = 2. * math::Tuple::dot(&ray.direction, &sphere_to_ray);
    let c = math::Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.;

    let discriminant = b * b - 4. * (a * c);
    let mut vec = Vec::with_capacity(2);
    if discriminant < 0. {
      return vec;
    }
    let t1 = (-b - discriminant.sqrt()) / (2. * a);
    let t2 = (-b + discriminant.sqrt()) / (2. * a);
    vec.push(Intersection {
      t: t1,
      object_id: sphere.id,
    });
    vec.push(Intersection {
      t: t2,
      object_id: sphere.id,
    });

    vec
  }
  pub fn transform(&self, m: &math::Matrix) -> Ray {
    Ray {
      origin: m * &self.origin,
      direction: m * &self.direction,
    }
  }
}
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Intersection {
  pub object_id: usize,
  pub t: f64,
}

impl Intersection {
  pub fn new(object_id: usize, t: f64) -> Intersection {
    Intersection { object_id, t }
  }
  pub fn intersections<'a>(the_intersections: &[&'a Intersection]) -> Vec<&'a Intersection> {
    the_intersections.to_vec()
  }
  pub fn hit(intersections: &mut Vec<&Intersection>) -> Option<Intersection> {
    intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    for intersection in intersections.iter() {
      if intersection.t > 0. {
        return Some((*intersection).clone());
      }
    }
    return None;
  }
}
