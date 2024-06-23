use crate::camera::Camera;
use crate::color::Color;
use crate::maths::*;
use crate::scene::HittableList;

pub struct Renderer {
    dimensions: (usize, usize),
    color_buffer: Vec<Vec<Color>>,
    next_line_to_process: usize,
    camera: Camera,

    world: HittableList,
}

impl Renderer {
    fn get_color_buffer_size(&self) -> (usize, usize) {
        let width = self.color_buffer.len();
        let height = self.color_buffer[0].len();
        (width, height)
    }

    pub fn new(width: usize, height: usize) -> Renderer {
        let color_buffer = vec![vec![Color::default(); width]; height];

        let pixel_aspect_ratio = 10.0 / 20.0;
        let focal_length = 1.0;
        let camera_center = Vec3::zero();

        Renderer {
            dimensions: (width, height),
            color_buffer,
            next_line_to_process: 0,
            camera: Camera::new(
                width as f64,
                height as f64,
                pixel_aspect_ratio,
                focal_length,
                camera_center,
            ),
            world: HittableList::default(),
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        let (_width, height) = self.get_color_buffer_size();

        self.next_line_to_process as f64 / height as f64
    }

    pub fn width(&self) -> usize {
        self.dimensions.0
    }
    pub fn height(&self) -> usize {
        self.dimensions.1
    }

    pub fn get_color_buffer(&self) -> &Vec<Vec<Color>> {
        &self.color_buffer
    }

    pub fn get_scene_object_count(&self) -> usize {
        self.world.object_count()
    }

    pub fn render_step(&mut self) {
        // render line-by line, but only for a maximum of 10ms
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_millis(15) {
            if self.next_line_to_process < self.color_buffer.len() {
                self.render_line(self.next_line_to_process);
                self.next_line_to_process += 1;
            } else {
                break;
            }
        }
    }

    fn render_line(&mut self, line_index: usize) {
        let row = self.color_buffer.get_mut(line_index).unwrap();
        for (xi, pixel) in row.iter_mut().enumerate() {
            let x: f64 = xi as f64;
            let y: f64 = line_index as f64;

            let ray = self.camera.get_pixel_ray(x, y);

            *pixel = self.camera.ray_color(&ray, &self.world);
        }
    }
}
