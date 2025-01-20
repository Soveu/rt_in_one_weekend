// Range: [0.0, 1.0)
#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub [f32; 3]);
pub type Point3 = Vec3;

impl Vec3 {
    pub const fn new() -> Self {
        Self([0.0; 3])
    }

    pub const fn add(self, other: Self) -> Self {
        Self([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }

    pub const fn sub(self, other: Self) -> Self {
        Self([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }

    pub const fn mul_scalar(self, x: f32) -> Self {
        Self([
            self.0[0] * x, 
            self.0[1] * x, 
            self.0[2] * x, 
        ])
    }

    pub fn len_square(self) -> f32 {
        let [x, y, z] = self.0;
        return x*x + y*y + z*z;
    }
    
    pub fn len(self) -> f32 {
        return self.len_square().sqrt();
    }

    pub fn unit(self) -> Self {
        self.mul_scalar(1.0 / self.len())
    }

    pub fn dot(self, other: Self) -> f32 {
        let [a, b, c] = self.0;
        let [d, e, f] = other.0;

        return a*d + b*e + c*f;
    }

    pub fn cross(self, other: Self) -> Self {
        let [a, b, c] = self.0;
        let [d, e, f] = other.0;

        Self([
            b*f - c*e,
            c*d - a*f,
            a*e - b*d,
        ])
    }
}
