use crate::camera::Camera;
use crate::color::Color;
use crate::maths::*;
use crate::random::*;
use crate::scene::HittableList;
use crate::terminal::*;

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
    pixels: Vec<TerminalPixel>,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            width,
            height,
            pixels: vec![TerminalPixel::default(); width * height],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> TerminalPixel {
        self.pixels[y * self.width + x]
    }
    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut TerminalPixel {
        &mut self.pixels[y * self.width + x]
    }
    pub fn get_rows_mut(&mut self) -> Vec<&mut [TerminalPixel]> {
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
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

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
            samples_per_pixel: 512,
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

        // render line-by line, but only for a maximum of 15ms
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
            let sample_scale = 1.0 / self.samples_per_pixel as f64;

            // note: no gamma correction needed for now because we directly display without
            // saving to a gamma file format
            *pixel = Renderer::render_pixel_samples(
                self.samples_per_pixel,
                x,
                y,
                sample_scale,
                &self.camera,
                &self.world,
                self.max_depth,
            );
        }
    }

    fn render_pixel_samples(
        samples_per_pixel: usize,
        x: f64,
        y: f64,
        sample_scale: f64,
        camera: &Camera,
        world: &HittableList,
        max_depth: i32,
    ) -> TerminalPixel {
        let mut pixel = RenderPixel::default();

        let subpixels_per_pixel = SUBPIXEL_X * SUBPIXEL_Y;
        let subpixel_size = Vec3::new(1.0 / SUBPIXEL_X as f64, 1.0 / SUBPIXEL_Y as f64, 0.0);
        let subpixel_sample_scale = sample_scale * subpixels_per_pixel as f64;

        for _ in 0..samples_per_pixel / subpixels_per_pixel {
            for subpixel_y in 0..SUBPIXEL_Y {
                for subpixel_x in 0..SUBPIXEL_X {
                    //let offset = Vec3::new(subpixel_x as f64, subpixel_y as f64, 0.0)
                    //    + Renderer::sample_square() * subpixel_size;
                    //let offset = Vec3::default();
                    let offset =
                        Vec3::new(subpixel_x as f64, subpixel_y as f64, 0.0) * subpixel_size;
                    let ray = camera.get_pixel_ray(x + offset.x, y + offset.y);

                    let mut subpx_color = pixel.get_color(subpixel_x, subpixel_y);
                    subpx_color += camera.ray_color(&ray, max_depth, world) * subpixel_sample_scale;
                    pixel.set_color(subpixel_x, subpixel_y, subpx_color);
                }
            }
        }
        //let col = pixel.average_color();
        //TerminalPixel::new(col, col, ' ')
        pixel.to_terminal_pixel()
    }

    fn sample_square() -> Vec3 {
        // sampl a random square in the pixel
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
