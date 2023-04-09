use crate::math::rand_sphere_coord;
use crate::math::ray::Ray;
use crate::math::vector::Vector3;

pub struct HitRecord {
    location: Vector3,
    normal: Vector3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn new(location: Vector3, outward_normal: Vector3, ray_direction: Vector3, t: f64) -> HitRecord {
        let front_face = ray_direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        }
        else {
            outward_normal * -1.0
        };

        HitRecord {
            location,
            normal,
            t,
            front_face
        }
    }

    pub fn bounce_ray(&self) -> Ray {
        Ray {
            origin: self.location,
            direction: self.normal + rand_sphere_coord()
        }
    }
}