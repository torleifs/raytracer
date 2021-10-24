use crate::{canvas::Canvas, Ray};
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
  half_height: f64,
}

impl Camera {
  pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
    let (half_width, half_height, pixel_size) =
      Camera::compute_pixel_size(hsize, vsize, field_of_view);
    Camera {
      hsize,
      vsize,
      field_of_view,
      transform: Matrix::new_identity_matrix(4),
      half_width,
      half_height,
      pixel_size,
    }
  }
  pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
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
    // Calculate offset from edge of canvas to world pixel center
    let x_offset = (px as f64 + 0.5) * self.pixel_size;
    let y_offset = (py as f64 + 0.5) * self.pixel_size;

    // this flips the coordinates around the center of the canvas
    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    // The screen is defined to be at z = -1 relative to camera
    // Camera is at (0,0,0)
    // This transforms the canvas pixel and origin according to
    // camera position and view vector
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
        let color = world.color_at(&ray, 4);
        canvas.write_pixel(x, y, &color);
      }
    }
    canvas
  }
  // computes the size one pixel represents in the world:
  fn compute_pixel_size(hsize: usize, vsize: usize, field_of_view: f64) -> (f64, f64, f64) {
    let half_view = (field_of_view / 2.0).tan();
    let aspect = hsize as f64 / vsize as f64;
    let half_width = if aspect >= 1.0 {
      half_view
    } else {
      half_view * aspect
    };
    let half_height = if aspect >= 1.0 {
      half_view / aspect
    } else {
      half_view
    };
    (half_width, half_height, (half_width * 2.0) / hsize as f64)
  }
}
