use crate::geometry::*;
use crate::maths::*;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn default() -> Self {
        let mut world = Self::new();
        world.add(Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }));
        world.add(Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }));
        world
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, interval))
            .fold(None, |acc, record| match acc {
                Some(acc_record) if acc_record.t < record.t => Some(acc_record),
                _ => Some(record),
            })
    }
}
