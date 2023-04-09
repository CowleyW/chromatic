use crate::math::color::Color;
use crate::math::hit_record::HitRecord;
use crate::math::vector::Vector3;
use crate::world::object::Object;
use std::cmp::Ordering;

/// A struct that represents a ray. Each ray has an origin and a direction vector which establishes
/// the property of the ray
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    /// Returns a new ray with the given origin and direction
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    /// Returns the vector at location `t` along the ray
    ///
    /// This location is equal to `O + D(t)`
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.direction * t)
    }

    /// Returns the color of the ray
    pub fn color(&self, objects: &Vec<Box<dyn Object>>, depth: usize) -> [f64; 3] {
        if depth == 0 {
            return [0.0, 0.0, 0.0];
        }

        let closest_object = objects
            .into_iter()
            .filter_map(|o| {
                if let Some(t) = o.hit_at(self) {
                    Some((t, o))
                } else {
                    None
                }
            })
            .min_by(|(t1, _), (t2, _)| {
                if t1 < t2 {
                    Ordering::Less
                } else if t1 > t2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

        if let Some((t, o)) = closest_object {
            let normal = (self.at(t) - o.position()).normalize();
            let bounce_ray = HitRecord::new(self.at(t), normal, self.direction, t).bounce_ray();

            let color = bounce_ray.color(&objects, depth - 1);
            [color[0] / 2.0, color[1] / 2.0, color[2] / 2.0]
        } else {
            let t = 0.5 * (self.direction.y + 1.0);
            let base = (1.0 - t);

            let r = (0.5 * t) + base;
            let g = (0.7 * t) + base;
            let b = t + base;

            [r, g, b]
        }
    }
}
