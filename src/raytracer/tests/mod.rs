use super::Intersection;
use super::Ray;
use super::Sphere;
use crate::math::Matrix;
use crate::math::Tuple;
use crate::util;

#[test]
fn create_query_ray() {
  let origin = Tuple::point(1., 2., 3.);
  let direction = Tuple::vector(4., 5., 6.);

  let ray = Ray::new(&origin, &direction);
  assert!(ray.origin.is_equal(&origin));
  assert!(ray.direction.is_equal(&direction));
}

#[test]
fn computing_point_from_distance() {
  let r = Ray::new(&Tuple::point(2., 3., 4.), &Tuple::vector(1., 0., 0.));
  assert!(Tuple::point(2., 3., 4.).is_equal(&Ray::position(&r, 0.)));
  assert!(Tuple::point(3., 3., 4.).is_equal(&Ray::position(&r, 1.)));
  assert!(Tuple::point(1., 3., 4.).is_equal(&Ray::position(&r, -1.)));
  assert!(Tuple::point(4.5, 3., 4.).is_equal(&Ray::position(&r, 2.5)));
}

#[test]
fn ray_intersects_sphere_at_two_points() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);
  assert_eq!(xs.len(), 2);
  assert!(util::equal(xs[0].t, 4.0));
  assert!(util::equal(xs[1].t, 6.0))
}
#[test]
fn ray_intersects_sphere_at_tangent() {
  let r = Ray::new(&Tuple::point(0., 1., -5.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);
  assert_eq!(xs.len(), 2);
  // return two points even if tangential!
  assert!(util::equal(xs[0].t, 5.0));
  assert!(util::equal(xs[1].t, 5.0))
}

#[test]
fn ray_misses_sphere() {
  let r = Ray::new(&Tuple::point(0., 2., -5.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);
  assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_within_sphere() {
  let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);
  assert!(util::equal(xs[0].t, -1.0));
  assert!(util::equal(xs[1].t, 1.0))
}

#[test]
fn sphere_is_behind_ray() {
  let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);
  assert!(util::equal(xs[0].t, -6.0));
  assert!(util::equal(xs[1].t, -4.0))
}

#[test]
fn intersection_has_t_and_object() {
  let s = Sphere::new();
  let i = Intersection::new(s.id, 3.5);
  assert_eq!(s.id, i.object_id);
}

#[test]
fn aggregate_intersections() {
  let s = Sphere::new();
  let i1 = Intersection::new(s.id, 1.);
  let i2 = Intersection::new(s.id, 2.);
  let xs = Intersection::intersections(&[&i1, &i2]);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].object_id, s.id);
  assert_eq!(xs[1].object_id, s.id);
}

#[test]
fn intersect_sets_intersected_object() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let s = Sphere::new();
  let xs = Ray::intersects(&s, &r);

  assert_eq!(xs[0].object_id, s.id);
  assert_eq!(xs[1].object_id, s.id);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
  let s = Sphere::new();
  let i1 = Intersection {
    t: 1.,
    object_id: s.id,
  };
  let i2 = Intersection {
    t: 2.,
    object_id: s.id,
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[&i1, &i2])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
  assert_eq!(i.t, i1.t);
}

#[test]
fn the_hit_when_domr_intersections_have_negative_t() {
  let s = Sphere::new();
  let i1 = Intersection {
    t: -1.,
    object_id: s.id,
  };
  let i2 = Intersection {
    t: 2.,
    object_id: s.id,
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[&i1, &i2])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
  assert_eq!(i.t, i2.t);
}

#[test]
#[should_panic]
fn the_hit_when_all_intersections_have_negative_t() {
  let s = Sphere::new();
  let i1 = Intersection {
    t: -2.,
    object_id: s.id,
  };
  let i2 = Intersection {
    t: -1.,
    object_id: s.id,
  };
  match Intersection::hit(&mut Intersection::intersections(&[&i1, &i2])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
}

#[test]
fn the_hit_is_the_lowest_nonnegative_intersection() {
  let s = Sphere::new();
  let i1 = Intersection {
    t: 5.,
    object_id: s.id,
  };
  let i2 = Intersection {
    t: 7.,
    object_id: s.id,
  };
  let i3 = Intersection {
    t: -3.,
    object_id: s.id,
  };
  let i4 = Intersection {
    t: 2.,
    object_id: s.id,
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[&i1, &i2, &i3, &i4])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
  assert_eq!(i.t, i4.t);
}

#[test]
pub fn translate_ray() {
  let r = Ray::new(&Tuple::point(1., 2., 3.), &Tuple::vector(0., 1., 0.));
  let m = Matrix::translation(3., 4., 5.);
  let r2 = r.transform(&m);

  assert!(r2.origin.is_equal(&Tuple::point(4., 6., 8.)));
  assert!(r2.direction.is_equal(&Tuple::vector(0., 1., 0.)));
}

#[test]
pub fn scale_ray() {
  let r = Ray::new(&Tuple::point(1., 2., 3.), &Tuple::vector(0., 1., 0.));
  let m = Matrix::scale(2., 3., 4.);
  let r2 = r.transform(&m);
  assert!(r2.origin.is_equal(&Tuple::point(2., 6., 12.)));
  assert!(r2.direction.is_equal(&Tuple::vector(0., 3., 0.)));
}

#[test]
pub fn sphere_default_transform() {
  let s = Sphere::new();
  assert_eq!(s.transform, Matrix::new_identity_matrix(4))
}

#[test]
pub fn change_sphere_transform() {
  let mut s = Sphere::new();
  let t = Matrix::translation(2., 3., 4.);
  s.transform = t.clone();

  assert_eq!(s.transform, t);
}
// TODO: consider change ray arguments to two tuples (Rust type)
#[test]
pub fn intersect_scaled_sphere_with_ray() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let mut s = Sphere::new();
  s.transform = Matrix::scale(2., 2., 2.);
  let xs = Ray::intersects(&s, &r);
  assert_eq!(xs.len(), 2);
  assert!(util::equal(xs[0].t, 3.0));
  assert!(util::equal(xs[1].t, 7.0))
}
#[test]
pub fn intersect_translated_sphere_with_ray() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let mut s = Sphere::new();
  s.transform = Matrix::translation(5., 0., 0.);
  let xs = Ray::intersects(&s, &r);
  assert_eq!(xs.len(), 0);
}
