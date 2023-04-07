use crate::math::gen_2d_range;
use crate::math::ray::Ray;
use crate::math::vector::Vector3;
use crate::world::object::Object;
use pixels::Pixels;
use std::cmp::Ordering;
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
            let mut cumulative_rgb = [0, 0, 0];
            for (x_offset, y_offset) in gen_2d_range(-1, 2) {
                let u = (x as f64 - x_offset as f64) / (width - 1) as f64;
                let v = ((height - y) as f64 - y_offset as f64) / (height - 1) as f64;
                let ray = Ray::new(
                    self.position,
                    llc + self.right * u + self.up * v - self.position,
                );
                let ray_color = self.color_at(ray, &objects);

                cumulative_rgb[0] += ray_color[0] as u16;
                cumulative_rgb[1] += ray_color[1] as u16;
                cumulative_rgb[2] += ray_color[2] as u16;
            }

            let final_rgb = [
                (cumulative_rgb[0] / 9) as u8,
                (cumulative_rgb[1] / 9) as u8,
                (cumulative_rgb[2] / 9) as u8,
            ];
            *pixel = image::Rgb(final_rgb);
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

    fn color_at(&self, ray: Ray, objects: &Vec<Box<dyn Object>>) -> [u8; 3] {
        let closest_object = objects
            .into_iter()
            .filter_map(|o| {
                if let Some(t) = o.hit_at(&ray) {
                    Some((t, o))
                } else {
                    None
                }
            })
            .min_by(|(t1, o1), (t2, o2)| {
                if t1 < t2 {
                    Ordering::Less
                } else if t1 > t2 {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

        if let Some((t, o)) = closest_object {
            let normal = (ray.at(t) - o.position()).normalize();

            let r = ((normal.x + 1.0) * 127.0) as u8;
            let g = ((normal.y + 1.0) * 127.0) as u8;
            let b = ((normal.z + 1.0) * 127.0) as u8;

            [r, g, b]
        } else {
            ray.color().data()
        }
    }
}
