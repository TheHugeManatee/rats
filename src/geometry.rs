use crate::maths::vec3;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: vec3,
    pub direction: vec3,
}

impl Ray {
    pub fn new(origin: vec3, direction: vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> vec3 {
        self.origin + self.direction * t
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    pub center: vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn hit(center: vec3, radius: f64, ray: &Ray) -> bool {
        let oc = center - ray.origin;
        let a = vec3::dot(ray.direction, ray.direction);
        let b = -2.0 * vec3::dot(ray.direction, oc);
        let c = vec3::dot(oc, oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;

        (discriminant >= 0.0)
    }
}
