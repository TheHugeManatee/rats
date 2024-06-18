use crate::maths::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    // returns the closest hit t for the ray
    pub fn hit(center: Vec3, radius: f64, ray: &Ray) -> Option<f64> {
        let oc = center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(ray.direction, oc);
        let c = oc.length_squared() - radius * radius;
        let discriminant = h * h - a * c;

        // no intersection
        if discriminant < 0.0 {
            return None;
        }

        Some((-h - discriminant.sqrt()) / a)
    }

    pub fn hit_normal(&self, point_on_surface: Vec3) -> Vec3 {
        (point_on_surface - self.center).normalized()
    }
}
