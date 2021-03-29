pub mod geometry;
pub mod rays;
pub mod lights;
pub mod materials;
pub mod world;
pub mod camera;
pub mod patterns;
pub use self::geometry::Sphere;
pub use self::geometry::Plane;
pub use self::rays::Intersection;
pub use self::rays::Ray;
pub use self::lights::PointLight;
pub use self::materials::Material;
pub use self::world::World;
pub use self::camera::Camera;

#[cfg(test)]
mod tests;
