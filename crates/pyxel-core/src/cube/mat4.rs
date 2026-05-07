// Math primitives follow the standard linear-algebra notation: i/j/k for
// matrix indices and c/s for cos/sin. Forcing iterator-based matrix code or
// renaming geometric components hurts readability without clarifying intent.
#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]

use crate::cube::quat::Quat;
use crate::cube::vec3::{RcVec3, Vec3};

// Immutable 4x4 matrix. Internal storage is row-major: data[row][col].
// Mutate methods return new RcMat4.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

define_rc_type!(RcMat4, Mat4);

impl Mat4 {
    // Constructor

    pub fn from_rows(data: [[f32; 4]; 4]) -> RcMat4 {
        new_rc_type!(Mat4 { data })
    }

    pub fn identity() -> RcMat4 {
        Self::from_rows([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // Element access

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row][col]
    }

    // Decomposed view (assumes affine T * R(XYZ extrinsic) * S)

    pub fn pos(&self) -> RcVec3 {
        Vec3::new(self.data[0][3], self.data[1][3], self.data[2][3])
    }

    pub fn scale_vec(&self) -> RcVec3 {
        let sx =
            (self.data[0][0].powi(2) + self.data[1][0].powi(2) + self.data[2][0].powi(2)).sqrt();
        let sy =
            (self.data[0][1].powi(2) + self.data[1][1].powi(2) + self.data[2][1].powi(2)).sqrt();
        let sz =
            (self.data[0][2].powi(2) + self.data[1][2].powi(2) + self.data[2][2].powi(2)).sqrt();
        Vec3::new(sx, sy, sz)
    }

    pub fn rot(&self) -> RcVec3 {
        // Extract Euler XYZ extrinsic from rotation submatrix after removing scale.
        let scale = *rc_ref!(&self.scale_vec());
        let r00 = self.data[0][0] / scale.x;
        let r10 = self.data[1][0] / scale.x;
        let r11 = self.data[1][1] / scale.y;
        let r12 = self.data[1][2] / scale.z;
        let r20 = self.data[2][0] / scale.x;
        let r21 = self.data[2][1] / scale.y;
        let r22 = self.data[2][2] / scale.z;

        // XYZ extrinsic decomposition (R = Rz * Ry * Rx applied to a column vector)
        let sy = -r20;
        let (rx, ry, rz);
        if sy.abs() < 0.9999 {
            ry = sy.asin();
            rx = r21.atan2(r22);
            rz = r10.atan2(r00);
        } else {
            // Gimbal lock: rz arbitrary; pick rz = 0
            ry = sy.asin();
            rx = (-r12).atan2(r11);
            rz = 0.0;
        }
        Vec3::new(rx.to_degrees(), ry.to_degrees(), rz.to_degrees())
    }

    // Class-method factories

    pub fn from_translation(pos: &Vec3) -> RcMat4 {
        Self::from_rows([
            [1.0, 0.0, 0.0, pos.x],
            [0.0, 1.0, 0.0, pos.y],
            [0.0, 0.0, 1.0, pos.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn from_rotation(rot: &Vec3) -> RcMat4 {
        let rx = Self::rotation_x(rot.x);
        let ry = Self::rotation_y(rot.y);
        let rz = Self::rotation_z(rot.z);
        // XYZ extrinsic: result = Rz * Ry * Rx
        let zy = rc_ref!(&rz).mul_mat(rc_ref!(&ry));
        rc_ref!(&zy).mul_mat(rc_ref!(&rx))
    }

    pub fn from_scale(scale: &Vec3) -> RcMat4 {
        Self::from_rows([
            [scale.x, 0.0, 0.0, 0.0],
            [0.0, scale.y, 0.0, 0.0],
            [0.0, 0.0, scale.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn from_quat(quat: &Quat) -> RcMat4 {
        quat.to_matrix()
    }

    pub fn compose(pos: &Vec3, rot: &Vec3, scale: &Vec3) -> RcMat4 {
        let t = Self::from_translation(pos);
        let r = Self::from_rotation(rot);
        let s = Self::from_scale(scale);
        let tr = rc_ref!(&t).mul_mat(rc_ref!(&r));
        rc_ref!(&tr).mul_mat(rc_ref!(&s))
    }

    pub fn look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> RcMat4 {
        // Right-handed, forward = -Z. Camera looks toward target.
        let f_rc = target.sub(eye);
        let f = rc_ref!(&f_rc).normalize();
        let f = rc_ref!(&f);
        let s_rc = f.cross(up);
        let s = rc_ref!(&s_rc).normalize();
        let s = rc_ref!(&s);
        let u_rc = s.cross(f);
        let u = rc_ref!(&u_rc);
        Self::from_rows([
            [s.x, u.x, -f.x, eye.x],
            [s.y, u.y, -f.y, eye.y],
            [s.z, u.z, -f.z, eye.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // Operators

    pub fn mul_mat(&self, other: &Self) -> RcMat4 {
        let mut result = [[0.0; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut sum = 0.0;
                for k in 0..4 {
                    sum += self.data[i][k] * other.data[k][j];
                }
                result[i][j] = sum;
            }
        }
        Self::from_rows(result)
    }

    pub fn mul_vec(&self, v: &Vec3) -> RcVec3 {
        let x =
            self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z + self.data[0][3];
        let y =
            self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z + self.data[1][3];
        let z =
            self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z + self.data[2][3];
        Vec3::new(x, y, z)
    }

    pub fn mul_dir(&self, v: &Vec3) -> RcVec3 {
        // Transform direction vector (ignore translation row).
        let x = self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z;
        let y = self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z;
        let z = self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z;
        Vec3::new(x, y, z)
    }

    // Mutate methods (return new Mat4)

    pub fn translate(&self, v: &Vec3) -> RcMat4 {
        let t = Self::from_translation(v);
        self.mul_mat(rc_ref!(&t))
    }

    pub fn rotate(&self, axis: &Vec3, deg: f32) -> RcMat4 {
        let r = Self::rotation_axis_angle(axis, deg);
        self.mul_mat(rc_ref!(&r))
    }

    pub fn rotate_x(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_x(deg);
        self.mul_mat(rc_ref!(&r))
    }

    pub fn rotate_y(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_y(deg);
        self.mul_mat(rc_ref!(&r))
    }

    pub fn rotate_z(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_z(deg);
        self.mul_mat(rc_ref!(&r))
    }

    pub fn scale_by(&self, v: &Vec3) -> RcMat4 {
        let s = Self::from_scale(v);
        self.mul_mat(rc_ref!(&s))
    }

    // Matrix operations

    pub fn inverse(&self) -> RcMat4 {
        let m = &self.data;
        let mut inv = [[0.0_f32; 4]; 4];

        inv[0][0] =
            m[1][1] * m[2][2] * m[3][3] - m[1][1] * m[2][3] * m[3][2] - m[2][1] * m[1][2] * m[3][3]
                + m[2][1] * m[1][3] * m[3][2]
                + m[3][1] * m[1][2] * m[2][3]
                - m[3][1] * m[1][3] * m[2][2];
        inv[0][1] = -m[0][1] * m[2][2] * m[3][3]
            + m[0][1] * m[2][3] * m[3][2]
            + m[2][1] * m[0][2] * m[3][3]
            - m[2][1] * m[0][3] * m[3][2]
            - m[3][1] * m[0][2] * m[2][3]
            + m[3][1] * m[0][3] * m[2][2];
        inv[0][2] =
            m[0][1] * m[1][2] * m[3][3] - m[0][1] * m[1][3] * m[3][2] - m[1][1] * m[0][2] * m[3][3]
                + m[1][1] * m[0][3] * m[3][2]
                + m[3][1] * m[0][2] * m[1][3]
                - m[3][1] * m[0][3] * m[1][2];
        inv[0][3] = -m[0][1] * m[1][2] * m[2][3]
            + m[0][1] * m[1][3] * m[2][2]
            + m[1][1] * m[0][2] * m[2][3]
            - m[1][1] * m[0][3] * m[2][2]
            - m[2][1] * m[0][2] * m[1][3]
            + m[2][1] * m[0][3] * m[1][2];
        inv[1][0] = -m[1][0] * m[2][2] * m[3][3]
            + m[1][0] * m[2][3] * m[3][2]
            + m[2][0] * m[1][2] * m[3][3]
            - m[2][0] * m[1][3] * m[3][2]
            - m[3][0] * m[1][2] * m[2][3]
            + m[3][0] * m[1][3] * m[2][2];
        inv[1][1] =
            m[0][0] * m[2][2] * m[3][3] - m[0][0] * m[2][3] * m[3][2] - m[2][0] * m[0][2] * m[3][3]
                + m[2][0] * m[0][3] * m[3][2]
                + m[3][0] * m[0][2] * m[2][3]
                - m[3][0] * m[0][3] * m[2][2];
        inv[1][2] = -m[0][0] * m[1][2] * m[3][3]
            + m[0][0] * m[1][3] * m[3][2]
            + m[1][0] * m[0][2] * m[3][3]
            - m[1][0] * m[0][3] * m[3][2]
            - m[3][0] * m[0][2] * m[1][3]
            + m[3][0] * m[0][3] * m[1][2];
        inv[1][3] =
            m[0][0] * m[1][2] * m[2][3] - m[0][0] * m[1][3] * m[2][2] - m[1][0] * m[0][2] * m[2][3]
                + m[1][0] * m[0][3] * m[2][2]
                + m[2][0] * m[0][2] * m[1][3]
                - m[2][0] * m[0][3] * m[1][2];
        inv[2][0] =
            m[1][0] * m[2][1] * m[3][3] - m[1][0] * m[2][3] * m[3][1] - m[2][0] * m[1][1] * m[3][3]
                + m[2][0] * m[1][3] * m[3][1]
                + m[3][0] * m[1][1] * m[2][3]
                - m[3][0] * m[1][3] * m[2][1];
        inv[2][1] = -m[0][0] * m[2][1] * m[3][3]
            + m[0][0] * m[2][3] * m[3][1]
            + m[2][0] * m[0][1] * m[3][3]
            - m[2][0] * m[0][3] * m[3][1]
            - m[3][0] * m[0][1] * m[2][3]
            + m[3][0] * m[0][3] * m[2][1];
        inv[2][2] =
            m[0][0] * m[1][1] * m[3][3] - m[0][0] * m[1][3] * m[3][1] - m[1][0] * m[0][1] * m[3][3]
                + m[1][0] * m[0][3] * m[3][1]
                + m[3][0] * m[0][1] * m[1][3]
                - m[3][0] * m[0][3] * m[1][1];
        inv[2][3] = -m[0][0] * m[1][1] * m[2][3]
            + m[0][0] * m[1][3] * m[2][1]
            + m[1][0] * m[0][1] * m[2][3]
            - m[1][0] * m[0][3] * m[2][1]
            - m[2][0] * m[0][1] * m[1][3]
            + m[2][0] * m[0][3] * m[1][1];
        inv[3][0] = -m[1][0] * m[2][1] * m[3][2]
            + m[1][0] * m[2][2] * m[3][1]
            + m[2][0] * m[1][1] * m[3][2]
            - m[2][0] * m[1][2] * m[3][1]
            - m[3][0] * m[1][1] * m[2][2]
            + m[3][0] * m[1][2] * m[2][1];
        inv[3][1] =
            m[0][0] * m[2][1] * m[3][2] - m[0][0] * m[2][2] * m[3][1] - m[2][0] * m[0][1] * m[3][2]
                + m[2][0] * m[0][2] * m[3][1]
                + m[3][0] * m[0][1] * m[2][2]
                - m[3][0] * m[0][2] * m[2][1];
        inv[3][2] = -m[0][0] * m[1][1] * m[3][2]
            + m[0][0] * m[1][2] * m[3][1]
            + m[1][0] * m[0][1] * m[3][2]
            - m[1][0] * m[0][2] * m[3][1]
            - m[3][0] * m[0][1] * m[1][2]
            + m[3][0] * m[0][2] * m[1][1];
        inv[3][3] =
            m[0][0] * m[1][1] * m[2][2] - m[0][0] * m[1][2] * m[2][1] - m[1][0] * m[0][1] * m[2][2]
                + m[1][0] * m[0][2] * m[2][1]
                + m[2][0] * m[0][1] * m[1][2]
                - m[2][0] * m[0][2] * m[1][1];

        let det =
            m[0][0] * inv[0][0] + m[0][1] * inv[1][0] + m[0][2] * inv[2][0] + m[0][3] * inv[3][0];
        if det.abs() < 1e-12 {
            return Self::identity();
        }
        let inv_det = 1.0 / det;
        for i in 0..4 {
            for j in 0..4 {
                inv[i][j] *= inv_det;
            }
        }
        Self::from_rows(inv)
    }

    pub fn transpose(&self) -> RcMat4 {
        let m = &self.data;
        Self::from_rows([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ])
    }

    pub fn determinant(&self) -> f32 {
        let m = &self.data;
        m[0][0]
            * (m[1][1] * (m[2][2] * m[3][3] - m[2][3] * m[3][2])
                - m[1][2] * (m[2][1] * m[3][3] - m[2][3] * m[3][1])
                + m[1][3] * (m[2][1] * m[3][2] - m[2][2] * m[3][1]))
            - m[0][1]
                * (m[1][0] * (m[2][2] * m[3][3] - m[2][3] * m[3][2])
                    - m[1][2] * (m[2][0] * m[3][3] - m[2][3] * m[3][0])
                    + m[1][3] * (m[2][0] * m[3][2] - m[2][2] * m[3][0]))
            + m[0][2]
                * (m[1][0] * (m[2][1] * m[3][3] - m[2][3] * m[3][1])
                    - m[1][1] * (m[2][0] * m[3][3] - m[2][3] * m[3][0])
                    + m[1][3] * (m[2][0] * m[3][1] - m[2][1] * m[3][0]))
            - m[0][3]
                * (m[1][0] * (m[2][1] * m[3][2] - m[2][2] * m[3][1])
                    - m[1][1] * (m[2][0] * m[3][2] - m[2][2] * m[3][0])
                    + m[1][2] * (m[2][0] * m[3][1] - m[2][1] * m[3][0]))
    }

    // Coordinate system conversions

    pub fn to_local(&self, mat: &Self) -> RcMat4 {
        let inv_rc = mat.inverse();
        rc_ref!(&inv_rc).mul_mat(self)
    }

    pub fn to_world(&self, mat: &Self) -> RcMat4 {
        mat.mul_mat(self)
    }

    pub fn to_local_dir(&self, mat: &Self) -> RcMat4 {
        // Build a translation-stripped inverse.
        let r_only = Self::strip_translation(mat);
        let inv_rc = rc_ref!(&r_only).inverse();
        rc_ref!(&inv_rc).mul_mat(self)
    }

    pub fn to_world_dir(&self, mat: &Self) -> RcMat4 {
        let r_only = Self::strip_translation(mat);
        rc_ref!(&r_only).mul_mat(self)
    }

    // Internal helpers

    fn rotation_x(deg: f32) -> RcMat4 {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self::from_rows([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, c, -s, 0.0],
            [0.0, s, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_y(deg: f32) -> RcMat4 {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self::from_rows([
            [c, 0.0, s, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-s, 0.0, c, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_z(deg: f32) -> RcMat4 {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self::from_rows([
            [c, -s, 0.0, 0.0],
            [s, c, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn rotation_axis_angle(axis: &Vec3, deg: f32) -> RcMat4 {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        let one_minus_c = 1.0 - c;
        let len = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if len < 1e-12 {
            return Self::identity();
        }
        let x = axis.x / len;
        let y = axis.y / len;
        let z = axis.z / len;
        Self::from_rows([
            [
                c + x * x * one_minus_c,
                x * y * one_minus_c - z * s,
                x * z * one_minus_c + y * s,
                0.0,
            ],
            [
                y * x * one_minus_c + z * s,
                c + y * y * one_minus_c,
                y * z * one_minus_c - x * s,
                0.0,
            ],
            [
                z * x * one_minus_c - y * s,
                z * y * one_minus_c + x * s,
                c + z * z * one_minus_c,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    fn strip_translation(mat: &Self) -> RcMat4 {
        let mut data = mat.data;
        data[0][3] = 0.0;
        data[1][3] = 0.0;
        data[2][3] = 0.0;
        Self::from_rows(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deref(rc: &RcMat4) -> Mat4 {
        *rc_ref!(rc)
    }

    fn deref_v(rc: &RcVec3) -> Vec3 {
        *rc_ref!(rc)
    }

    fn approx_eq_mat(a: &Mat4, b: &Mat4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if (a.data[i][j] - b.data[i][j]).abs() > 1e-4 {
                    return false;
                }
            }
        }
        true
    }

    fn approx_eq_vec(a: &Vec3, b: &Vec3) -> bool {
        (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4 && (a.z - b.z).abs() < 1e-4
    }

    #[test]
    fn test_identity() {
        let m = deref(&Mat4::identity());
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(m.data[i][j], if i == j { 1.0 } else { 0.0 });
            }
        }
    }

    #[test]
    fn test_get() {
        let m = deref(&Mat4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]));
        assert_eq!(m.get(0, 0), 1.0);
        assert_eq!(m.get(1, 2), 7.0);
        assert_eq!(m.get(3, 3), 16.0);
    }

    #[test]
    fn test_from_translation_and_pos() {
        let v = Vec3 {
            x: 5.0,
            y: 6.0,
            z: 7.0,
        };
        let m = deref(&Mat4::from_translation(&v));
        assert_eq!(m.data[0][3], 5.0);
        assert_eq!(m.data[1][3], 6.0);
        assert_eq!(m.data[2][3], 7.0);
        let p = deref_v(&m.pos());
        assert_eq!(p, v);
    }

    #[test]
    fn test_from_scale_and_scale_vec() {
        let v = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let m = deref(&Mat4::from_scale(&v));
        assert_eq!(m.data[0][0], 2.0);
        assert_eq!(m.data[1][1], 3.0);
        assert_eq!(m.data[2][2], 4.0);
        let s = deref_v(&m.scale_vec());
        assert!(approx_eq_vec(&s, &v));
    }

    #[test]
    fn test_from_rotation_y_90() {
        let v = Vec3 {
            x: 0.0,
            y: 90.0,
            z: 0.0,
        };
        let m = deref(&Mat4::from_rotation(&v));
        // Rotating (1, 0, 0) by 90 deg around Y -> (0, 0, -1) in right-handed.
        let p = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r = deref_v(&m.mul_vec(&p));
        assert!(approx_eq_vec(
            &r,
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0
            }
        ));
    }

    #[test]
    fn test_compose_decompose() {
        let pos = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let rot = Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        };
        let scale = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let m = deref(&Mat4::compose(&pos, &rot, &scale));
        assert!(approx_eq_vec(&deref_v(&m.pos()), &pos));
        assert!(approx_eq_vec(&deref_v(&m.scale_vec()), &scale));
        assert!(approx_eq_vec(&deref_v(&m.rot()), &rot));
    }

    #[test]
    fn test_mul_mat_identity() {
        let m = Mat4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let m_ref = rc_ref!(&m);
        let i = Mat4::identity();
        let result = deref(&m_ref.mul_mat(rc_ref!(&i)));
        assert!(approx_eq_mat(&result, m_ref));
    }

    #[test]
    fn test_mul_vec_translation() {
        let t = Mat4::from_translation(&Vec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        });
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let r = deref_v(&rc_ref!(&t).mul_vec(&v));
        assert!(approx_eq_vec(
            &r,
            &Vec3 {
                x: 11.0,
                y: 22.0,
                z: 33.0
            }
        ));
    }

    #[test]
    fn test_mul_dir_ignores_translation() {
        let t = Mat4::from_translation(&Vec3 {
            x: 10.0,
            y: 20.0,
            z: 30.0,
        });
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let r = deref_v(&rc_ref!(&t).mul_dir(&v));
        assert!(approx_eq_vec(&r, &v));
    }

    #[test]
    fn test_inverse_round_trip() {
        let m = Mat4::compose(
            &Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            &Vec3 {
                x: 30.0,
                y: 45.0,
                z: 60.0,
            },
            &Vec3 {
                x: 1.5,
                y: 2.0,
                z: 0.5,
            },
        );
        let m_ref = rc_ref!(&m);
        let inv = m_ref.inverse();
        let identity = m_ref.mul_mat(rc_ref!(&inv));
        let i_default = Mat4::identity();
        assert!(approx_eq_mat(rc_ref!(&identity), rc_ref!(&i_default)));
    }

    #[test]
    fn test_transpose() {
        let m = Mat4::from_rows([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ]);
        let t = deref(&rc_ref!(&m).transpose());
        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(t.data[i][j], rc_ref!(&m).data[j][i]);
            }
        }
    }

    #[test]
    fn test_determinant_identity() {
        assert!((rc_ref!(&Mat4::identity()).determinant() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_translate_method() {
        let m = Mat4::identity();
        let v = Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let result = rc_ref!(&m).translate(&v);
        let p = deref_v(&rc_ref!(&result).pos());
        assert!(approx_eq_vec(&p, &v));
    }

    #[test]
    fn test_rotate_x() {
        let m = Mat4::identity();
        let result = rc_ref!(&m).rotate_x(90.0);
        let v = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let r = deref_v(&rc_ref!(&result).mul_vec(&v));
        // (0,1,0) rotated 90 deg around X -> (0, 0, 1)
        assert!(approx_eq_vec(
            &r,
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0
            }
        ));
    }

    #[test]
    fn test_scale_by_method() {
        let m = Mat4::identity();
        let v = Vec3 {
            x: 2.0,
            y: 3.0,
            z: 4.0,
        };
        let result = rc_ref!(&m).scale_by(&v);
        let s = deref_v(&rc_ref!(&result).scale_vec());
        assert!(approx_eq_vec(&s, &v));
    }

    #[test]
    fn test_look_at_basic() {
        let eye = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        };
        let target = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let up = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let m = deref(&Mat4::look_at(&eye, &target, &up));
        // The translation column should equal eye position.
        assert!(approx_eq_vec(
            &Vec3 {
                x: m.data[0][3],
                y: m.data[1][3],
                z: m.data[2][3]
            },
            &eye,
        ));
    }
}
