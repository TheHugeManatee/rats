use crate::color::*;
use crate::geometry::{Ray, Sphere};
use crate::maths::{vec2, vec3};
use ratatui::prelude::Color;

pub struct Camera {
    pub focal_length: f64,
    pub origin: vec3,
}

pub struct Renderer {
    dimensions: (usize, usize),
    color_buffer: Vec<Vec<Color>>,
    next_line_to_process: usize,
    camera: Camera,
    pixel00_loc: vec3,
    pixel_delta_u: vec3,
    pixel_delta_v: vec3,
}

impl Renderer {
    fn get_color_buffer_size(&self) -> (usize, usize) {
        let width = self.color_buffer.len();
        let height = self.color_buffer[0].len();
        (width, height)
    }

    pub fn new(width: usize, height: usize) -> Renderer {
        let color_buffer = vec![vec![Color::default(); width]; height];

        // camera parameters and vectors across full viewport
        let focal_length = 1.0;
        let camera_center = vec3::zero();
        let viewport_height = 2.0;
        let viewport_width = viewport_height * width as f64 / height as f64;
        let viewport_u = vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = vec3::new(0.0, -viewport_height, 0.0);

        // horizontal and vertical delta vectors
        let pixel_delta_u = viewport_u / width as f64;
        let pixel_delta_v = viewport_v / height as f64;

        // location of the upper left pixel
        let viewport_upper_left =
            camera_center - vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        Renderer {
            dimensions: (width, height),
            color_buffer,
            next_line_to_process: 0,
            camera: Camera {
                focal_length,
                origin: camera_center,
            },
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        let (_width, height) = self.get_color_buffer_size();

        self.next_line_to_process as f64 / height as f64
    }

    pub fn get_color_buffer(&self) -> &Vec<Vec<Color>> {
        &self.color_buffer
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
            let pixel_center = self.pixel00_loc + self.pixel_delta_u * x + self.pixel_delta_v * y;
            // create a ray from the camera origin to the pixel
            // direction is intentionally not normalized
            let ray = Ray::new(self.camera.origin, pixel_center - self.camera.origin);

            *pixel = Self::ray_color(ray);
        }
    }

    fn ray_color(ray: Ray) -> Color {
        // hit sphere
        if Sphere::hit(vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
            return vec3::new(1.0, 0.0, 0.0).to_color();
        }

        // background: lerp from white to blue
        let unit_direction = ray.direction.normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        vec3::lerp(vec3::new(1.0, 1.0, 1.0), vec3::new(0.5, 0.7, 1.0), a).to_color()
    }
}
