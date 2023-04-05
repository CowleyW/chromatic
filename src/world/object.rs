use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vector::Vector3;

pub trait Object {
    fn hit_by(&self, ray: &Ray) -> bool;
    fn color(&self) -> Color;
}

pub struct Sphere {
    position: Vector3,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(position: Vector3, radius: f64, color: Color) -> Sphere {
        Sphere {
            position,
            radius,
            color,
        }
    }
}

impl Object for Sphere {
    fn hit_by(&self, ray: &Ray) -> bool {
        let oc = ray.origin - self.position;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction) * 2.0;
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }

    fn color(&self) -> Color {
        self.color
    }
}
