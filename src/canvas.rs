use std::fs::File;
use std::io::Write;

use crate::math;

pub struct Canvas {
  pub width: i32,
  pub height: i32,
  pixels: Vec<math::Color>,
}

pub trait PixelValueTrait {}
impl PixelValueTrait for i32 {}
impl PixelValueTrait for f64 {}

impl Canvas {
  pub fn pixel_at(&self, x: i32, y: i32) -> &math::Color {
    let index_of_pixel = y * self.width + x;
    &(self.pixels[index_of_pixel as usize])
  }

  pub fn write_pixel(&mut self, x: i32, y: i32, color: math::Color) {
    let mirrored_y = self.height - y;
    let index_of_pixel = mirrored_y * self.width + x;
    if x < self.width && y < self.height {
      self.pixels[index_of_pixel as usize] = color;
    }
  }
  pub fn write_pixel_f(&mut self, x: f64, y: f64, color: math::Color) {
    if x < 0.0 || y < 0.0 {
      return;
    }
    let x_i = x.round() as i32;
    let y_i = y.round() as i32;
    self.write_pixel(x_i, y_i, color);
  }
  pub fn to_ppm(&self) -> String {
    let header = format!("P3\n{} {}\n255\n", self.width, self.height);
    let mut pixel_string = String::from(header);
    for (i, color) in self.pixels.iter().enumerate() {
      let r = &Canvas::scale_clamp_to_string(color.r());
      let g = &Canvas::scale_clamp_to_string(color.g());
      let b = &Canvas::scale_clamp_to_string(color.b());
      pixel_string.push_str(&format!("{} {} {}", r, g, b));
      if (i + 1) % self.width as usize == 0 {
        pixel_string.push('\n');
      } else {
        pixel_string.push(' ');
      }
    }
    pixel_string
  }
  pub fn new(width: i32, height: i32) -> Canvas {
    Canvas {
      width,
      height,
      pixels: vec![math::Color::new(0.0, 0.0, 0.0); (height * width) as usize],
    }
  }
  pub fn new_with_fill(width: i32, height: i32, color: &math::Color) -> Canvas {
    let color_clone = color.clone();
    Canvas {
      width,
      height,
      pixels: vec![color_clone; (height * width) as usize],
    }
  }

  pub fn write_ppm_to_disk(file_name: &String, ppm: &String) {
    let mut out_file = File::create(file_name).expect("could not create file");
    out_file
      .write_all(ppm.as_bytes())
      .expect("Failed to write to file");
  }
  fn scale_clamp_to_string(n: f64) -> String {
    let scaled_number = (n * 255.0).round() as i32;
    if scaled_number > 255 {
      255.to_string()
    } else if scaled_number < 0 {
      0.to_string()
    } else {
      scaled_number.to_string()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Canvas;
  use crate::math;
  #[test]
  fn create_canvas() {
    let c = Canvas::new(10, 20);
    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);
    for x in 0..10 {
      for y in 0..20 {
        let color = c.pixel_at(x, y);
        assert!(color.is_equal(&math::Color::new(0.0, 0.0, 0.0)))
      }
    }
  }
  #[test]
  fn write_pixel_to_canvas() {
    let mut c: Canvas = Canvas::new(10, 20);
    let red = math::Color::new(1.0, 0.0, 0.0);
    let red_clone = red.clone();
    c.write_pixel(2, 3, red);
    let res = c.pixel_at(2, 3);
    assert!(res.is_equal(&red_clone));
  }

  #[test]
  fn construct_ppm_header() {
    let c = Canvas::new(5, 3);
    let ppm = c.to_ppm();
    let lines = ppm.lines().collect::<Vec<_>>();
    assert_eq!(lines[0], "P3");
    assert_eq!(lines[1], "5 3");
    assert_eq!(lines[2], "255");
  }
  #[test]
  fn construct_ppm_pixel_data() {
    let mut c = Canvas::new(5, 3);
    let c1 = math::Color::new(1.5, 0.0, 0.0);
    let c2 = math::Color::new(0.0, 0.5, 0.0);
    let c3 = math::Color::new(-0.5, 0.0, 1.0);

    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);
    let ppm = c.to_ppm();
    let lines = ppm.lines().collect::<Vec<_>>();
    assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
    assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
  }

  #[test]
  fn split_long_lines() {
    let fill_color = math::Color::new(1.0, 0.8, 0.6);
    let c = Canvas::new_with_fill(10, 2, &fill_color);
    let ppm = c.to_ppm();
    let lines = ppm.lines().collect::<Vec<_>>();
    assert_eq!(lines[3], "255 204 153 ");
  }
}
