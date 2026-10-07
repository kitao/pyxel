// Quaternion formulas use conventional component and scalar names.
#![allow(clippy::many_single_char_names)]

use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::vec3::{sum_wide_products, RcVec3, Vec3};

// Component order is (x, y, z, w), with w as the scalar.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

define_rc_type!(RcQuat, Quat);

impl Quat {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> RcQuat {
        new_rc_type!(Quat { x, y, z, w })
    }

    pub fn identity() -> RcQuat {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    // Operators

    pub fn neg(&self) -> RcQuat {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }

    pub fn mul_quat(&self, other: &Self) -> RcQuat {
        new_rc_type!(self.mul_quat_value(other))
    }

    fn mul_quat_value(&self, other: &Self) -> Self {
        // Hamilton product
        let x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        let y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        let z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;
        let w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        Self { x, y, z, w }
    }

    pub fn mul_vec(&self, v: &Vec3) -> RcVec3 {
        // q * v * q^-1 — for unit quaternion (assumed)
        let qx = self.x;
        let qy = self.y;
        let qz = self.z;
        let qw = self.w;

        // t = 2 * (q.xyz x v)
        let tx = 2.0 * (qy * v.z - qz * v.y);
        let ty = 2.0 * (qz * v.x - qx * v.z);
        let tz = 2.0 * (qx * v.y - qy * v.x);

        // result = v + qw * t + q.xyz x t
        let rx = v.x + qw * tx + (qy * tz - qz * ty);
        let ry = v.y + qw * ty + (qz * tx - qx * tz);
        let rz = v.z + qw * tz + (qx * ty - qy * tx);
        if !(rx.is_finite() && ry.is_finite() && rz.is_finite())
            && self.is_finite()
            && v.is_finite()
        {
            let (qx, qy, qz, qw) = (f64::from(qx), f64::from(qy), f64::from(qz), f64::from(qw));
            let (x, y, z) = (f64::from(v.x), f64::from(v.y), f64::from(v.z));
            let (tx, ty, tz) = (
                2.0 * (qy * z - qz * y),
                2.0 * (qz * x - qx * z),
                2.0 * (qx * y - qy * x),
            );
            return Vec3::new(
                (x + qw * tx + (qy * tz - qz * ty)) as f32,
                (y + qw * ty + (qz * tx - qx * tz)) as f32,
                (z + qw * tz + (qx * ty - qy * tx)) as f32,
            );
        }
        Vec3::new(rx, ry, rz)
    }

    // Factories

    pub fn from_axis_angle(axis: &Vec3, deg: f32) -> RcQuat {
        new_rc_type!(Self::from_axis_angle_value(axis, deg))
    }

    fn from_axis_angle_value(axis: &Vec3, deg: f32) -> Self {
        let len_sq = axis.length_squared();
        if let Some(len) = axis.wide_length(len_sq) {
            let half = deg.to_radians() * 0.5;
            let scale = f64::from(half.sin()) / len;
            return Self {
                x: (f64::from(axis.x) * scale) as f32,
                y: (f64::from(axis.y) * scale) as f32,
                z: (f64::from(axis.z) * scale) as f32,
                w: half.cos(),
            };
        }
        let len = len_sq.sqrt();
        if len == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            };
        }
        let half = deg.to_radians() * 0.5;
        let s = half.sin() / len;
        Self {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half.cos(),
        }
    }

    pub fn from_euler(euler: &Vec3) -> RcQuat {
        // XYZ extrinsic: result = Rz * Ry * Rx (applied to vector from right)
        let x_axis = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let y_axis = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let z_axis = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let qx = Self::from_axis_angle_value(&x_axis, euler.x);
        let qy = Self::from_axis_angle_value(&y_axis, euler.y);
        let qz = Self::from_axis_angle_value(&z_axis, euler.z);
        let zy = qz.mul_quat_value(&qy);
        new_rc_type!(zy.mul_quat_value(&qx))
    }

    pub fn from_two_vectors(a: &Vec3, b: &Vec3) -> RcQuat {
        let a_len_sq = a.length_squared();
        let b_len_sq = b.length_squared();
        let len_ab = a_len_sq.sqrt() * b_len_sq.sqrt();
        if (!a_len_sq.is_normal() || !b_len_sq.is_normal() || !len_ab.is_normal())
            && a.is_finite()
            && b.is_finite()
        {
            return Self::from_two_vectors_wide(a, b);
        }
        if len_ab == 0.0 {
            return Self::identity();
        }
        let dot = a.x * b.x + a.y * b.y + a.z * b.z;
        let cos_theta = (dot / len_ab).clamp(-1.0, 1.0);
        if cos_theta < -0.999_999 {
            // Opposite vectors: pick any perpendicular axis and rotate 180°.
            let a_len = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt();
            let axis = if a.x.abs() < 0.9 * a_len {
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

            let perp_x = a.y * axis.z - a.z * axis.y;
            let perp_y = a.z * axis.x - a.x * axis.z;
            let perp_z = a.x * axis.y - a.y * axis.x;
            let plen = (perp_x * perp_x + perp_y * perp_y + perp_z * perp_z).sqrt();
            return Self::new(perp_x / plen, perp_y / plen, perp_z / plen, 0.0);
        }

        // axis = cross(a, b), w = sqrt((|a|^2 * |b|^2)) + dot(a, b), then normalize
        let cross_x = a.y * b.z - a.z * b.y;
        let cross_y = a.z * b.x - a.x * b.z;
        let cross_z = a.x * b.y - a.y * b.x;
        let s = len_ab + dot;
        let len_sq = cross_x * cross_x + cross_y * cross_y + cross_z * cross_z + s * s;
        if !len_sq.is_normal() && a.is_finite() && b.is_finite() {
            return Self::from_two_vectors_wide(a, b);
        }
        let len = len_sq.sqrt();
        Self::new(cross_x / len, cross_y / len, cross_z / len, s / len)
    }

    fn from_two_vectors_wide(a: &Vec3, b: &Vec3) -> RcQuat {
        let (ax, ay, az) = (f64::from(a.x), f64::from(a.y), f64::from(a.z));
        let (bx, by, bz) = (f64::from(b.x), f64::from(b.y), f64::from(b.z));
        let a_len = a.length_f64();
        let len_ab = a_len * b.length_f64();
        if len_ab == 0.0 {
            return Self::identity();
        }
        let dot = ax * bx + ay * by + az * bz;
        if (dot / len_ab).clamp(-1.0, 1.0) < f64::from(-0.999_999_f32) {
            let (x, y, z) = if ax.abs() < f64::from(0.9_f32) * a_len {
                (0.0, az, -ay)
            } else {
                (-az, 0.0, ax)
            };
            let len = (x * x + y * y + z * z).sqrt();
            return Self::new((x / len) as f32, (y / len) as f32, (z / len) as f32, 0.0);
        }
        let (x, y, z) = (ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx);
        let w = len_ab + dot;
        let len = (x * x + y * y + z * z + w * w).sqrt();
        Self::new(
            (x / len) as f32,
            (y / len) as f32,
            (z / len) as f32,
            (w / len) as f32,
        )
    }

    pub fn from_matrix(mat: &Mat4) -> RcQuat {
        mat.rot()
    }

    pub(crate) fn from_matrix_value(mat: &Mat4) -> Self {
        // Shepperd's method: branch on trace / largest diagonal so the
        // extraction divides by the largest quaternion component.
        let m = &mat.data;
        let trace = m[0][0] + m[1][1] + m[2][2];
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            let w = 0.25 * s;
            let x = (m[2][1] - m[1][2]) / s;
            let y = (m[0][2] - m[2][0]) / s;
            let z = (m[1][0] - m[0][1]) / s;
            Self { x, y, z, w }
        } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
            let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
            let w = (m[2][1] - m[1][2]) / s;
            let x = 0.25 * s;
            let y = (m[0][1] + m[1][0]) / s;
            let z = (m[0][2] + m[2][0]) / s;
            Self { x, y, z, w }
        } else if m[1][1] > m[2][2] {
            let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
            let w = (m[0][2] - m[2][0]) / s;
            let x = (m[0][1] + m[1][0]) / s;
            let y = 0.25 * s;
            let z = (m[1][2] + m[2][1]) / s;
            Self { x, y, z, w }
        } else {
            let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
            let w = (m[1][0] - m[0][1]) / s;
            let x = (m[0][2] + m[2][0]) / s;
            let y = (m[1][2] + m[2][1]) / s;
            let z = 0.25 * s;
            Self { x, y, z, w }
        }
    }

    pub fn from_direction(forward: &Vec3, up: &Vec3) -> RcQuat {
        // Align the object's -Z axis with `forward`; +Y follows `up` as
        // closely as the constraint allows. Right-handed.
        let f_len_sq = forward.x * forward.x + forward.y * forward.y + forward.z * forward.z;
        if f_len_sq < 1e-12 {
            return Self::identity();
        }
        let (fx, fy, fz) = if let Some(len) = forward.wide_length(f_len_sq) {
            (
                (f64::from(forward.x) / len) as f32,
                (f64::from(forward.y) / len) as f32,
                (f64::from(forward.z) / len) as f32,
            )
        } else {
            let inv_f = 1.0 / f_len_sq.sqrt();
            (forward.x * inv_f, forward.y * inv_f, forward.z * inv_f)
        };

        // right = forward × up (matches look_at convention, right-handed).
        let rx0 = fy * up.z - fz * up.y;
        let ry0 = fz * up.x - fx * up.z;
        let rz0 = fx * up.y - fy * up.x;
        let r_len_sq = rx0 * rx0 + ry0 * ry0 + rz0 * rz0;
        if r_len_sq < 1e-12 {
            // forward parallel to up — fall back to rotating canonical -Z
            // onto the requested direction.
            return Self::from_two_vectors(
                &Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                &Vec3 {
                    x: fx,
                    y: fy,
                    z: fz,
                },
            );
        }
        let right = Vec3 {
            x: rx0,
            y: ry0,
            z: rz0,
        };
        let f = Vec3 {
            x: fx,
            y: fy,
            z: fz,
        };
        let (rx, ry, rz) = if !right.is_finite() && f.is_finite() && up.is_finite() {
            let right = f.cross_normalized_wide(up);
            (right.x, right.y, right.z)
        } else if let Some(len) = right.wide_length(r_len_sq) {
            (
                (f64::from(rx0) / len) as f32,
                (f64::from(ry0) / len) as f32,
                (f64::from(rz0) / len) as f32,
            )
        } else {
            let inv_r = 1.0 / r_len_sq.sqrt();
            (rx0 * inv_r, ry0 * inv_r, rz0 * inv_r)
        };

        // up' = right × forward (so up' aligns with the up hint as
        // closely as the constraint allows).
        let ux = ry * fz - rz * fy;
        let uy = rz * fx - rx * fz;
        let uz = rx * fy - ry * fx;
        let mat = Mat4 {
            data: [
                [rx, ux, -fx, 0.0],
                [ry, uy, -fy, 0.0],
                [rz, uz, -fz, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        let q = Self::from_matrix_value(&mat);
        Self::new(q.x, q.y, q.z, q.w)
    }

    // Unary operations

    pub fn conjugate(&self) -> RcQuat {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn inverse(&self) -> RcQuat {
        let len_sq = self.length_squared();
        if len_sq == 0.0 {
            return Self::identity();
        }
        Self::new(
            -self.x / len_sq,
            -self.y / len_sq,
            -self.z / len_sq,
            self.w / len_sq,
        )
    }

    pub fn normalize(&self) -> RcQuat {
        let q = self.normalize_value();
        Self::new(q.x, q.y, q.z, q.w)
    }

    pub(crate) fn normalize_value(&self) -> Self {
        let len_sq = self.length_squared();
        if let Some(len) = self.wide_length(len_sq) {
            return Self {
                x: (f64::from(self.x) / len) as f32,
                y: (f64::from(self.y) / len) as f32,
                z: (f64::from(self.z) / len) as f32,
                w: (f64::from(self.w) / len) as f32,
            };
        }
        let len = len_sq.sqrt();
        if len == 0.0 {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            };
        }
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
            w: self.w / len,
        }
    }

    pub fn length(&self) -> f32 {
        let len_sq = self.length_squared();
        self.wide_length(len_sq)
            .map_or_else(|| len_sq.sqrt(), |len| len as f32)
    }

    pub(crate) fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    // Match Vec3's finite-range fallback without changing four-component storage.
    fn wide_length(&self, len_sq: f32) -> Option<f64> {
        if len_sq.is_normal()
            || (self.x == 0.0 && self.y == 0.0 && self.z == 0.0 && self.w == 0.0)
            || !self.is_finite()
        {
            return None;
        }
        let (x, y, z, w) = (
            f64::from(self.x),
            f64::from(self.y),
            f64::from(self.z),
            f64::from(self.w),
        );
        Some((x * x + y * y + z * z + w * w).sqrt())
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    // Binary operations

    pub fn dot(&self, other: &Self) -> f32 {
        let dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        if !dot.is_finite() && self.is_finite() && other.is_finite() {
            return sum_wide_products([
                (self.x, other.x),
                (self.y, other.y),
                (self.z, other.z),
                (self.w, other.w),
            ]) as f32;
        }
        dot
    }

    pub fn angle_to(&self, other: &Self) -> f32 {
        let dot = self.dot(other).abs().clamp(0.0, 1.0);
        2.0 * dot.acos().to_degrees()
    }

    // Conversions

    pub fn to_matrix(&self) -> RcMat4 {
        Mat4::from_rows(self.matrix_value().data)
    }

    pub(crate) fn matrix_value(&self) -> Mat4 {
        let q = self.normalize_value();
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;

        Mat4 {
            data: [
                [1.0 - 2.0 * (yy + zz), 2.0 * (xy - wz), 2.0 * (xz + wy), 0.0],
                [2.0 * (xy + wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - wx), 0.0],
                [2.0 * (xz - wy), 2.0 * (yz + wx), 1.0 - 2.0 * (xx + yy), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn to_euler(&self) -> RcVec3 {
        // XYZ extrinsic (matches from_euler).
        let q = self.normalize_value();
        let r00 = 1.0 - 2.0 * (q.y * q.y + q.z * q.z);
        let r10 = 2.0 * (q.x * q.y + q.w * q.z);
        let r11 = 1.0 - 2.0 * (q.x * q.x + q.z * q.z);
        let r12 = 2.0 * (q.y * q.z - q.w * q.x);
        let r20 = 2.0 * (q.x * q.z - q.w * q.y);
        let r21 = 2.0 * (q.y * q.z + q.w * q.x);
        let r22 = 1.0 - 2.0 * (q.x * q.x + q.y * q.y);

        let sy = -r20;
        let sy_clamped = sy.clamp(-1.0, 1.0);
        let (rx, ry, rz);
        if sy.abs() < 0.999_999 {
            ry = sy_clamped.asin();
            rx = r21.atan2(r22);
            rz = r10.atan2(r00);
        } else {
            // At gimbal lock, fix Z to zero and recover the coupled rotation as X.
            ry = sy_clamped.asin();
            rx = (-r12).atan2(r11);
            rz = 0.0;
        }

        Vec3::new(rx.to_degrees(), ry.to_degrees(), rz.to_degrees())
    }

    pub fn to_axis_angle(&self) -> (RcVec3, f32) {
        let q = self.normalize_value();
        let w = q.w.clamp(-1.0, 1.0);
        let half = w.acos();
        let sin_half = (1.0 - w * w).sqrt();
        let axis = if sin_half < 1e-6 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            Vec3::new(q.x / sin_half, q.y / sin_half, q.z / sin_half)
        };
        (axis, (2.0 * half).to_degrees())
    }

    // Interpolation

    pub fn slerp(&self, other: &Self, t: f32) -> RcQuat {
        let q = self.slerp_value(other, t);
        Self::new(q.x, q.y, q.z, q.w)
    }

    pub(crate) fn slerp_value(&self, other: &Self, t: f32) -> Self {
        let mut cos_theta = self.dot(other);
        let (other_x, other_y, other_z, other_w);
        // q and -q encode the same rotation; choose the shorter interpolation arc.
        if cos_theta < 0.0 {
            cos_theta = -cos_theta;
            other_x = -other.x;
            other_y = -other.y;
            other_z = -other.z;
            other_w = -other.w;
        } else {
            other_x = other.x;
            other_y = other.y;
            other_z = other.z;
            other_w = other.w;
        }

        if cos_theta > 0.9995 {
            // Avoid division by sin(theta) near identical rotations.
            let x = self.x + t * (other_x - self.x);
            let y = self.y + t * (other_y - self.y);
            let z = self.z + t * (other_z - self.z);
            let w = self.w + t * (other_w - self.w);
            let len = (x * x + y * y + z * z + w * w).sqrt();
            return Self {
                x: x / len,
                y: y / len,
                z: z / len,
                w: w / len,
            };
        }

        let theta = cos_theta.acos();
        let sin_theta = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;
        Self {
            x: self.x * a + other_x * b,
            y: self.y * a + other_y * b,
            z: self.z * a + other_z * b,
            w: self.w * a + other_w * b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_retains_wide_cancellation_residual() {
        for large in [1e20, 1e30, 3e38] {
            let a = Quat {
                x: large,
                y: 1.0,
                z: -large,
                w: 2.0,
            };
            let b = Quat {
                x: large,
                y: 1.0,
                z: large,
                w: 1.0,
            };
            // Equal stored products cancel, leaving 1 + 2.
            assert_eq!(a.dot(&b), 3.0);
        }
    }

    #[test]
    fn test_dot_keeps_ordinary_f32_cancellation() {
        let large = 1e10;
        let a = Quat {
            x: large,
            y: 1.0,
            z: -large,
            w: 2.0,
        };
        let b = Quat {
            x: large,
            y: 1.0,
            z: large,
            w: 1.0,
        };
        // The middle 1 is lost before cancellation; the last 2 remains.
        assert_eq!(a.dot(&b).to_bits(), 2.0_f32.to_bits());
    }

    fn deref(rc: &RcQuat) -> Quat {
        *rc_ref!(rc)
    }

    fn deref_v(rc: &RcVec3) -> Vec3 {
        *rc_ref!(rc)
    }

    // 1e-4 allows a few f32 ulps at the Euler angles used here, up to 180 degrees.
    fn approx_eq_v(a: &Vec3, b: &Vec3) -> bool {
        (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4 && (a.z - b.z).abs() < 1e-4
    }

    #[test]
    fn test_to_euler_round_trip() {
        let in_euler = Vec3 {
            x: 0.0,
            y: 30.0,
            z: 0.0,
        };
        let q = Quat::from_euler(&in_euler);
        let out_euler = deref_v(&rc_ref!(&q).to_euler());
        assert!(approx_eq_v(&out_euler, &in_euler));
    }

    #[test]
    fn test_from_two_vectors_anti_parallel() {
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

        let q = Quat::from_two_vectors(&a, &b);
        let r = deref_v(&rc_ref!(&q).mul_vec(&a));
        assert!(approx_eq_v(&r, &b));
    }

    #[test]
    fn test_from_two_vectors_zero_input_returns_identity() {
        let a = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let b = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let q = deref(&Quat::from_two_vectors(&a, &b));
        let id = deref(&Quat::identity());
        assert_eq!(q, id);
    }

    #[test]
    fn test_from_two_vectors_same_vector_returns_identity() {
        let a = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let q = deref(&Quat::from_two_vectors(&a, &a));
        let id = deref(&Quat::identity());
        assert_eq!(q, id);
    }

    #[test]
    fn test_from_two_vectors_preserves_small_turns() {
        let from = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        for y in [0.00001, 0.0001, 0.001] {
            let target = Vec3 { x: 1.0, y, z: 0.0 };
            let turn = Quat::from_two_vectors(&from, &target);
            let actual = deref_v(&rc_ref!(&turn).mul_vec(&from));
            let length = (1.0 + f64::from(y).powi(2)).sqrt();
            let expected_y = f64::from(y) / length;
            // Relative 2e-6 allows accumulated f32 rounding while detecting loss of the tiny turn.
            assert!((f64::from(actual.y) / expected_y - 1.0).abs() < 2e-6);
            assert!((f64::from(actual.x) - 1.0 / length).abs() < 2e-6);
            assert_eq!(actual.z, 0.0);
        }
    }

    #[test]
    fn test_to_euler_gimbal_lock_positive_pitch() {
        let in_euler = Vec3 {
            x: 0.0,
            y: 90.0,
            z: 0.0,
        };
        let q = Quat::from_euler(&in_euler);
        let qn = deref(&rc_ref!(&q).normalize());
        let sy = -2.0 * (qn.x * qn.z - qn.w * qn.y);
        assert!(sy.abs() >= 0.999_999);

        let out_euler = deref_v(&rc_ref!(&q).to_euler());
        assert!(approx_eq_v(
            &out_euler,
            &Vec3 {
                x: 0.0,
                y: 90.0,
                z: 0.0,
            }
        ));
    }

    #[test]
    fn test_slerp_nearly_identical_rotations_preserves_midpoint() {
        // This angle gap takes the normalized linear-interpolation branch.
        let axis = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let a = Quat::from_axis_angle(&axis, 10.0);
        let b = Quat::from_axis_angle(&axis, 11.0);

        let mid = deref(&rc_ref!(&a).slerp(&rc_ref!(&b), 0.5));
        let len = mid.x * mid.x + mid.y * mid.y + mid.z * mid.z + mid.w * mid.w;
        assert!((len - 1.0).abs() < 1e-4);

        let expected = Quat::from_axis_angle(&axis, 10.5);
        let probe = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r_mid = deref_v(&mid.mul_vec(&probe));
        let r_expected = deref_v(&rc_ref!(&expected).mul_vec(&probe));
        assert!(approx_eq_v(&r_mid, &r_expected));
    }

    #[test]
    fn test_slerp_negate_takes_short_path() {
        // a and -a represent the same orientation.
        let a = Quat::from_axis_angle(
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            30.0,
        );
        let a_val = *rc_ref!(&a);
        let neg = Quat::new(-a_val.x, -a_val.y, -a_val.z, -a_val.w);

        let mid = rc_ref!(&a).slerp(&rc_ref!(&neg), 0.5);
        let probe = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r_a = deref_v(&rc_ref!(&a).mul_vec(&probe));
        let r_mid = deref_v(&rc_ref!(&mid).mul_vec(&probe));
        assert!(approx_eq_v(&r_a, &r_mid));
    }

    #[test]
    fn test_inverse_zero_returns_identity() {
        let q = Quat::new(0.0, 0.0, 0.0, 0.0);
        let inv = deref(&rc_ref!(&q).inverse());
        let id = deref(&Quat::identity());
        assert_eq!(inv, id);
    }

    #[test]
    fn test_from_direction_parallel_to_up_uses_fallback() {
        let forward = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let q = Quat::from_direction(&forward, &up);
        let local_forward = Vec3 {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };
        let r = deref_v(&rc_ref!(&q).mul_vec(&local_forward));
        assert!(approx_eq_v(&r, &forward));
    }
}
