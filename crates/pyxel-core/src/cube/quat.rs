// Quaternion math uses the standard (x, y, z, w) and intermediate scalar
// names (s, t, m, ...) by convention. Renaming them obscures the algebra.
#![allow(clippy::many_single_char_names)]

use crate::cube::mat4::{Mat4, RcMat4};
use crate::cube::vec3::{RcVec3, Vec3};

// Immutable quaternion. Component order is (x, y, z, w) with w as the scalar.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

define_rc_type!(RcQuat, Quat);

impl Quat {
    // Constructor

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
        // Hamilton product
        let x = self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y;
        let y = self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x;
        let z = self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w;
        let w = self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z;
        Self::new(x, y, z, w)
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
        Vec3::new(rx, ry, rz)
    }

    // Class-method factories

    pub fn from_axis_angle(axis: &Vec3, deg: f32) -> RcQuat {
        let len = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if len == 0.0 {
            return Self::identity();
        }
        let half = deg.to_radians() * 0.5;
        let s = half.sin() / len;
        Self::new(axis.x * s, axis.y * s, axis.z * s, half.cos())
    }

    pub fn from_euler(rot: &Vec3) -> RcQuat {
        // XYZ extrinsic: result = Rz * Ry * Rx (applied to vector from right)
        let x_axis = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let y_axis = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let z_axis = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
        let qx = Self::from_axis_angle(&x_axis, rot.x);
        let qy = Self::from_axis_angle(&y_axis, rot.y);
        let qz = Self::from_axis_angle(&z_axis, rot.z);
        let zy = rc_ref!(&qz).mul_quat(rc_ref!(&qy));
        rc_ref!(&zy).mul_quat(rc_ref!(&qx))
    }

    pub fn from_two_vectors(a: &Vec3, b: &Vec3) -> RcQuat {
        let len_ab = (a.x * a.x + a.y * a.y + a.z * a.z).sqrt()
            * (b.x * b.x + b.y * b.y + b.z * b.z).sqrt();
        if len_ab == 0.0 {
            return Self::identity();
        }
        let dot = a.x * b.x + a.y * b.y + a.z * b.z;
        let cos_theta = (dot / len_ab).clamp(-1.0, 1.0);
        if cos_theta > 0.999_999 {
            return Self::identity();
        }
        if cos_theta < -0.999_999 {
            // Opposite vectors: pick any perpendicular axis and rotate 180 deg
            let axis = if a.x.abs() < 0.9 {
                Vec3 { x: 1.0, y: 0.0, z: 0.0 }
            } else {
                Vec3 { x: 0.0, y: 1.0, z: 0.0 }
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
        let len = (cross_x * cross_x + cross_y * cross_y + cross_z * cross_z + s * s).sqrt();
        Self::new(cross_x / len, cross_y / len, cross_z / len, s / len)
    }

    pub fn from_matrix(mat: &Mat4) -> RcQuat {
        let m = &mat.data;
        let trace = m[0][0] + m[1][1] + m[2][2];
        if trace > 0.0 {
            let s = (trace + 1.0).sqrt() * 2.0;
            let w = 0.25 * s;
            let x = (m[2][1] - m[1][2]) / s;
            let y = (m[0][2] - m[2][0]) / s;
            let z = (m[1][0] - m[0][1]) / s;
            Self::new(x, y, z, w)
        } else if m[0][0] > m[1][1] && m[0][0] > m[2][2] {
            let s = (1.0 + m[0][0] - m[1][1] - m[2][2]).sqrt() * 2.0;
            let w = (m[2][1] - m[1][2]) / s;
            let x = 0.25 * s;
            let y = (m[0][1] + m[1][0]) / s;
            let z = (m[0][2] + m[2][0]) / s;
            Self::new(x, y, z, w)
        } else if m[1][1] > m[2][2] {
            let s = (1.0 + m[1][1] - m[0][0] - m[2][2]).sqrt() * 2.0;
            let w = (m[0][2] - m[2][0]) / s;
            let x = (m[0][1] + m[1][0]) / s;
            let y = 0.25 * s;
            let z = (m[1][2] + m[2][1]) / s;
            Self::new(x, y, z, w)
        } else {
            let s = (1.0 + m[2][2] - m[0][0] - m[1][1]).sqrt() * 2.0;
            let w = (m[1][0] - m[0][1]) / s;
            let x = (m[0][2] + m[2][0]) / s;
            let y = (m[1][2] + m[2][1]) / s;
            let z = 0.25 * s;
            Self::new(x, y, z, w)
        }
    }

    // Unary

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
        let len = self.length();
        if len == 0.0 {
            return Self::identity();
        }
        Self::new(self.x / len, self.y / len, self.z / len, self.w / len)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    // Binary

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn angle_to(&self, other: &Self) -> f32 {
        let dot = self.dot(other).abs().clamp(0.0, 1.0);
        2.0 * dot.acos().to_degrees()
    }

    // Conversions

    pub fn to_matrix(&self) -> RcMat4 {
        let q = self.normalize();
        let q = rc_ref!(&q);
        let xx = q.x * q.x;
        let yy = q.y * q.y;
        let zz = q.z * q.z;
        let xy = q.x * q.y;
        let xz = q.x * q.z;
        let yz = q.y * q.z;
        let wx = q.w * q.x;
        let wy = q.w * q.y;
        let wz = q.w * q.z;
        Mat4::from_rows([
            [
                1.0 - 2.0 * (yy + zz),
                2.0 * (xy - wz),
                2.0 * (xz + wy),
                0.0,
            ],
            [
                2.0 * (xy + wz),
                1.0 - 2.0 * (xx + zz),
                2.0 * (yz - wx),
                0.0,
            ],
            [
                2.0 * (xz - wy),
                2.0 * (yz + wx),
                1.0 - 2.0 * (xx + yy),
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn to_euler(&self) -> RcVec3 {
        let m = self.to_matrix();
        rc_ref!(&m).rot()
    }

    pub fn to_axis_angle(&self) -> (RcVec3, f32) {
        let q = self.normalize();
        let q = rc_ref!(&q);
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
        let mut cos_theta = self.dot(other);
        let (other_x, other_y, other_z, other_w);
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
            // Linear interpolation, then normalize
            let x = self.x + t * (other_x - self.x);
            let y = self.y + t * (other_y - self.y);
            let z = self.z + t * (other_z - self.z);
            let w = self.w + t * (other_w - self.w);
            let len = (x * x + y * y + z * z + w * w).sqrt();
            return Self::new(x / len, y / len, z / len, w / len);
        }
        let theta = cos_theta.acos();
        let sin_theta = theta.sin();
        let a = ((1.0 - t) * theta).sin() / sin_theta;
        let b = (t * theta).sin() / sin_theta;
        Self::new(
            self.x * a + other_x * b,
            self.y * a + other_y * b,
            self.z * a + other_z * b,
            self.w * a + other_w * b,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deref(rc: &RcQuat) -> Quat {
        *rc_ref!(rc)
    }

    fn deref_v(rc: &RcVec3) -> Vec3 {
        *rc_ref!(rc)
    }

    fn approx_eq_q(a: &Quat, b: &Quat) -> bool {
        (a.x - b.x).abs() < 1e-4
            && (a.y - b.y).abs() < 1e-4
            && (a.z - b.z).abs() < 1e-4
            && (a.w - b.w).abs() < 1e-4
    }

    fn approx_eq_v(a: &Vec3, b: &Vec3) -> bool {
        (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4 && (a.z - b.z).abs() < 1e-4
    }

    #[test]
    fn test_identity() {
        let q = deref(&Quat::identity());
        assert_eq!(q, Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 });
    }

    #[test]
    fn test_constructor() {
        let q = deref(&Quat::new(1.0, 2.0, 3.0, 4.0));
        assert_eq!(q.x, 1.0);
        assert_eq!(q.y, 2.0);
        assert_eq!(q.z, 3.0);
        assert_eq!(q.w, 4.0);
    }

    #[test]
    fn test_from_axis_angle_y_90() {
        let axis = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let q = deref(&Quat::from_axis_angle(&axis, 90.0));
        let expected = Quat { x: 0.0, y: (45.0_f32).to_radians().sin(), z: 0.0, w: (45.0_f32).to_radians().cos() };
        assert!(approx_eq_q(&q, &expected));
    }

    #[test]
    fn test_mul_vec_y_90() {
        let axis = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let q = Quat::from_axis_angle(&axis, 90.0);
        let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let r = deref_v(&rc_ref!(&q).mul_vec(&v));
        // (1, 0, 0) rotated 90 deg around Y-axis (right-handed) -> (0, 0, -1)
        assert!(approx_eq_v(&r, &Vec3 { x: 0.0, y: 0.0, z: -1.0 }));
    }

    #[test]
    fn test_mul_quat_identity() {
        let q = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 45.0);
        let i = Quat::identity();
        let r = deref(&rc_ref!(&q).mul_quat(rc_ref!(&i)));
        let q_val = deref(&q);
        assert!(approx_eq_q(&r, &q_val));
    }

    #[test]
    fn test_conjugate() {
        let q = Quat { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let c = deref(&q.conjugate());
        assert_eq!(c, Quat { x: -1.0, y: -2.0, z: -3.0, w: 4.0 });
    }

    #[test]
    fn test_inverse_unit() {
        let q = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 60.0);
        let q_ref = rc_ref!(&q);
        let inv = q_ref.inverse();
        let combined = q_ref.mul_quat(rc_ref!(&inv));
        let i = deref(&Quat::identity());
        assert!(approx_eq_q(rc_ref!(&combined), &i));
    }

    #[test]
    fn test_normalize() {
        let q = Quat { x: 2.0, y: 0.0, z: 0.0, w: 0.0 };
        let n = deref(&q.normalize());
        assert!((n.length() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_length() {
        let q = Quat { x: 1.0, y: 2.0, z: 2.0, w: 4.0 };
        assert!((q.length() - 5.0).abs() < 1e-5);
        assert_eq!(q.length_squared(), 25.0);
    }

    #[test]
    fn test_dot() {
        let a = Quat { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let b = Quat { x: 5.0, y: 6.0, z: 7.0, w: 8.0 };
        assert_eq!(a.dot(&b), 70.0);
    }

    #[test]
    fn test_angle_to_identity() {
        let i = Quat { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        assert!((i.angle_to(&i) - 0.0).abs() < 1e-3);
    }

    #[test]
    fn test_to_matrix_round_trip() {
        let q = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 30.0);
        let m = rc_ref!(&q).to_matrix();
        let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let v_q = deref_v(&rc_ref!(&q).mul_vec(&v));
        let v_m = deref_v(&rc_ref!(&m).mul_vec(&v));
        assert!(approx_eq_v(&v_q, &v_m));
    }

    #[test]
    fn test_to_axis_angle_round_trip() {
        let axis_in = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let q = Quat::from_axis_angle(&axis_in, 60.0);
        let (axis_out, deg_out) = rc_ref!(&q).to_axis_angle();
        assert!((deg_out - 60.0).abs() < 1e-3);
        assert!(approx_eq_v(rc_ref!(&axis_out), &axis_in));
    }

    #[test]
    fn test_slerp_endpoints() {
        let a = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 0.0);
        let b = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 90.0);
        let s0 = deref(&rc_ref!(&a).slerp(rc_ref!(&b), 0.0));
        let s1 = deref(&rc_ref!(&a).slerp(rc_ref!(&b), 1.0));
        assert!(approx_eq_q(&s0, rc_ref!(&a)));
        assert!(approx_eq_q(&s1, rc_ref!(&b)));
    }

    #[test]
    fn test_from_two_vectors_unit() {
        let a = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let b = Vec3 { x: 0.0, y: 1.0, z: 0.0 };
        let q = Quat::from_two_vectors(&a, &b);
        let r = deref_v(&rc_ref!(&q).mul_vec(&a));
        assert!(approx_eq_v(&r, &b));
    }

    #[test]
    fn test_from_matrix_round_trip() {
        let q = Quat::from_axis_angle(&Vec3 { x: 0.0, y: 1.0, z: 0.0 }, 30.0);
        let m = rc_ref!(&q).to_matrix();
        let q2 = Quat::from_matrix(rc_ref!(&m));
        let v = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
        let v1 = deref_v(&rc_ref!(&q).mul_vec(&v));
        let v2 = deref_v(&rc_ref!(&q2).mul_vec(&v));
        assert!(approx_eq_v(&v1, &v2));
    }
}
