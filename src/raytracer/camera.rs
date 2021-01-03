use crate::{Ray, canvas::Canvas};
use std::cmp::Ordering;

use crate::math::{Matrix, Tuple};

use super::World;

pub struct Camera {
  pub hsize: usize,
  pub vsize: usize,
  pub field_of_view: f64,
  pub transform: Matrix,
  pub pixel_size: f64,
  half_width: f64,
  half_height: f64
}

impl Camera {
  pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
    let (half_width, half_height, pixel_size) = Camera::compute_pixel_size(hsize, vsize, field_of_view);
    Camera {
      hsize,
      vsize,
      field_of_view,
      transform: Matrix::new_identity_matrix(4),
      half_width,
      half_height,
      pixel_size
    }
  }
  pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix{
    let forward = (to - from).normalize();
    let normalized_up = up.normalize();
    let left = Tuple::cross(&forward, &normalized_up);
    let true_up = Tuple::cross(&left, &forward);

    let orientation = Matrix::new(&[
      &[left.x, left.y, left.z, 0.],
      &[true_up.x, true_up.y, true_up.z, 0.],
      &[-forward.x, -forward.y, -forward.z, 0.],
      &[0.00000, 0.00000, 0.00000, 1.00000],
    ]);

    orientation * Matrix::translation(-from.x, -from.y, -from.z)
  }
  pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
    let x_offset = (px as f64 + 0.5) * self.pixel_size;
    let y_offset = (py as f64 + 0.5) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height -  y_offset;

    // Converting from object to world space:
    // The screen is defined to be at z = -1 relative to camera
    let pixel = &self.transform.invert().expect("help") * &Tuple::point(world_x, world_y, -1.);
    let origin = &self.transform.invert().expect("help") * &Tuple::point(0., 0., 0.);
    let direction = (pixel - &origin).normalize();

    Ray::new(&origin, &direction)
  }

  pub fn render(&self, world: &World) -> Canvas {
    
    let mut canvas = Canvas::new(self.hsize, self.vsize);
    for x in 0..self.hsize {
      for y in 0..self.vsize {
        let ray = self.ray_for_pixel(x, y);
        let color = world.color_at(&ray);
       
        canvas.write_pixel(x, y, &color);
      }
    }
    
    canvas
  }
  fn compute_pixel_size(hsize: usize, vsize: usize, field_of_view: f64) -> (f64, f64, f64) {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as f64 / vsize as f64;
    let half_width= match aspect.partial_cmp(&0.0).expect("") {
      Ordering::Less => half_view * aspect,
      _ => half_view
    }; 
    let half_height = match aspect.partial_cmp(&0.0).expect("") {
      Ordering::Less => half_view ,
      _ => half_view / aspect
    }; 
    
    (half_width, half_height, (half_width * 2.0) / hsize as f64)
  }

}