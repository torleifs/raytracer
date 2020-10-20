mod canvas;
mod color;
mod math;
mod util;

struct Environment {
  pub gravity: math::tuple::Tuple,
  pub wind: math::Tuple,
}
#[derive(Clone)]
struct Projectile {
  position: math::Tuple,
  velocity: math::Tuple,
}
impl Projectile {
  pub fn projectile(position: math::Tuple, velocity: math::Tuple) -> Projectile {
    Projectile { position, velocity }
  }
}

fn tick(gr: &Environment, projectile: Projectile) -> Projectile {
  let mut velocity = gr.gravity.clone();
  velocity = velocity + &projectile.velocity;
  velocity = velocity + &gr.wind;
  let new_pos = projectile.position + &projectile.velocity;
  return Projectile::projectile(new_pos, velocity);
}
fn main() {
  let projectile = Projectile::projectile(
    math::Tuple::point(0.0, 1.0, 0.0),
    math::Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25,
  );
  let env = Environment {
    gravity: math::Tuple::vector(0.0, -0.1, 0.0),
    wind: math::Tuple::vector(-0.01, 0.0, 0.0),
  };
  let mut new_proj = projectile;
  let mut c = canvas::Canvas::new(900, 550);
  let color = color::Color::new(1.0, 0.0, 0.0);
  while new_proj.position.y > 0.0 {
    new_proj = tick(&env, new_proj);
    println!("{:?}", new_proj.position);
    c.write_pixel_f(new_proj.position.x, new_proj.position.y, color.clone());
  }
  let ppm = c.to_ppm();
  canvas::Canvas::write_ppm_to_disk(
    &"/Users/torleifs/code/rust/raytracer/test.ppm".to_string(),
    &ppm,
  );
  println!("Hello, world!");
}
