use crate::raytracer::Camera;
use std::{rc::Rc};
use std::f64::consts;
mod canvas;
mod color;
mod math;
mod raytracer;
mod util;
use color::Color;
use raytracer::{Material, PointLight, Ray, World};
use raytracer::Sphere;
use math::Tuple;
use math::Matrix;
use crate::raytracer::geometry::Shape;
fn main() {
  let mut floor = Sphere::new();
  floor.transform = Matrix::scale(10., 0.01, 10.0);
  floor.material = Material::new();
  floor.material.color = Color::new(1., 0.9, 0.9);
  floor.material.specular = 0.;

  let mut left_wall = Sphere::new();
  left_wall.transform = Matrix::translation(0., 0., 5.) *
                        Matrix::rotation_y(-consts::FRAC_PI_4) * 
                        Matrix::rotation_x(consts::FRAC_PI_2) *
                        Matrix::scale(10., 0.01, 10.0);
  left_wall.material = floor.material.clone();

  let mut right_wall = Sphere::new();
  right_wall.transform = Matrix::translation(0., 0., 5.) *
                        Matrix::rotation_y(consts::FRAC_PI_4) * 
                        Matrix::rotation_x(consts::FRAC_PI_2) *
                        Matrix::scale(10., 0.01, 10.0);
  right_wall.material = left_wall.material.clone();

  let mut middle = Sphere::new();
  middle.transform = Matrix::translation(-0.5, 1., 0.5);
  middle.material = Material:: new();
  middle.material.color = Color::new(0.1, 1., 0.5);
  middle.material.diffuse = 0.7;
  middle.material.specular = 0.3;

  let mut right = Sphere::new();
  right.transform = Matrix::translation(1.5, 0.5, - 0.5) *
                    Matrix::scale(0.5, 0.5, 0.5);
  right.material = Material::new();
  right.material.color = Color::new(0.5, 1.0, 0.1);
  right.material.diffuse = 0.7;
  right.material.specular = 0.3;

  let mut left = Sphere::new();
  left.transform = Matrix::translation(-1.5, 0.33, -0.75) * 
                    Matrix::scale(0.33, 0.33, 0.33);
  left.material = Material::new();
  left.material.color = Color::new(1.0, 0.8, 0.1);
  left.material.diffuse = 0.7;
  left.material.specular = 0.3;

  let mut w = World::new();
  w.shapes = vec![Rc::new(floor), Rc::new(left_wall), Rc::new(right_wall), Rc::new(middle), Rc::new(left), Rc::new(right)];
  w.lights = vec![PointLight::new(&Tuple::point(-10., 10., -10.), &Color::new(1., 1., 1.))];

  let mut camera = Camera::new(1000, 500, consts::FRAC_PI_3);
  camera.transform = Camera::view_transform(&Tuple::point(0., 1.5, -5.), &Tuple::point(0., 1., 0.), &Tuple::vector(0., 1., 0.));
  let c = camera.render(&w);

  let ppm = c.to_ppm();
  canvas::Canvas::write_ppm_to_disk(
    &"/Users/torleifs/code/rust/raytracer/test.ppm".to_string(),
    &ppm,
  );
  println!("Finished rendering");
}
