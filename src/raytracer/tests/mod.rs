use std::{f64::consts, rc::Rc};
use std::cell::RefCell;
use super::Intersection;
use super::Ray;
use super::Sphere;

use super::World;
use super::Camera;
use super::geometry::normal_at;
use super::lights::PointLight;
use super::materials::Material;
use crate::math::Matrix;
use crate::math::Tuple;
use crate::util;
use crate::color::Color;
use crate::raytracer::geometry::Shape;
use crate::raytracer::geometry::TestShape;
use crate::raytracer::geometry::Plane;

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
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);
  assert_eq!(xs.len(), 2);
  assert!(util::equal(xs[0].t, 4.0));
  assert!(util::equal(xs[1].t, 6.0))
}
#[test]
fn ray_intersects_sphere_at_tangent() {
  let r = Ray::new(&Tuple::point(0., 1., -5.), &Tuple::vector(0., 0., 1.));
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);
  assert_eq!(xs.len(), 2);
  // return two points even if tangential!
  assert!(util::equal(xs[0].t, 5.0));
  assert!(util::equal(xs[1].t, 5.0))
}

#[test]
fn ray_misses_sphere() {
  let r = Ray::new(&Tuple::point(0., 2., -5.), &Tuple::vector(0., 0., 1.));
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);
  assert_eq!(xs.len(), 0);
}

#[test]
fn ray_originates_within_sphere() {
  let r = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);
  assert!(util::equal(xs[0].t, -1.0));
  assert!(util::equal(xs[1].t, 1.0))
}

#[test]
fn sphere_is_behind_ray() {
  let r = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);
  assert!(util::equal(xs[0].t, -6.0));
  assert!(util::equal(xs[1].t, -4.0))
}

#[test]
fn intersection_has_t_and_object() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i = Intersection::new(&s, 3.5);
  assert_eq!(s.get_id(), i.shape.get_id());
}

#[test]
fn aggregate_intersections() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i1 = Intersection::new(&s, 1.);
  let i2 = Intersection::new(&s, 2.);
  let xs = Intersection::intersections(&[i1, i2]);
  assert_eq!(xs.len(), 2);
  assert_eq!(xs[0].shape.get_id(), s.get_id());
  assert_eq!(xs[1].shape.get_id(), s.get_id());
}

#[test]
fn intersect_sets_intersected_object() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let xs = s.intersect(&r);

  assert_eq!(xs[0].shape.get_id(), s.get_id());
  assert_eq!(xs[1].shape.get_id(), s.get_id());
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i1 = Intersection {
    t: 1.,
    shape: Rc::clone(&s),
  };
  let i2 = Intersection {
    t: 2.,
    shape: Rc::clone(&s),
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[i1.clone(), i2])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
  assert_eq!(i.t, i1.t);
}

#[test]
fn the_hit_when_domr_intersections_have_negative_t() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i1 = Intersection {
    t: -1.,
    shape:  Rc::clone(&s),
  };
  let i2 = Intersection {
    t: 2.,
    shape:  Rc::clone(&s),
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[i1, i2.clone()])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
  assert_eq!(i.t, i2.t);
}

#[test]
#[should_panic]
fn the_hit_when_all_intersections_have_negative_t() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i1 = Intersection {
    t: -2.,
    shape:  Rc::clone(&s),
  };
  let i2 = Intersection {
    t: -1.,
    shape:  Rc::clone(&s),
  };
  match Intersection::hit(&mut Intersection::intersections(&[i1, i2])) {
    Some(an_i) => an_i,
    None => panic!(),
  };
}

#[test]
fn the_hit_is_the_lowest_nonnegative_intersection() {
  let s:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i1 = Intersection {
    t: 5.,
    shape: Rc::clone(&s),
  };
  let i2 = Intersection {
    t: 7.,
    shape: Rc::clone(&s),
  };
  let i3 = Intersection {
    t: -3.,
    shape: Rc::clone(&s),
  };
  let i4 = Intersection {
    t: 2.,
    shape: Rc::clone(&s),
  };
  let i = match Intersection::hit(&mut Intersection::intersections(&[i1, i2, i3, i4.clone()])) {
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
  assert_eq!(s.transform.into_inner(), Matrix::new_identity_matrix(4))
}

#[test]
pub fn change_sphere_transform() {
  let mut s = Sphere::new();
  let t = Matrix::translation(2., 3., 4.);
  s.transform = RefCell::new(t.clone());

  assert_eq!(s.transform.into_inner(), t);
}
// TODO: consider change ray arguments to two tuples (Rust type)
#[test]
pub fn intersect_scaled_sphere_with_ray() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let mut s = Sphere::new();
  s.transform = RefCell::new(Matrix::scale(2., 2., 2.));
  let s:Rc<dyn Shape> = Rc::new(s);
  let xs = s.intersect(&r);
  assert_eq!(xs.len(), 2);
  assert!(util::equal(xs[0].t, 3.0));
  assert!(util::equal(xs[1].t, 7.0))
}
#[test]
pub fn intersect_translated_sphere_with_ray() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let mut s = Sphere::new();
  s.transform = RefCell::new(Matrix::translation(5., 0., 0.));
  let s:Rc<dyn Shape> = Rc::new(s);
  let xs = s.intersect(&r);
  assert_eq!(xs.len(), 0);
}

#[test]
pub fn normal_of_sphere_on_x_axis_point() {
  let s = Sphere::new();
  let n = s.local_normal_at(&Tuple::point(1., 0., 0.));
  assert_eq!(n, Tuple::vector(1., 0., 0.));
}

#[test]
pub fn normal_of_sphere_on_y_axis_point() {
  let s = Sphere::new();
  let n = s.local_normal_at(&Tuple::point(0., 1., 0.));
  assert_eq!(n, Tuple::vector(0., 1., 0.));
}

#[test]
pub fn normal_of_sphere_on_z_axis_point() {
  let s = Sphere::new();
  let n = s.local_normal_at(&Tuple::point(0., 0., 1.));
  assert_eq!(n, Tuple::vector(0., 0., 1.));
}

#[test]
pub fn normal_of_sphere_on_non_axial_point() {
  let s = Sphere::new();
  let n = s.local_normal_at(&Tuple::point(
    (3. as f64).sqrt() / 3.,
    (3. as f64).sqrt() / 3.,
    (3. as f64).sqrt() / 3.,
  ));
  assert_eq!(
    n,
    Tuple::vector(
      (3. as f64).sqrt() / 3.,
      (3. as f64).sqrt() / 3.,
      (3. as f64).sqrt() / 3.
    )
  );
}

#[test]
pub fn normal_is_normalized() {
  let s = Sphere::new();
  let n = s.local_normal_at(&Tuple::point(
    (3. as f64).sqrt() / 3.,
    (3. as f64).sqrt() / 3.,
    (3. as f64).sqrt() / 3.,
  ));
  assert_eq!(n, n.normalize());
}



#[test]
pub fn compute_normal_on_translated_sphere() {
  let mut s = Sphere::new();
  s.transform = RefCell::new(Matrix::translation(0., 1., 0.));
  let n = normal_at(Rc::new(s), &Tuple::point(0., 1.70711, -0.70711));
  assert_eq!(n, Tuple::vector(0., 0.70711, -0.70711));
}

#[test]
pub fn compute_normal_on_transformed_sphere() {
  let mut s = Sphere::new();
  s.transform = RefCell::new(Matrix::scale(1., 0.5, 1.) * Matrix::rotation_z(consts::PI / 5.));
  let n = normal_at(Rc::new(s), &Tuple::point(0., (2. as f64).sqrt() / 2., -(2. as f64).sqrt() / 2.));
  assert_eq!(n, Tuple::vector(0., 0.97014, -0.24254));
}

/////////////////////
#[test]
pub fn point_light_has_position_and_intensity() {
  let intensity = Color::new(1., 1., 1.);
  let position = Tuple::point(0., 0., 0.);
  
  let light = PointLight::new(&position, &intensity);
  assert_eq!(light.position, position);
  assert_eq!(light.intensity, intensity);
}

#[test]
pub fn default_material() {
  let m = Material::new();

  assert_eq!(m.color, Color::new(1., 1., 1.));
  assert_eq!(m.ambient, 0.1);
  assert_eq!(m.diffuse, 0.9);
  assert_eq!(m.specular, 0.9);
  assert_eq!(m.shininess, 200.0);
}

#[test]
pub fn sphere_has_default_material() {
  let s = Sphere::new();
  assert_eq!(s.material.into_inner(), Material::new());
}


#[test]
pub fn sphere_can_be_assigned_material() {
  let mut s = Sphere::new();
  let mut m = Material::new();
  m.ambient = 1.;
  s.material = RefCell::new(m.clone());
  assert_eq!(s.material.into_inner(), m);
}

/// Light source
#[test]
pub fn lighting_with_eye_between_light_and_surface() {
  let m = Material::new();
  let position = Tuple::point(0., 0., 0.);
  let eye_vector = Tuple::vector(0., 0.,-1.);
  let normal_vector = Tuple::vector(0., 0.,-1.);
  let light = PointLight::new(&Tuple::point(0., 0., -10.), &Color::new(1., 1., 1.));
  
  let result = Material::lighting(&m, &light, &position, &eye_vector, &normal_vector, false);
  assert_eq!(&Color::new(1.9, 1.9, 1.9), &result);
}
#[test]
pub fn lighting_with_eye_between_light_and_surface_eye_offset_45() {
  let m = Material::new();
  let position = Tuple::point(0., 0., 0.);
  let eye_vector = Tuple::vector(0., (2.0 as f64).sqrt() / 2.,-(2.0 as f64).sqrt() / 2.);
  let normal_vector = Tuple::vector(0., 0.,-1.);
  let light = PointLight::new(&Tuple::point(0., 0., -10.), &Color::new(1., 1., 1.));
  
  let result = Material::lighting(&m, &light, &position, &eye_vector, &normal_vector, false);
  assert_eq!(&Color::new(1.0, 1.0, 1.0), &result);
}
#[test]
pub fn lighting_with_eye_opposite_surface_light_offset_45() {
  let m = Material::new();
  let position = Tuple::point(0., 0., 0.);
  let eye_vector = Tuple::vector(0., 0.,-1.);
  let normal_vector = Tuple::vector(0., 0.,-1.);
  let light = PointLight::new(&Tuple::point(0., 10., -10.), &Color::new(1., 1., 1.));
  
  let result = Material::lighting(&m, &light, &position, &eye_vector, &normal_vector, false);
  assert_eq!(&Color::new(0.7364, 0.7364, 0.7364), &result);
}
#[test]
pub fn lighting_with_eye_in_path_of_reflection_vector() {
  let m = Material::new();
  let position = Tuple::point(0., 0., 0.);
  let eye_vector = Tuple::vector(0., -(2.0 as f64).sqrt() / 2.,-(2.0 as f64).sqrt() / 2.);
  let normal_vector = Tuple::vector(0., 0.,-1.);
  let light = PointLight::new(&Tuple::point(0., 10., -10.), &Color::new(1., 1., 1.));
  
  let result = Material::lighting(&m, &light, &position, &eye_vector, &normal_vector, false);
  assert_eq!(&Color::new(1.6364, 1.6364, 1.6364), &result);
}

#[test]
pub fn lighting_with_light_behind_surface() {
  let m = Material::new();
  let position = Tuple::point(0., 0., 0.);
  let eye_vector = Tuple::vector(0., 0.,-1.);
  let normal_vector = Tuple::vector(0., 0.,-1.);
  let light = PointLight::new(&Tuple::point(0., 0., 10.), &Color::new(1., 1., 1.));
  
  let result = Material::lighting(&m, &light, &position, &eye_vector, &normal_vector, false);
  assert_eq!(&Color::new(0.1, 0.1, 0.1), &result);
}

#[test]
pub fn create_world() {
  let w = World::new();

  assert_eq!(w.shapes.len(), 0);
  assert_eq!(w.lights.len(), 0);
}

fn are_shapes_equivalent(s1: &Rc<dyn Shape>, s2: &Rc<dyn Shape>) -> bool {

  return s1.get_material() == s2.get_material() &&
    s1.get_transform() == s2.get_transform()
}
#[test]
pub fn default_world() {
  let light = PointLight::new(&Tuple::point(-10., 10., -10.), &Color::new(1., 1., 1.));
  let mut s1 = Sphere::new();
  let mut mat = Material::new();
  mat.color = Color::new(0.8, 1.0, 0.8);
  mat.diffuse = 0.7;
  mat.specular = 0.2;
  s1.material = RefCell::new(mat);
  let s1:Rc<dyn Shape> = Rc::new(s1);
  let mut s2 = Sphere::new();
  s2.transform = RefCell::new(Matrix::scale(0.5, 0.5, 0.5));
  let s2:Rc<dyn Shape> = Rc::new(s2);
  let w = World::default();
  assert_eq!(w.lights, vec![light]);
  assert!(are_shapes_equivalent(&w.shapes[0], &s1));
  assert!(are_shapes_equivalent(&w.shapes[1], &s2));
}

#[test]
pub fn intersect_world_ray() {
  let w = World::default();
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let xs = w.intersect_world(&ray);
  assert_eq!(xs.len(), 4);
  assert!(util::equal(xs[0].t, 4.0));
  assert!(util::equal(xs[1].t, 4.5));
  assert!(util::equal(xs[2].t, 5.5));
  assert!(util::equal(xs[3].t, 6.0));
}
#[test]
pub fn precompute_intersect() {
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let shape:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i = Intersection::new(&shape, 4.0);
  let comps = Ray::precompute(&i, &ray);
  assert!(util::equal(comps.t, i.t));
  assert_eq!(comps.point, Tuple::point(0., 0., -1.));
  assert_eq!(comps.eye_vector, Tuple::vector(0., 0., -1.));
  assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
}

#[test]
pub fn precompute_outside() {
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let shape:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i = Intersection::new(&shape, 4.0);
  let comps = Ray::precompute(&i, &ray);
  assert!(!comps.inside);
}

#[test]
pub fn precompute_inside() {
  let ray = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
  let shape:Rc<dyn Shape> = Rc::new(Sphere::new());
  let i = Intersection::new(&shape, 1.0);

  let comps = Ray::precompute(&i, &ray);

  assert!(comps.inside);
  assert_eq!(comps.normal_vector, Tuple::vector(0., 0., -1.));
}

#[test]
pub fn shade_intersection() {
  let w = World::default();
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let shape = &w.shapes[0];
  let i = Intersection::new(shape, 4.0);
  let comps = Ray::precompute(&i, &ray);

  let c = World::shade_hit(&w, &comps);
  assert_eq!(c, Color::new(0.38066, 0.47583, 0.3806)); // TODO: the b component is not the same as in book. page 95
}


#[test]
pub fn shade_intersection_from_inside() {
  let mut w = World::default();
  w.lights = vec![PointLight::new(&Tuple::point(0., 0.25, 0.), &Color::new(1., 1., 1.))];
  let ray = Ray::new(&Tuple::point(0., 0., 0.), &Tuple::vector(0., 0., 1.));
  let shape = &w.shapes[1];
  let i = Intersection::new(shape, 0.5);
  let comps = Ray::precompute(&i, &ray);

  let c = World::shade_hit(&w, &comps);
  assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
}

#[test]
pub fn shade_intersection_in_shadow() {
  let mut w = World::new();
  w.lights = vec![PointLight::new(&Tuple::point(0., 0., -10.), &Color::new(1., 1., 1.))];
  let s1 = Sphere::new();
  let mut s2 = Sphere::new();
  s2.transform = RefCell::new(Matrix::translation(0., 0., 10.));
  w.shapes = vec![Rc::new(s1), Rc::new(s2)];

  let ray = Ray::new(&Tuple::point(0., 0., 5.), &Tuple::vector(0., 0., 1.));
  //let shape = &w.shapes[1];
  let i = Intersection::new(&w.shapes[1], 4.0);
  let comps = Ray::precompute(&i, &ray);

  let c = World::shade_hit(&w, &comps);
  assert_eq!(c, Color::new(0.1, 0.1, 0.1));
}

#[test]
pub fn color_when_ray_misses() {
  let w = World::default();
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 1., 0.));

  let c = w.color_at(&ray);
  assert_eq!(c, Color::new(0., 0., 0.));
}

#[test]
pub fn color_when_ray_hits() {
  let w = World::default();
  let ray = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));

  let c = w.color_at(&ray);
  assert_eq!(c, Color::new(0.38066, 0.47583, 0.3806));
}

#[test]
pub fn color_with_intersection_behind_ray() {
  let w = World::default_world_with_ambient_materials(1.0);
  let inner = &w.shapes[1];

  let ray = Ray::new(&Tuple::point(0., 0., 0.75), 
                &Tuple::vector(0., 0., -1.));

  let c = w.color_at(&ray);
  let m_inner= inner.get_material();
  assert_eq!(c, m_inner.color);
}

// Camera

#[test]
pub fn transform_matrix_default_orientation() {
  let from = Tuple::point(0., 0., 0.);
  let to = Tuple::point(0., 0., -1.);
  let up = Tuple::vector(0., 1., 0.,);

  let t = Camera::view_transform(&from, &to, &up);

  assert_eq!(&Matrix::new_identity_matrix(4), &t);
}


#[test]
pub fn transform_matrix_positive_z_direction() {
  let from = Tuple::point(0., 0., 0.);
  let to = Tuple::point(0., 0., 1.);
  let up = Tuple::vector(0., 1., 0.,);

  let t = Camera::view_transform(&from, &to, &up);

  assert_eq!(&Matrix::scale(-1., 1., -1.), &t);
}


#[test]
pub fn view_transform_moves_he_world() {
  let from = Tuple::point(0., 0., 8.);
  let to = Tuple::point(0., 0., 0.);
  let up = Tuple::vector(0., 1., 0.,);

  let t = Camera::view_transform(&from, &to, &up);

  assert_eq!(&Matrix::translation(0., 0., -8.), &t);
}


#[test]
pub fn transform_matrix_arbitrary() {
  let from = Tuple::point(1., 3., 2.);
  let to = Tuple::point(4., -2., 8.);
  let up = Tuple::vector(1., 1., 0.,);

  let t = Camera::view_transform(&from, &to, &up);
  let m: Matrix = Matrix::new(&[
    &[-0.50709, 0.50709, 0.67612, -2.36643],
    &[0.76772, 0.60609, 0.12122, -2.82843],
    &[-0.35857, 0.59761, -0.71714, 0.00000],
    &[0.00000, 0.00000, 0.00000, 1.00000],
]);
  assert_eq!(&m, &t);
}

#[test]
pub fn construct_camera() {
  let hsize = 160;
  let vsize = 120;
  let field_of_view = consts::PI / 2.;

  let c = Camera::new(hsize, vsize, field_of_view);

  assert_eq!(c.hsize, hsize);
  assert_eq!(c.vsize, vsize);
  assert!(util::equal(c.field_of_view, consts::PI / 2.));
  assert_eq!(&c.transform, &Matrix::new_identity_matrix(4));
}

#[test]
pub fn pixel_size_horizontal_canvas() {
  let c = Camera::new(200, 125, consts::PI / 2.);

  assert!(util::equal(c.pixel_size, 0.01));
}

#[test]
pub fn pixel_size_vertical_canvas() {
  let c = Camera::new(125, 200, consts::PI / 2.);

  assert!(util::equal(c.pixel_size, 0.01));
}

#[test]
pub fn construct_ray_through_center_of_canvas() {
  let c = Camera::new(201, 101, consts::PI /2.);

  let r = c.ray_for_pixel(100, 50);
  assert_eq!(&r.origin, &Tuple::point(0., 0., 0.));
  assert_eq!(&r.direction, &Tuple::vector(0., 0., -1.));
}

#[test]
pub fn construct_ray_through_corner_of_canvas() {
  let c = Camera::new(201, 101, consts::PI /2.);

  let r = c.ray_for_pixel(0, 0);
  assert_eq!(&r.origin, &Tuple::point(0., 0., 0.));
  assert_eq!(&r.direction, &Tuple::vector(0.66519, 0.33259, -0.66851));
}

#[test]
pub fn construct_ray_when_camera_is_transformed() {
  let mut c = Camera::new(201, 101, consts::PI /2.);
  c.transform = Matrix::rotation_y(consts::PI/4.) * Matrix::translation(0., -2., 5.);
  let r = c.ray_for_pixel(100, 50);
  assert_eq!(&r.origin, &Tuple::point(0., 2., -5.));
  assert_eq!(&r.direction, &Tuple::vector((2.0 as f64).sqrt() / 2., 0.0, -(2.0 as f64).sqrt()  / 2.));
}

#[test]
pub fn render_world_with_camera() {
  let w = World::default();
  let mut c = Camera::new(11, 11, consts::FRAC_PI_2);
  let from = Tuple::point(0., 0., -5.);
  let to = Tuple::point(0., 0., 0.);
  let up = Tuple::vector(0., 1., 0.);
  c.transform = Camera::view_transform(&from, &to, &up);

  let canvas = c.render(&w);
  
  let cc = canvas.pixel_at(5,5);
  assert_eq!(cc, &Color::new(0.38066, 0.47583, 0.2855))
}

#[test]
pub fn light_surface_in_shadow() {
  let eye_vec = Tuple::vector(0., 0., -1.);
  let normal_vec = Tuple::vector(0., 0., -1.);

  let light = PointLight::new(&Tuple::point(0., 0., -10.), &Color::new(1., 1., 1.));
  let in_shadow = true;
  let m = Material::new();
  let pos = Tuple::point(0., 0., 0.);
  let result = Material::lighting(&m, &light, &pos, &eye_vec, &normal_vec, in_shadow);
  assert_eq!(&result, &Color::new(0.1, 0.1, 0.1));
}

#[test]
pub fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
  let w = World::default();
  let p = Tuple::point(0., 10.0, 0.);

  assert!(!w.is_shadowed(&p));
}

#[test]
pub fn shadow_when_object_is_between_point_and_light() {
  let w = World::default();
  let p = Tuple::point(10., -10.0, 10.);

  assert!(w.is_shadowed(&p));
}

#[test]
pub fn no_shadow_when_object_behind_light() {
  let w = World::default();
  let p = Tuple::point(-20., 20.0, -20.);

  assert!(!w.is_shadowed(&p));
}

#[test]
pub fn no_shadow_when_object_behind_point() {
  let w = World::default();
  let p = Tuple::point(-2., 2.0, -2.);

  assert!(!w.is_shadowed(&p));
}

#[test]
pub fn hit_should_offset_point() {
  let r = Ray::new(&Tuple::point(0., 0., -5.), &Tuple::vector(0., 0., 1.));
  let mut s = Sphere::new();
  s.transform = RefCell::new(Matrix::translation(0., 0., 1.));
  let s_s :Rc<dyn Shape> = Rc::new(s);
  
  let i = Intersection::new(&s_s, 5.0);
  let comps = Ray::precompute(&i, &r);
  assert!(comps.over_point.z < - util::EPSILON / 2.);
  assert!(comps.point.z > comps.over_point.z);
}

#[test]
pub fn shape_default_transformation() {
  let s = TestShape::new();
  assert_eq!(s.get_transform(), Matrix::new_identity_matrix(4));
}

#[test]
pub fn shape_assign_transformation() {
  let s = TestShape::new();
  s.set_transform(Matrix::translation(2., 3., 4.));
  assert_eq!(s.get_transform(), Matrix::translation(2., 3., 4.));
}

#[test]
pub fn shape_default_material() {
  let s = TestShape::new();
  let m = s.get_material();
  assert_eq!(m, Material::new());
}

#[test]
pub fn shape_assign_material() {
  let s = TestShape::new();
  let mut m = Material::new();
  m.ambient = 1.0;
  s.set_material(m.clone());
  assert_eq!(s.get_material(), m);
}

#[test]
pub fn intersect_scaled_shape_with_ray() {
  let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
  let s = TestShape::new();
  s.set_transform(Matrix::scale(2.0, 2.0, 2.0));
  let _ = s.intersect(&r);

  assert_eq!(s.get_saved_ray().origin, Tuple::point(0., 0., -2.5));
  assert_eq!(s.get_saved_ray().direction, Tuple::vector(0., 0., 0.5));
}

#[test]
pub fn intersect_transformed_shape_with_ray() {
  let r = Ray::new(&Tuple::point(0.0, 0.0, -5.0), &Tuple::vector(0.0, 0.0, 1.0));
  let s = TestShape::new();
  s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
  let _ = s.intersect(&r);

  assert_eq!(s.get_saved_ray().origin, Tuple::point(-5.0, 0., -5.0));
  assert_eq!(s.get_saved_ray().direction, Tuple::vector(0., 0., 1.0));
}

#[test]
pub fn compute_normal_translated_shape() {
  let s = TestShape::new();
  s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
  let n = normal_at(Rc::new(s), &Tuple::point(0.0, 1.70711, -0.70711));

  assert_eq!(n, Tuple::vector(0.0, 0.70711, -0.70711));
}

#[test]
pub fn compute_normal_transformed_shape() {
  let s = TestShape::new();
  let m = Matrix::scale(1.0, 0.5, 1.0) * Matrix::rotation_z(consts::PI / 5.0);
  s.set_transform(m);
  let n = normal_at(Rc::new(s), &Tuple::point(0.0, (2.0 as f64).sqrt() / 2.0, -(2.0 as f64).sqrt() / 2.0));

  assert_eq!(n, Tuple::vector(0.0, 0.97014, -0.24254));
}

#[test]
pub fn normal_of_plane_is_constant() {
  let p = Plane::new();
  let n1 = p.local_normal_at(&Tuple::point(0.0, 0.0, 0.0));
  let n2 = p.local_normal_at(&Tuple::point(10.0, 0.0, -10.0));
  let n3 = p.local_normal_at(&Tuple::point(-5.0, 0.0, 150.0));
  
  assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
  assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
  assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
}

#[test]
pub fn intersect_with_ray_parallel_to_plane() {
  let p = Plane::new();
  let r = Ray::new(&Tuple::point(0.0, 10.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
  let xs = p.intersect(&r);

  assert_eq!(0, xs.len());
}

#[test]
pub fn intersect_with_coplanar_ray() {
  let p = Plane::new();
  let r = Ray::new(&Tuple::point(0.0, 0.0, 0.0), &Tuple::vector(0.0, 0.0, 1.0));
  let xs = p.intersect(&r);
  
  assert_eq!(0, xs.len());
}

#[test]
pub fn ray_intersect_from_above() {
  let p = Plane::new();
  let r = Ray::new(&Tuple::point(0.0, 1.0, 0.0), &Tuple::vector(0.0, -1.0, 0.0));

  let xs = p.intersect(&r);

  assert_eq!(xs.len(), 1);
  assert!(util::equal(xs[0].t, 1.0));
  assert_eq!(xs[0].shape.get_id(), p.get_id());
}

#[test]
pub fn ray_intersect_from_below() {
  let p = Plane::new();
  let r = Ray::new(&Tuple::point(0.0, -1.0, 0.0), &Tuple::vector(0.0, 1.0, 0.0));

  let xs = p.intersect(&r);

  assert_eq!(xs.len(), 1);
  assert!(util::equal(xs[0].t, 1.0));
  assert_eq!(xs[0].shape.get_id(), p.get_id());
}