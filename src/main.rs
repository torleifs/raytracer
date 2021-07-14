use crate::raytracer::Camera;
use core::cell::RefCell;
use std::f64::consts;
use std::rc::Rc;
mod canvas;
mod color;
mod math;
mod raytracer;
mod util;
use crate::raytracer::patterns::StripePattern;
use color::Color;
use math::Matrix;
use math::Tuple;
use raytracer::Plane;
use raytracer::Sphere;
use raytracer::{Material, PointLight, Ray, World};

fn main() {
  let mut floor = Plane::new();

  let mut mat = Material::new();
  mat.color = Color::new(1., 0.9, 0.9);
  mat.specular = 0.;
  mat.pattern = Some(Rc::new(StripePattern::new(
    Color::new(0.0, 1.0, 0.0),
    Color::new(0.0, 0.0, 1.0),
  )));
  floor.material = Rc::new(mat);

  let mut middle = Sphere::new();
  middle.transform = RefCell::new(Matrix::translation(-0.5, 1., 0.5));
  mat = Material::new();
  mat.pattern = Some(Rc::new(StripePattern::new_with_transform(
    Color::new(1.0, 1.0, 0.0),
    Color::new(0.0, 1.0, 1.0),
    Matrix::rotation_y(-0.5) * Matrix::scale(0.2, 1., 1.),
  )));

  mat.color = Color::new(0.1, 1., 0.5);
  mat.diffuse = 0.7;
  mat.specular = 0.3;
  middle.material = Rc::new(mat);

  let mut right = Sphere::new();
  right.transform =
    RefCell::new(Matrix::translation(1.5, 0.5, -0.5) * Matrix::scale(0.5, 0.5, 0.5));
  mat = Material::new();
  mat.color = Color::new(0.5, 1.0, 0.1);
  mat.diffuse = 0.7;
  mat.specular = 0.3;
  right.material = Rc::new(mat);

  let mut left = Sphere::new();
  left.transform =
    RefCell::new(Matrix::translation(-1.5, 0.33, -0.75) * Matrix::scale(0.33, 0.33, 0.33));
  mat = Material::new();
  mat.color = Color::new(1.0, 0.8, 0.1);
  mat.diffuse = 0.7;
  mat.specular = 0.3;
  left.material = Rc::new(mat);

  let mut w = World::new();
  w.shapes = vec![
    Rc::new(floor),
    Rc::new(middle),
    Rc::new(left),
    Rc::new(right),
  ];
  w.lights = vec![PointLight::new(
    &Tuple::point(-10., 10., -10.),
    &Color::new(1., 1., 1.),
  )];

  let mut camera = Camera::new(1000, 500, consts::FRAC_PI_3);
  camera.transform = Camera::view_transform(
    &Tuple::point(0., 1.5, -5.),
    &Tuple::point(0., 1., 0.),
    &Tuple::vector(0., 1., 0.),
  );
  let c = camera.render(&w);

  let ppm = c.to_ppm();
  canvas::Canvas::write_ppm_to_disk(&"/Users/torleifs/code/raytracer/test.ppm".to_string(), &ppm);
  println!("Finished rendering");
}
