use crate::math::color::Color;
use crate::math::vector::Vector3;
use crate::world::object::Sphere;
use crate::world::world::World;

mod math;
mod world;

fn main() {
    env_logger::init();

    let position = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = Vector3::new(1.0, 0.0, 0.0);

    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -3.0), 0.5, Color::new(255, 0, 0));
    let sphere2 = Sphere::new(Vector3::new(-0.5, 0.0, -2.0), 0.25, Color::new(0, 128, 128));

    let mut world = World::new("Chromatic - Raytracer".to_string(), 1920, 1280);
    world.camera(position, up, right);
    world.add_object(Box::new(sphere1));
    world.add_object(Box::new(sphere2));
    world.run();
}
