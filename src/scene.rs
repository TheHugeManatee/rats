use crate::color::Color;
use crate::geometry::*;
use crate::materials::{Dielectric, Lambertian, Metal};
use crate::maths::{Interval, Point};
use std::rc::Rc;

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

        let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
        let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
        let material_left = Rc::new(Dielectric::new(1.50));
        let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
        let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

        world.add(Box::new(Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )));
        world.add(Box::new(Sphere::new(
            Point::new(0.0, 0.0, -1.2),
            0.5,
            material_center,
        )));
        world.add(Box::new(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.5,
            material_left,
        )));
        world.add(Box::new(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.4,
            material_bubble,
        )));
        world.add(Box::new(Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            material_right,
        )));

        world
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn object_count(&self) -> usize {
        self.objects.len()
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
