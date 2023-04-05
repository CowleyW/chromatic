use crate::math::ray::Ray;
use crate::math::vector::Vector3;
use crate::world::object::Object;
use pixels::Pixels;
use std::ops::Deref;

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

    pub fn render_to(&self, pixels: &mut Pixels, objects: &Vec<Box<dyn Object>>) {
        let width = pixels.texture().width();
        let height = pixels.texture().height();

        // Lower-left corner
        let llc = self.position
            - self.right / 2.0
            - self.up / 2.0
            - Vector3::new(0.0, 0.0, self.focal_len);

        let mut buf = image::ImageBuffer::new(width, height);

        for (x, y, pixel) in buf.enumerate_pixels_mut() {
            let u = x as f64 / (width - 1) as f64;
            let v = (height - y) as f64 / (height - 1) as f64;
            let ray = Ray::new(
                self.position,
                llc + self.right * u + self.up * v - self.position,
            );

            *pixel = image::Rgb(ray.color().data());
            for object in objects.into_iter() {
                if let Some(t) = object.hit_at(&ray) {
                    let normal = (ray.at(t) - object.position()).normalize();

                    let r = ((normal.x + 1.0) * 127.0) as u8;
                    let g = ((normal.y + 1.0) * 127.0) as u8;
                    let b = ((normal.z + 1.0) * 127.0) as u8;

                    *pixel = image::Rgb([r, g, b]);
                }
            }
        }

        let _ = buf.save("image.png");

        pixels
            .frame_mut()
            .chunks_exact_mut(4)
            .zip(buf.chunks_exact(3))
            .for_each(|(pixel, im_pixel)| {
                pixel[0] = im_pixel[0];
                pixel[1] = im_pixel[1];
                pixel[2] = im_pixel[2];
                pixel[3] = 0xff;
            });
    }
}
