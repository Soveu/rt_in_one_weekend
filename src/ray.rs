use crate::vec::{Vec3, Point3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point3 {
        self.orig.add(self.dir.mul_scalar(t))
    }
}
