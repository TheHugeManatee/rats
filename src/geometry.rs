use crate::maths::*;

pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3, // normal of the hit point, always points against the ray
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // the normal given on construction is expected to point outward from the surface
    pub fn new(point: Point, outward_normal: Vec3, t: f64, ray: &Ray) -> Self {
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal * -1.0
        };

        Self {
            point,
            normal,
            t,
            front_face,
        }
    }
}

// The hittable trait is used to define objects that can be hit by a ray
pub trait Hittable {
    // The hit method is used to determine if a ray hits the object
    // and if it does, it returns a HitRecord
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
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
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
    pub fn hit_normal(&self, point_on_surface: Point) -> Vec3 {
        (point_on_surface - self.center).normalized()
    }
}

impl Hittable for Sphere {
    // returns the closest hit t for the ray
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = Vec3::dot(ray.direction, oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        // no intersection
        if discriminant < 0.0 {
            return None;
        }

        let t = (h - discriminant.sqrt()) / a;

        if t < t_max && t >= t_min {
            let point = ray.at(t);
            let normal = self.hit_normal(point);
            return Some(HitRecord::new(point, normal, t, ray));
        }
        None
    }
}
