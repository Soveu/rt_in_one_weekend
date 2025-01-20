use crate::ray::Ray;
use crate::vec::{Point3, Vec3};
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn with_outward_normal(p: Point3, t: f32, r: &Ray, outward_normal: Vec3) -> Self {
        let front_face = r.dir.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            outward_normal.mul_scalar(-1.0)
        };
        Self {
            p,
            t,
            front_face,
            normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord>;
}
