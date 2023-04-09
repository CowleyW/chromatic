use crate::math::vector::Vector3;
use rand::Rng;

pub mod color;
pub mod ray;
pub mod vector;
pub mod hit_record;

pub fn gen_2d_range(from: i32, to: i32) -> impl Iterator<Item = (i32, i32)> {
    (from..to).flat_map(move |a| (from..to).map(move |b| (a, b)))
}

pub fn rand_sphere_coord() -> Vector3 {
    let mut rng = rand::thread_rng();
    let mut len = 2.0;

    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    while len > 1.0 {
        x = rng.gen_range(-1.0..1.0);
        y = rng.gen_range(-1.0..1.0);
        z = rng.gen_range(-1.0..1.0);

        len = (x*x + y*y + z*z as f64).sqrt();
    }

    Vector3 {x, y, z}.normalize()
}