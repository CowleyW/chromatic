use crate::math::gen_2d_range;
use crate::math::ray::Ray;
use crate::math::vector::Vector3;
use crate::world::object::Object;
use pixels::Pixels;
use std::cmp::Ordering;
use image::Rgb;

#[derive(Debug)]
pub struct Camera {
    position: Vector3,
    up: Vector3,
    right: Vector3,
    focal_len: f64,
}

impl Camera {
    pub fn new(position: Vector3, up: Vector3, right: Vector3, focal_len: f64) -> Camera {
        Camera {
            position,
            up,
            right,
            focal_len,
        }
    }

    pub fn render_to(&self, width: u32, height: u32, objects: &Vec<Box<dyn Object>>) -> image::ImageBuffer<Rgb<u8>, Vec<u8>>{
        // Lower-left corner
        let llc = self.position
            - self.right / 2.0
            - self.up / 2.0
            - Vector3::new(0.0, 0.0, self.focal_len);

        let mut buf = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            if x == 0 {
                println!("Beginning line {}, ========> {:.1}%.", y, ((y * 100) as f64) / height as f64)
            }
            let mut cumulative_rgb = [0.0, 0.0, 0.0];
            for (x_offset, y_offset) in gen_2d_range(-1, 2) {
                let u = (x as f64 - x_offset as f64) / (width - 1) as f64;
                let v = ((height - y) as f64 - y_offset as f64) / (height - 1) as f64;
                let ray = Ray::new(
                    self.position,
                    llc + self.right * u + self.up * v - self.position,
                );
                let ray_color = ray.color(&objects, 50);

                cumulative_rgb[0] += ray_color[0];
                cumulative_rgb[1] += ray_color[1];
                cumulative_rgb[2] += ray_color[2];
            }

            let final_rgb = [
                (255.0 * (cumulative_rgb[0] / 9.0).sqrt()) as u8,
                (255.0 * (cumulative_rgb[1] / 9.0).sqrt()) as u8,
                (255.0 * (cumulative_rgb[2] / 9.0).sqrt()) as u8,
            ];
            *pixel = image::Rgb(final_rgb);
        }

        let _ = buf.save("image.png");

        buf
    }
}
