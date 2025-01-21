use crate::vec::Vec3;

// Range: [0.0, 1.0)
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const fn new() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    fn _to_rgb16(x: f32) -> [u8; 2] {
        ((x.clamp(0.0, 0.9999) * 65536.0).floor() as u16).to_be_bytes()
    }

    pub fn to_rgb16(self) -> [u8; 6] {
        let r = Self::_to_rgb16(self.r);
        let g = Self::_to_rgb16(self.g);
        let b = Self::_to_rgb16(self.b);

        [r[0], r[1], g[0], g[1], b[0], b[1]]
    }

    fn _to_rgb8(x: f32) -> u8 {
        (x.clamp(0.0, 0.9999) * 256.0).floor() as u8
    }

    #[allow(dead_code)]
    pub fn to_rgb8(self) -> [u8; 3] {
        [
            Self::_to_rgb8(self.r),
            Self::_to_rgb8(self.g),
            Self::_to_rgb8(self.b),
        ]
    }

    pub fn to_vec3(self) -> Vec3 {
        Vec3([self.r, self.g, self.b])
    }

    pub fn from_vec3(v: Vec3) -> Self {
        Self {
            r: v.0[0],
            g: v.0[1],
            b: v.0[2],
        }
    }
}
