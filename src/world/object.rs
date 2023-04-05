use crate::math::color::Color;
use crate::math::ray::Ray;
use crate::math::vector::Vector3;

pub trait Object {
    fn hit_at(&self, ray: &Ray) -> Option<f64>;
    fn color(&self) -> Color;
    fn position(&self) -> Vector3;
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
    fn hit_at(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.position;

        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction) * 2.0;
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Vector3 {
        self.position
    }
}
