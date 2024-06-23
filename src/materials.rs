use crate::color::Color;
use crate::geometry::{HitRecord, Ray};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.normal + crate::random::random_vec3_unit();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered_ray: Ray::new(hit_record.point, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = ray.direction.reflect(hit_record.normal);
        let scattered_direction = reflected + crate::random::random_vec3_unit() * self.fuzz;

        Some(ScatterRecord {
            attenuation: self.albedo,
            scattered_ray: Ray::new(hit_record.point, scattered_direction),
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if hit_record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.direction.normalized();

        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract
            || Self::reflectance(cos_theta, ri) > crate::random::random_double()
        {
            unit_direction.reflect(hit_record.normal)
        } else {
            crate::maths::refract(unit_direction, hit_record.normal, ri)
        };

        Some(ScatterRecord {
            attenuation,
            scattered_ray: Ray::new(hit_record.point, direction),
        })
    }
}
