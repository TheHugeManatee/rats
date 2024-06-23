use crate::color::*;
use crate::geometry::*;
use crate::maths::*;
use crate::scene::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pixel_width: f64,
    pixel_height: f64,
    focal_length: f64,
    origin: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        pixel_width: f64,
        pixel_height: f64,
        pixel_aspect_ratio: f64,
        focal_length: f64,
        origin: Vec3,
    ) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * pixel_width / pixel_height;
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0) * pixel_aspect_ratio;
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // horizontal and vertical delta vectors
        let pixel_delta_u = viewport_u / pixel_width;
        let pixel_delta_v = viewport_v / pixel_height;

        // location of the upper left pixel
        let viewport_upper_left =
            origin - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;
        Self {
            pixel_width,
            pixel_height,
            focal_length,
            origin,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    // takes fractional pixel positions x and y
    pub fn get_pixel_ray(&self, x: f64, y: f64) -> Ray {
        let pixel_center = self.pixel00_loc + self.pixel_delta_u * x + self.pixel_delta_v * y;

        // create a ray from the camera origin to the pixel
        // direction is intentionally not normalized
        Ray::new(self.origin, pixel_center - self.origin)
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color {
        match world.hit(&ray, &Interval::new(0.0, f64::INFINITY)) {
            Some(hit) => {
                let normal = hit.normal;
                Vec3::new(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5
            }
            None => {
                // background: lerp from white to blue
                let unit_direction = ray.direction.normalized();
                let a = 0.5 * (unit_direction.y + 1.0);
                Vec3::lerp(Vec3::new(1.0, 1.0, 1.0), Vec3::new(0.5, 0.7, 1.0), a)
            }
        }
    }
}
