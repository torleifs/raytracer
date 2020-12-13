mod canvas;
mod color;
mod math;
mod raytracer;
mod util;
use color::Color;
use raytracer::{Material, PointLight, Ray};
use raytracer::Sphere;


fn main() {
  let mut c = canvas::Canvas::new(100, 100);
  let mut color = color::Color::new(1.0, 0.0, 0.0);

  let mut sphere = Sphere::new();
  sphere.material = Material::new();
  sphere.material.color = Color::new(1., 0.2, 1.);
  //sphere.transform = math::Matrix::shear(1., 0., 0., 0., 0., 0.) * math::Matrix::scale(0.5, 1., 1.);
  let light = PointLight::new(&math::Tuple::point(-10., 10., -10.), &Color::new(1., 1., 1.));

  let ray_origin = math::Tuple::point(0., 0., -5.);
  let wall_size = 7.0;
  let pixel_size: f64 = wall_size / 100.;

  for x in -49..50 {
    let world_x = pixel_size * (x as f64);
    for y in -49..50 {
      let world_y = pixel_size * (y as f64);
      let position = math::Tuple::point(world_x, world_y, 10.);
      let ray_unit_vector = (position - &ray_origin).normalize();
      let r = Ray::new(&math::Tuple::point(0., 0., -5.), &ray_unit_vector);
      let xs = Ray::intersects(&sphere, &r);
      if xs.len() > 0 {
        let point = Ray::position(&r, xs[0].t);
        let normal = sphere.normal_at(&point);
        let eye_vector = -r.direction;
        let color = Material::lighting(&sphere.material, &light, &point, &eye_vector, &normal);
        c.write_pixel(x, y, color.clone());
      }
    }
  }
  let ppm = c.to_ppm();
  canvas::Canvas::write_ppm_to_disk(
    &"/Users/torleifs/code/rust/raytracer/test.ppm".to_string(),
    &ppm,
  );
  println!("Finished rendering");
}
