use crate::world::camera::Camera;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use crate::math::vector::Vector3;
use crate::world::object::Object;

pub struct World {
    name: String,
    width: u32,
    height: u32,
    camera: Option<Camera>,
    objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new(name: String, width: usize, height: usize) -> World {
        World {
            name,
            width: width as u32,
            height: height as u32,
            camera: None,
            objects: Vec::new(),
        }
    }

    pub fn camera(&mut self, position: Vector3, up: Vector3, right: Vector3) {
        let up = up.normalize() * 2.0;
        let right = {
            let aspect_ratio = (self.width as f64) / (self.height as f64);
            right.normalize() * 2.0 * aspect_ratio
        };
        let camera = Camera::new(position, up, right, 1.0);

        self.camera = Some(camera);
    }

    pub fn add_object(&mut self, object: Box<dyn Object>) {
        self.objects.push(object);
    }

    pub fn run(self) -> ! {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Chromatic")
            .with_inner_size(PhysicalSize::new(self.width, self.height))
            .build(&event_loop)
            .unwrap();

        let mut pixels = {
            let surface_texture = SurfaceTexture::new(self.width, self.height, &window);
            Pixels::new(self.width, self.height, surface_texture).unwrap()
        };

        if let Some(camera) = self.camera {
            camera.render_to(&mut pixels, &self.objects);
        }

        event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    window_id,
                } if window_id == window.id() => control_flow.set_exit(),
                Event::RedrawRequested(_) => {
                    let _ = pixels.render();
                }
                _ => (),
            }
        });
    }
}
