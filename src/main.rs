mod math;
struct Environment {
    pub gravity: math::Tuple,
    pub wind: math::Tuple,
}
#[derive(Copy, Clone)]
struct Projectile {
    position: math::Tuple,
    velocity: math::Tuple,
}
impl Projectile {
    pub fn projectile(position: math::Tuple, velocity: math::Tuple) -> Projectile {
        Projectile { position, velocity }
    }
}

fn tick(environment: &Environment, projectile: &Projectile) -> Projectile {
    let velocity = environment.gravity + projectile.velocity;
    let new_pos = projectile.position + projectile.velocity;
    return Projectile::projectile(new_pos, velocity);
}
fn main() {
    let projectile = Projectile::projectile(
        math::Tuple::point(0.0, 1.0, 0.0),
        math::Tuple::vector(1.0, 1.0, 0.0),
    );
    let env = Environment {
        gravity: math::Tuple::vector(0.0, -0.01, 0.0),
        wind: math::Tuple::vector(-0.01, 0.0, 0.0),
    };
    let mut new_proj = projectile;
    while new_proj.position.y > 0.0 {
        new_proj = tick(&env, &new_proj);
        println!("{:?}", new_proj.position);
    }
    println!("Hello, world!");
}
