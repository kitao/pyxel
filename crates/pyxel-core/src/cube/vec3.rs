use crate::cube::mat4::Mat4;

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
        let dot = self.x * other.x + self.y * other.y + self.z * other.z;
        if !dot.is_finite() && self.is_finite() && other.is_finite() {
            return self.dot_f64(other) as f32;
        }
        dot
    }

    fn dot_f64(&self, other: &Self) -> f64 {
        f64::from(self.x) * f64::from(other.x)
            + f64::from(self.y) * f64::from(other.y)
            + f64::from(self.z) * f64::from(other.z)
    }

    pub fn cross(&self, other: &Self) -> RcVec3 {
        let (x, y, z) = (
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
        if !(x.is_finite() && y.is_finite() && z.is_finite())
            && self.is_finite()
            && other.is_finite()
        {
            let (x, y, z) = self.cross_f64(other);
            return Self::new(x as f32, y as f32, z as f32);
        }
        Self::new(x, y, z)
    }

    fn cross_f64(&self, other: &Self) -> (f64, f64, f64) {
        let (x, y, z) = (f64::from(self.x), f64::from(self.y), f64::from(self.z));
        let (ox, oy, oz) = (f64::from(other.x), f64::from(other.y), f64::from(other.z));
        (y * oz - z * oy, z * ox - x * oz, x * oy - y * ox)
    }

    // Basis consumers must normalize a wide cross product before narrowing it.
    pub(crate) fn cross_normalized_wide(&self, other: &Self) -> Self {
        let (x, y, z) = self.cross_f64(other);
        let len = (x * x + y * y + z * z).sqrt();
        if len == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }
        Self {
            x: (x / len) as f32,
            y: (y / len) as f32,
            z: (z / len) as f32,
        }
    }

    pub fn length(&self) -> f32 {
        let len_sq = self.length_squared();
        self.wide_length(len_sq)
            .map_or_else(|| len_sq.sqrt(), |len| len as f32)
    }

    // Finite components can lose range when squared in f32. Keep ordinary
    // rounding and true-zero/nonfinite behavior at each consuming operation.
    pub(crate) fn wide_length(&self, len_sq: f32) -> Option<f64> {
        (!len_sq.is_normal()
            && (self.x != 0.0 || self.y != 0.0 || self.z != 0.0)
            && self.is_finite())
        .then(|| self.length_f64())
    }

    pub(crate) fn length_f64(&self) -> f64 {
        let (x, y, z) = (f64::from(self.x), f64::from(self.y), f64::from(self.z));
        (x * x + y * y + z * z).sqrt()
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn distance_to(&self, other: &Self) -> f32 {
        let distance_sq = self.distance_squared_to(other);
        if !distance_sq.is_normal() && self != other && self.is_finite() && other.is_finite() {
            let dx = f64::from(self.x) - f64::from(other.x);
            let dy = f64::from(self.y) - f64::from(other.y);
            let dz = f64::from(self.z) - f64::from(other.z);
            return (dx * dx + dy * dy + dz * dz).sqrt() as f32;
        }
        distance_sq.sqrt()
    }

    pub fn distance_squared_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }

    pub fn angle_to(&self, other: &Self) -> f32 {
        let self_len_sq = self.length_squared();
        let other_len_sq = other.length_squared();
        let len_product = self_len_sq.sqrt() * other_len_sq.sqrt();
        if (!self_len_sq.is_normal() || !other_len_sq.is_normal() || !len_product.is_normal())
            && self.is_finite()
            && other.is_finite()
        {
            let len_product = self.length_f64() * other.length_f64();
            if len_product == 0.0 {
                return 0.0;
            }
            let dot = self.dot_f64(other);
            return (dot / len_product).clamp(-1.0, 1.0).acos().to_degrees() as f32;
        }
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
        let len_sq = self.length_squared();
        if let Some(len) = self.wide_length(len_sq) {
            return Self::new(
                (f64::from(self.x) / len) as f32,
                (f64::from(self.y) / len) as f32,
                (f64::from(self.z) / len) as f32,
            );
        }
        let len = len_sq.sqrt();
        if len == 0.0 {
            Self::zero()
        } else {
            Self::new(self.x / len, self.y / len, self.z / len)
        }
    }

    pub fn clamp_length(&self, max_length: f32) -> RcVec3 {
        let len_sq = self.length_squared();
        if max_length.is_finite() {
            if let Some(len) = self.wide_length(len_sq) {
                if len > f64::from(max_length) {
                    let scale = f64::from(max_length) / len;
                    return Self::new(
                        (f64::from(self.x) * scale) as f32,
                        (f64::from(self.y) * scale) as f32,
                        (f64::from(self.z) * scale) as f32,
                    );
                }
                return Self::new(self.x, self.y, self.z);
            }
        }
        let len = len_sq.sqrt();
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
        let value = self.lerp_value(other, t);
        Self::new(value.x, value.y, value.z)
    }

    pub(crate) fn lerp_value(&self, other: &Self, t: f32) -> Self {
        let value = Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            z: self.z + (other.z - self.z) * t,
        };
        if !value.is_finite() && self.is_finite() && other.is_finite() && t.is_finite() {
            let t = f64::from(t);
            return Self {
                x: (f64::from(self.x) + (f64::from(other.x) - f64::from(self.x)) * t) as f32,
                y: (f64::from(self.y) + (f64::from(other.y) - f64::from(self.y)) * t) as f32,
                z: (f64::from(self.z) + (f64::from(other.z) - f64::from(self.z)) * t) as f32,
            };
        }
        value
    }

    pub fn slerp(&self, other: &Self, t: f32) -> RcVec3 {
        if t == 0.0 {
            return Self::new(self.x, self.y, self.z);
        }
        if t == 1.0 {
            return Self::new(other.x, other.y, other.z);
        }

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
            let cross = Self {
                x: self.y * other.z - self.z * other.y,
                y: self.z * other.x - self.x * other.z,
                z: self.x * other.y - self.y * other.x,
            };
            let cross_len = cross.length();
            let self_len = self.length();

            let (tangent, theta) = if cross_len > 0.0 {
                // Near-opposite vectors still define a unique interpolation plane.
                (
                    Self {
                        x: cross.y * self.z - cross.z * self.y,
                        y: cross.z * self.x - cross.x * self.z,
                        z: cross.x * self.y - cross.y * self.x,
                    },
                    (cross_len / len_product).atan2(dot),
                )
            } else {
                // Exactly opposite vectors admit any perpendicular direction.
                let tangent = if self.x.abs() < 0.9 * self_len {
                    Self {
                        x: 0.0,
                        y: self.z,
                        z: -self.y,
                    }
                } else {
                    Self {
                        x: -self.z,
                        y: 0.0,
                        z: self.x,
                    }
                };
                (tangent, theta)
            };

            let angle = t * theta;
            let c = angle.cos();
            let s = angle.sin() * self_len / tangent.length();
            return Self::new(
                self.x * c + tangent.x * s,
                self.y * c + tangent.y * s,
                self.z * c + tangent.z * s,
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
        let value = Self {
            x: self.x - normal.x * d,
            y: self.y - normal.y * d,
            z: self.z - normal.z * d,
        };
        if !value.is_finite() && self.is_finite() && normal.is_finite() {
            let d = self.dot_f64(normal) * 2.0;
            return Self::new(
                (f64::from(self.x) - f64::from(normal.x) * d) as f32,
                (f64::from(self.y) - f64::from(normal.y) * d) as f32,
                (f64::from(self.z) - f64::from(normal.z) * d) as f32,
            );
        }
        Self::new(value.x, value.y, value.z)
    }

    pub fn project(&self, other: &Self) -> RcVec3 {
        let other_len_sq = other.length_squared();
        let dot = self.dot(other);
        let scale = dot / other_len_sq;
        let dot_underflows = !dot.is_normal()
            && (dot != 0.0
                || [(self.x, other.x), (self.y, other.y), (self.z, other.z)]
                    .iter()
                    .any(|&(a, b)| a != 0.0 && b != 0.0 && !(a * b).is_normal()));
        if (!other_len_sq.is_normal() || !dot.is_finite() || !scale.is_finite() || dot_underflows)
            && self.is_finite()
            && other.is_finite()
            && (other.x != 0.0 || other.y != 0.0 || other.z != 0.0)
        {
            let (x, y, z) = (f64::from(other.x), f64::from(other.y), f64::from(other.z));
            let dot = self.dot_f64(other);
            let scale = dot / (x * x + y * y + z * z);
            return Self::new((x * scale) as f32, (y * scale) as f32, (z * scale) as f32);
        }
        if other_len_sq == 0.0 {
            Self::zero()
        } else {
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

// Finite f32 products fit exactly in f64. TwoSum retains cancellation residuals
// before rounding their sum, including a small offset between large products.
pub(super) fn sum_wide_products(terms: [(f32, f32); 4]) -> f64 {
    let mut expansion = [0.0_f64; 4];
    let mut len = 0;
    for (a, b) in terms {
        let mut sum = f64::from(a) * f64::from(b);
        let mut next_len = 0;
        for index in 0..len {
            let value = expansion[index];
            let combined = sum + value;
            let value_part = combined - sum;
            let error = (sum - (combined - value_part)) + (value - value_part);
            if error != 0.0 {
                expansion[next_len] = error;
                next_len += 1;
            }
            sum = combined;
        }
        expansion[next_len] = sum;
        len = next_len + 1;
    }
    expansion[..len].iter().sum()
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

    #[test]
    fn test_slerp_near_opposite_preserves_target_plane_and_endpoints() {
        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let delta = 0.0005_f32;
        let b = Vec3 {
            x: -delta.cos(),
            y: delta.sin(),
            z: 0.0,
        };

        let mid = a.slerp(&b, 0.5);
        let mid = rc_ref!(&mid);
        // Half of the XY angle PI-delta is (sin(delta/2), cos(delta/2), 0).
        assert!((mid.x - (delta * 0.5).sin()).abs() < 1e-6);
        assert!((mid.y - (delta * 0.5).cos()).abs() < 1e-6);
        assert_eq!(mid.z, 0.0);
        assert_eq!(*rc_ref!(&a.slerp(&b, 0.0)), a);
        assert_eq!(*rc_ref!(&a.slerp(&b, 1.0)), b);
    }
}
