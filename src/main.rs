use crate::math::color::Color;
use crate::math::vector::Vector3;
use crate::world::object::Sphere;
use crate::world::world::World;

mod math;
mod world;

fn main() {
    // print any messages that originate from dependencies
    env_logger::init();

    // Create a mutable world where objects can be added and a camera can be set
    let mut world = World::new("Chromatic - Raytracer".to_string(), 1920, 1280);

    // Initialize the camera values. 'up' and 'right' correspond to directions to determine
    // which direction the camera is facing
    let position = Vector3::new(0.0, 0.0, 0.0);
    let up = Vector3::new(0.0, 1.0, 0.0);
    let right = Vector3::new(1.0, 0.0, 0.0);
    world.camera(position, up, right);

    // Add two spheres to the world
    let sphere1 = Sphere::new(Vector3::new(0.0, 0.0, -1.5), 0.5, Color::new(255, 0, 0));
    let sphere2 = Sphere::new(Vector3::new(0.0, -100.5, 0.0), 100.0, Color::new(0, 128, 128));
    world.add_object(Box::new(sphere1));
    world.add_object(Box::new(sphere2));

    world.run();
}
