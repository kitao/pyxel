use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const UP: Self = Self {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    #[must_use]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::ZERO
        } else {
            self * (1.0 / len)
        }
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

/// 4x4 matrix in column-major order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub m: [f32; 16],
}

impl Mat4 {
    // Column-major index: m[col * 4 + row]
    #[must_use]
    pub fn identity() -> Self {
        let mut m = [0.0; 16];
        m[0] = 1.0;
        m[5] = 1.0;
        m[10] = 1.0;
        m[15] = 1.0;
        Self { m }
    }

    #[must_use]
    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut m = Self::identity();
        m.m[12] = x;
        m.m[13] = y;
        m.m[14] = z;
        m
    }

    #[must_use]
    pub fn scale(sx: f32, sy: f32, sz: f32) -> Self {
        let mut m = Self::identity();
        m.m[0] = sx;
        m.m[5] = sy;
        m.m[10] = sz;
        m
    }

    #[must_use]
    pub fn rotation_x(deg: f32) -> Self {
        let r = deg.to_radians();
        let (s, c) = (r.sin(), r.cos());
        let mut m = Self::identity();
        m.m[5] = c;
        m.m[6] = s;
        m.m[9] = -s;
        m.m[10] = c;
        m
    }

    #[must_use]
    pub fn rotation_y(deg: f32) -> Self {
        let r = deg.to_radians();
        let (s, c) = (r.sin(), r.cos());
        let mut m = Self::identity();
        m.m[0] = c;
        m.m[2] = -s;
        m.m[8] = s;
        m.m[10] = c;
        m
    }

    #[must_use]
    pub fn rotation_z(deg: f32) -> Self {
        let r = deg.to_radians();
        let (s, c) = (r.sin(), r.cos());
        let mut m = Self::identity();
        m.m[0] = c;
        m.m[1] = s;
        m.m[4] = -s;
        m.m[5] = c;
        m
    }

    #[must_use]
    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let f = (target - eye).normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);
        let mut m = Self::identity();
        m.m[0] = s.x;
        m.m[4] = s.y;
        m.m[8] = s.z;
        m.m[1] = u.x;
        m.m[5] = u.y;
        m.m[9] = u.z;
        m.m[2] = -f.x;
        m.m[6] = -f.y;
        m.m[10] = -f.z;
        m.m[12] = -s.dot(eye);
        m.m[13] = -u.dot(eye);
        m.m[14] = f.dot(eye);
        m
    }

    #[must_use]
    pub fn perspective(fov_deg: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fov_deg.to_radians() / 2.0).tan();
        let nf = 1.0 / (near - far);
        let mut m = [0.0; 16];
        m[0] = f / aspect;
        m[5] = f;
        m[10] = (far + near) * nf;
        m[11] = -1.0;
        m[14] = 2.0 * far * near * nf;
        Self { m }
    }

    #[must_use]
    pub fn transform_point(self, v: Vec3) -> Vec3 {
        let w = self.m[3] * v.x + self.m[7] * v.y + self.m[11] * v.z + self.m[15];
        let inv_w = if w == 0.0 { 1.0 } else { 1.0 / w };
        Vec3::new(
            (self.m[0] * v.x + self.m[4] * v.y + self.m[8] * v.z + self.m[12]) * inv_w,
            (self.m[1] * v.x + self.m[5] * v.y + self.m[9] * v.z + self.m[13]) * inv_w,
            (self.m[2] * v.x + self.m[6] * v.y + self.m[10] * v.z + self.m[14]) * inv_w,
        )
    }

    /// Transform a point and return (NDC position, clip-space w).
    /// w <= 0 means the point is behind the camera.
    #[must_use]
    pub fn transform_point_w(self, v: Vec3) -> (Vec3, f32) {
        let w = self.m[3] * v.x + self.m[7] * v.y + self.m[11] * v.z + self.m[15];
        let inv_w = if w == 0.0 { 1.0 } else { 1.0 / w };
        (
            Vec3::new(
                (self.m[0] * v.x + self.m[4] * v.y + self.m[8] * v.z + self.m[12]) * inv_w,
                (self.m[1] * v.x + self.m[5] * v.y + self.m[9] * v.z + self.m[13]) * inv_w,
                (self.m[2] * v.x + self.m[6] * v.y + self.m[10] * v.z + self.m[14]) * inv_w,
            ),
            w,
        )
    }

    #[must_use]
    pub fn transform_dir(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.m[0] * v.x + self.m[4] * v.y + self.m[8] * v.z,
            self.m[1] * v.x + self.m[5] * v.y + self.m[9] * v.z,
            self.m[2] * v.x + self.m[6] * v.y + self.m[10] * v.z,
        )
    }
}

impl Mul for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut m = [0.0; 16];
        for col in 0..4 {
            for row in 0..4 {
                m[col * 4 + row] = (0..4)
                    .map(|k| self.m[k * 4 + row] * rhs.m[col * 4 + k])
                    .sum();
            }
        }
        Self { m }
    }
}
