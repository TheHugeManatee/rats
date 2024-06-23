use crate::camera::Camera;
use crate::color::Color;
use crate::maths::*;
use crate::random::*;
use crate::scene::HittableList;

pub struct Renderer {
    color_buffer: FrameBuffer,
    next_line_to_process: usize,
    render_duration: std::time::Duration,
    camera: Camera,
    samples_per_pixel: usize, // Count of random samples for each pixel
    max_depth: i32,           // Maximum number of ray bounces into scene
    world: HittableList,
}

pub struct FrameBuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            width,
            height,
            pixels: vec![Color::default(); width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Color {
        &mut self.pixels[y * self.width + x]
    }
    pub fn get_rows_mut(&mut self) -> Vec<&mut [Color]> {
        self.pixels.chunks_mut(self.width).collect()
    }
    pub fn get_size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl Renderer {
    fn get_color_buffer_size(&self) -> (usize, usize) {
        self.color_buffer.get_size()
    }

    pub fn get_render_duration(&self) -> std::time::Duration {
        self.render_duration
    }

    pub fn new(width: usize, height: usize) -> Renderer {
        let color_buffer = FrameBuffer::new(width, height);

        let pixel_aspect_ratio = 10.0 / 20.0;
        let focal_length = 1.0;
        let camera_center = Vec3::zero();

        Renderer {
            color_buffer,
            next_line_to_process: 0,
            render_duration: std::time::Duration::from_micros(0),
            camera: Camera::new(
                width as f64,
                height as f64,
                pixel_aspect_ratio,
                focal_length,
                camera_center,
            ),
            samples_per_pixel: 500,
            max_depth: 10,
            world: HittableList::default(),
        }
    }

    pub fn get_progress_percentage(&self) -> f64 {
        let (_width, height) = self.get_color_buffer_size();
        let progress_interval = Interval { min: 0.0, max: 1.0 };
        progress_interval.clamp(self.next_line_to_process as f64 / height as f64)
    }

    pub fn get_color_buffer(&self) -> &FrameBuffer {
        &self.color_buffer
    }

    pub fn get_scene_object_count(&self) -> usize {
        self.world.object_count()
    }

    pub fn render_step(&mut self) {
        let mut lines_processed = 0;

        // render line-by line, but only for a maximum of 10ms
        let start = std::time::Instant::now();
        while start.elapsed() < std::time::Duration::from_millis(15) {
            if self.next_line_to_process < self.color_buffer.height {
                self.render_line(self.next_line_to_process);
                self.next_line_to_process += 1;
                lines_processed += 1;
            } else {
                break;
            }
        }
        // only update render duration if we actually rendered something
        if lines_processed > 0 {
            self.render_duration += start.elapsed();
        }
    }

    fn render_line(&mut self, line_index: usize) {
        let row = &mut self.color_buffer.get_rows_mut()[line_index];

        for (xi, pixel) in row.iter_mut().enumerate() {
            let x: f64 = xi as f64;
            let y: f64 = line_index as f64;
            let mut pixel_color = Color::default();
            let sample_scale = 1.0 / self.samples_per_pixel as f64;
            for _ in 0..self.samples_per_pixel {
                let offset = Renderer::sample_square();

                let ray = self.camera.get_pixel_ray(x + offset.x, y + offset.y);
                pixel_color +=
                    self.camera.ray_color(&ray, self.max_depth, &self.world) * sample_scale;
            }
            *pixel = pixel_color;
        }
    }

    fn sample_square() -> Vec3 {
        // sampl a random square in the pixel
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
