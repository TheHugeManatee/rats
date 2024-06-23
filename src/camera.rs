use crate::color::*;
use crate::geometry::*;
use crate::maths::*;
use crate::scene::*;

pub struct Camera {
    pub focal_length: f64,
    pub origin: Vec3,
}

impl Camera {
    pub fn new(focal_length: f64, origin: Vec3) -> Self {
        Self {
            focal_length,
            origin,
        }
    }

    pub fn render(&self, scene: &HittableList) {
        // ...
    }

    fn ray_color(&self, ray: &Ray, scene: &HittableList) -> Color {
        // ...
        Vec3::default()
    }
}
