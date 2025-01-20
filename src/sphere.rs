use crate::hittable::*;
use crate::ray::Ray;
use crate::vec::Point3;
use std::ops::Range;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Range<f32>) -> Option<HitRecord> {
        let oc = self.center.sub(r.orig);

        let a = r.dir.len_square();
        let h = r.dir.dot(oc);
        let c = oc.len_square() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        let sqrt_disc = if discriminant >= 0.0 {
            discriminant.sqrt()
        } else {
            return None;
        };

        let root = (h - sqrt_disc) / a;

        if ray_t.contains(&root) {
            let t = root;
            let p = r.at(root);
            let outward_normal = p.sub(self.center).mul_scalar(1.0 / self.radius);
            return Some(HitRecord::with_outward_normal(p, t, r, outward_normal));
        }

        let root = (h + sqrt_disc) / a;
        if ray_t.contains(&root) {
            let t = root;
            let p = r.at(root);
            let outward_normal = p.sub(self.center).mul_scalar(1.0 / self.radius);
            return Some(HitRecord::with_outward_normal(p, t, r, outward_normal));
        }

        return None;
    }
}
