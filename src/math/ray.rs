use crate::math::color::Color;
use crate::math::vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.direction * t)
    }

    pub fn color(&self) -> Color {
        let dir = self.direction.normalize();

        let t = 0.5 * (dir.y + 1 as f64);
        let base = (255.0 * (1.0 - t)) as u8;

        let r = (127.0 * t) as u8;
        let g = (178.2 * t) as u8;
        let b = (255.0 * t) as u8;
        Color {
            r: base + r,
            g: base + g,
            b: base + b,
        }
    }
}
