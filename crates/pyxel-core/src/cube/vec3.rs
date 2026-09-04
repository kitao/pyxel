use crate::cube::mat4::Mat4;

// Immutable 3D vector. Arithmetic and transform methods return new values.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

define_rc_type!(RcVec3, Vec3);

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> RcVec3 {
        new_rc_type!(Vec3 { x, y, z })
    }

    // Constants

    pub fn zero() -> RcVec3 {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn one() -> RcVec3 {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn right() -> RcVec3 {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn left() -> RcVec3 {
        Self::new(-1.0, 0.0, 0.0)
    }

    pub fn up() -> RcVec3 {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn down() -> RcVec3 {
        Self::new(0.0, -1.0, 0.0)
    }

    pub fn forward() -> RcVec3 {
        Self::new(0.0, 0.0, -1.0)
    }

    pub fn back() -> RcVec3 {
        Self::new(0.0, 0.0, 1.0)
    }

    // Operators

    pub fn add(&self, other: &Self) -> RcVec3 {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Self) -> RcVec3 {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn mul(&self, scalar: f32) -> RcVec3 {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }

    pub fn div(&self, scalar: f32) -> RcVec3 {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }

    pub fn neg(&self) -> RcVec3 {
        Self::new(-self.x, -self.y, -self.z)
    }

    // Math

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> RcVec3 {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        self.distance_squared_to(other).sqrt()
    }

    pub fn distance_squared_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn angle_to(&self, other: &Self) -> f32 {
        let len_product = self.length() * other.length();
        if len_product == 0.0 {
            0.0
        } else {
            (self.dot(other) / len_product)
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees()
        }
    }

    pub fn normalize(&self) -> RcVec3 {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }

    pub fn clamp_length(&self, max_length: f32) -> RcVec3 {
        let len = self.length();
        if len > max_length {
            let scale = max_length / len;
            Self::new(self.x * scale, self.y * scale, self.z * scale)
        } else {
            Self::new(self.x, self.y, self.z)
        }
    }

    pub fn min(&self, other: &Self) -> RcVec3 {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }

    pub fn max(&self, other: &Self) -> RcVec3 {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }

    pub fn lerp(&self, other: &Self, t: f32) -> RcVec3 {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }

    pub fn slerp(&self, other: &Self, t: f32) -> RcVec3 {
        let len_product = self.length() * other.length();
        if len_product == 0.0 {
            return self.lerp(other, t);
        }
        let dot = (self.dot(other) / len_product).clamp(-1.0, 1.0);
        let theta = dot.acos();
        if theta.abs() < 1e-6 {
            return self.lerp(other, t);
        }
        if std::f32::consts::PI - theta < 1e-3 {
            // Anti-parallel vectors: rotate through any perpendicular axis.
            let self_len = self.length();
            let axis = if self.x.abs() < 0.9 * self_len {
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                }
            } else {
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
            };
            let perp_rc = self.cross(&axis);
            let perp = rc_ref!(&perp_rc).normalize();
            let perp = rc_ref!(&perp);
            let angle = t * theta;
            let c = angle.cos();
            let s = angle.sin() * self.length();
            return Self::new(
                self.x * c + perp.x * s,
                self.y * c + perp.y * s,
                self.z * c + perp.z * s,
            );
        }
        let sin_theta = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;
        Self::new(
            self.x * a + other.x * b,
            self.y * a + other.y * b,
            self.z * a + other.z * b,
        )
    }

    pub fn reflect(&self, normal: &Self) -> RcVec3 {
        let d = self.dot(normal) * 2.0;
        Self::new(
            self.x - normal.x * d,
            self.y - normal.y * d,
            self.z - normal.z * d,
        )
    }

    pub fn project(&self, other: &Self) -> RcVec3 {
        let other_len_sq = other.length_squared();
        if other_len_sq == 0.0 {
            Self::zero()
        } else {
            let scale = self.dot(other) / other_len_sq;
            Self::new(other.x * scale, other.y * scale, other.z * scale)
        }
    }

    // Coordinate system conversions

    pub fn to_local(&self, mat: &Mat4) -> RcVec3 {
        let inv_rc = mat.inverse();
        let result = rc_ref!(&inv_rc).mul_vec(self);
        result
    }

    pub fn to_world(&self, mat: &Mat4) -> RcVec3 {
        mat.mul_vec(self)
    }

    pub fn to_local_dir(&self, mat: &Mat4) -> RcVec3 {
        let inv_rc = mat.inverse();
        let result = rc_ref!(&inv_rc).mul_dir(self);
        result
    }

    pub fn to_world_dir(&self, mat: &Mat4) -> RcVec3 {
        mat.mul_dir(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::mat4::Mat4;

    fn deref(rc: &RcVec3) -> Vec3 {
        *rc_ref!(rc)
    }

    #[test]
    fn test_arithmetic() {
        let a = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let b = Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        };
        assert_eq!(
            deref(&a.add(&b)),
            Vec3 {
                x: 5.0,
                y: 7.0,
                z: 9.0
            }
        );
        assert_eq!(
            deref(&a.sub(&b)),
            Vec3 {
                x: -3.0,
                y: -3.0,
                z: -3.0
            }
        );
        assert_eq!(
            deref(&a.mul(2.0)),
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
        assert_eq!(
            deref(&a.div(2.0)),
            Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5
            }
        );
        assert_eq!(
            deref(&a.neg()),
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );
    }

    #[test]
    fn test_angle_to() {
        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        assert!((a.angle_to(&b) - 90.0).abs() < 1e-3);
        assert!((a.angle_to(&a) - 0.0).abs() < 1e-3);
        // Zero-length input returns 0 instead of NaN.
        let z = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert_eq!(z.angle_to(&a), 0.0);
    }

    #[test]
    fn test_slerp_antiparallel_rotates_through_perpendicular() {
        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        };
        let mid = deref(&a.slerp(&b, 0.5));
        assert!((mid.length() - 1.0).abs() < 1e-5);
        assert!(mid.x.abs() < 1e-5);
        assert!((mid.z - 1.0).abs() < 1e-5);
        let quarter = deref(&a.slerp(&b, 0.25));
        let expected = (0.5_f32).sqrt();
        assert!((quarter.x - expected).abs() < 1e-5);
        assert!((quarter.z - expected).abs() < 1e-5);
        let end = deref(&a.slerp(&b, 1.0));
        assert!((end.x - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn test_to_local_round_trip_with_to_world() {
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let mat_rc = Mat4::from_translation(&Vec3 {
            x: 5.0,
            y: 0.0,
            z: -1.0,
        });
        let mat = rc_ref!(&mat_rc);
        let world = v.to_world(&mat);
        let back = rc_ref!(&world).to_local(&mat);
        let back = deref(&back);
        assert_eq!(back, v);
    }

    #[test]
    fn test_to_world_dir_ignores_translation() {
        // Direction vectors do not pick up the translation column.
        let dir = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let mat_rc = Mat4::from_translation(&Vec3 {
            x: 100.0,
            y: 100.0,
            z: 100.0,
        });
        let mat = rc_ref!(&mat_rc);
        let r = deref(&dir.to_world_dir(&mat));
        assert_eq!(r.x, 1.0);
        assert_eq!(r.y, 0.0);
        assert_eq!(r.z, 0.0);
    }

    #[test]
    fn test_to_local_dir_round_trip() {
        let dir = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let mat_rc = Mat4::from_translation(&Vec3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });
        let mat = rc_ref!(&mat_rc);
        let world = dir.to_world_dir(&mat);
        let back = rc_ref!(&world).to_local_dir(&mat);
        let back = deref(&back);
        assert_eq!(back, dir);
    }
}
