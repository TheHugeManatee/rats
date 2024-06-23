use crate::maths::Vec3;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

pub fn random_vec3() -> Vec3 {
    Vec3::new(random_double(), random_double(), random_double())
}
pub fn random_vec3_min_max(min: f64, max: f64) -> Vec3 {
    Vec3::new(
        random_double_range(min, max),
        random_double_range(min, max),
        random_double_range(min, max),
    )
}

pub fn random_vec3_unit() -> Vec3 {
    // generate xyz from gaussian distribution
    let mut rng = rand::thread_rng();

    Vec3 {
        x: rng.sample(StandardNormal),
        y: rng.sample(StandardNormal),
        z: rng.sample(StandardNormal),
    }
    .normalized()
}

pub fn random_vec3_on_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_vec3_unit();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
