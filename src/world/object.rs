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

        let a = ray.direction.len_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            None
        } else {
            let root = (-half_b - discriminant.sqrt()) / a;
            if root < 0.0 {
                let root = (-half_b + discriminant.sqrt()) / a;
                if root < 0.0 {
                    None
                }
                else {
                    Some(root)
                }
            }
            else {
                Some(root)
            }
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> Vector3 {
        self.position
    }
}

