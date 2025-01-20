use crate::sphere::Sphere;
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::ops::Range;

pub struct World {
    pub spheres: Vec<Sphere>,
}

impl World {
    pub const fn new() -> Self {
        Self { spheres: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
    }

    pub fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        self.spheres.iter()
            .fold((None, ray_t), |(result, ray_t), s| {
                match s.hit(r, ray_t.clone()) {
                    Some(hh) => (Some(hh), (ray_t.start .. hh.t)),
                    None => (result, ray_t),
                }
            }).0
    }
}
