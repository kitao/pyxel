// Math primitives follow the standard linear-algebra notation: i/j/k for
// matrix indices and c/s for cos/sin. Forcing iterator-based matrix code or
// renaming geometric components hurts readability without clarifying intent.
#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]

use crate::cube::quat::{Quat, RcQuat};
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

    // Plain (non-Rc) identity for callers that need a value-typed Mat4
    // (raster / draw internals). Avoids materializing through RcMat4.
    pub fn identity_value() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    // Element access

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row][col]
    }

    // Decomposed view (assumes affine T * R(XYZ extrinsic) * S)

    pub fn pos(&self) -> RcVec3 {
        let p = self.pos_value();
        Vec3::new(p.x, p.y, p.z)
    }

    // Plain (non-Rc) translation column for internal hot paths; see the
    // value-typed operator cores under Operators.
    pub fn pos_value(&self) -> Vec3 {
        Vec3 {
            x: self.data[0][3],
            y: self.data[1][3],
            z: self.data[2][3],
        }
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

    pub fn rot(&self) -> RcQuat {
        // Strip scale from the upper-left 3x3 and derive the rotation Quat.
        let scale = *rc_ref!(&self.scale_vec());
        let sx = if scale.x.abs() > 1e-9 { scale.x } else { 1.0 };
        let sy = if scale.y.abs() > 1e-9 { scale.y } else { 1.0 };
        let sz = if scale.z.abs() > 1e-9 { scale.z } else { 1.0 };
        let rot_only = Mat4 {
            data: [
                [
                    self.data[0][0] / sx,
                    self.data[0][1] / sy,
                    self.data[0][2] / sz,
                    0.0,
                ],
                [
                    self.data[1][0] / sx,
                    self.data[1][1] / sy,
                    self.data[1][2] / sz,
                    0.0,
                ],
                [
                    self.data[2][0] / sx,
                    self.data[2][1] / sy,
                    self.data[2][2] / sz,
                    0.0,
                ],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        Quat::from_matrix(&rot_only)
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

    pub fn from_euler(euler: &Vec3) -> RcMat4 {
        let rx = Self::rotation_x(euler.x);
        let ry = Self::rotation_y(euler.y);
        let rz = Self::rotation_z(euler.z);
        // XYZ extrinsic: result = Rz * Ry * Rx
        let zy = rc_ref!(&rz).mul_mat(rc_ref!(&ry));
        rc_ref!(&zy).mul_mat(rc_ref!(&rx))
    }

    pub fn from_axis_angle(axis: &Vec3, deg: f32) -> RcMat4 {
        Self::rotation_axis_angle(axis, deg)
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

    pub fn compose(pos: &Vec3, rot: &Quat, scale: &Vec3) -> RcMat4 {
        let t = Self::from_translation(pos);
        let r = Self::from_quat(rot);
        let s = Self::from_scale(scale);
        let tr = rc_ref!(&t).mul_mat(rc_ref!(&r));
        rc_ref!(&tr).mul_mat(rc_ref!(&s))
    }

    pub fn look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> RcMat4 {
        // Right-handed, forward = -Z. Camera looks toward target.
        let f_rc = target.sub(eye);
        let f = rc_ref!(&f_rc).normalize();
        let f = rc_ref!(&f);
        let mut s_rc = f.cross(up);
        if rc_ref!(&s_rc).length_squared() < 1e-12 {
            // up is parallel to forward (e.g. looking straight up with
            // up=Vec3.UP). Pick a fallback up that is guaranteed not to
            // be parallel: world Z when forward aligns with world Y,
            // world Y otherwise.
            let alt_up = if f.y.abs() > 0.9 {
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                }
            } else {
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                }
            };
            s_rc = f.cross(&alt_up);
        }
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
        Self::from_rows(self.mul_mat_value(other).data)
    }

    // Plain (non-Rc) operator cores for internal hot paths (raster, draw,
    // scene, collision): per-vertex and per-triangle work must not heap-
    // allocate. The Rc operators delegate here, so both paths compute
    // bit-identical results.

    #[must_use]
    pub fn mul_mat_value(&self, other: &Self) -> Self {
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
        Self { data: result }
    }

    pub fn mul_vec(&self, v: &Vec3) -> RcVec3 {
        let r = self.mul_vec_value(v);
        Vec3::new(r.x, r.y, r.z)
    }

    pub fn mul_vec_value(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0][0] * v.x
                + self.data[0][1] * v.y
                + self.data[0][2] * v.z
                + self.data[0][3],
            y: self.data[1][0] * v.x
                + self.data[1][1] * v.y
                + self.data[1][2] * v.z
                + self.data[1][3],
            z: self.data[2][0] * v.x
                + self.data[2][1] * v.y
                + self.data[2][2] * v.z
                + self.data[2][3],
        }
    }

    pub fn mul_dir(&self, v: &Vec3) -> RcVec3 {
        let r = self.mul_dir_value(v);
        Vec3::new(r.x, r.y, r.z)
    }

    // Transform direction vector (ignore translation row).
    pub fn mul_dir_value(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z,
            y: self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z,
            z: self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z,
        }
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
        Self::from_rows(self.inverse_value().data)
    }

    // Plain (non-Rc) inverse for internal hot paths; see the value-typed
    // operator cores under Operators. A singular matrix falls back to
    // identity, keeping the panic-free posture of the Rc operator.
    #[must_use]
    pub fn inverse_value(&self) -> Self {
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
            return Self::identity_value();
        }
        let inv_det = 1.0 / det;
        for i in 0..4 {
            for j in 0..4 {
                inv[i][j] *= inv_det;
            }
        }
        Self { data: inv }
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
    use crate::cube::quat::Quat;

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
    fn test_from_euler_y_90() {
        let v = Vec3 {
            x: 0.0,
            y: 90.0,
            z: 0.0,
        };
        let m = deref(&Mat4::from_euler(&v));
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
        let rot_euler = Vec3 {
            x: 0.0,
            y: 45.0,
            z: 0.0,
        };
        let rot_rc = Quat::from_euler(&rot_euler);
        let rot = *rc_ref!(&rot_rc);
        let scale = Vec3 {
            x: 2.0,
            y: 2.0,
            z: 2.0,
        };
        let m = deref(&Mat4::compose(&pos, &rot, &scale));
        assert!(approx_eq_vec(&deref_v(&m.pos()), &pos));
        assert!(approx_eq_vec(&deref_v(&m.scale_vec()), &scale));
        let extracted = m.rot();
        let extracted_euler = deref_v(&rc_ref!(&extracted).to_euler());
        assert!(approx_eq_vec(&extracted_euler, &rot_euler));
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
        let rot_rc = Quat::from_euler(&Vec3 {
            x: 30.0,
            y: 45.0,
            z: 60.0,
        });
        let rot = *rc_ref!(&rot_rc);
        let m = Mat4::compose(
            &Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            &rot,
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

    #[test]
    fn test_look_at_basis_columns() {
        // Eye at +Z looking toward origin with world up: column 0 = +X
        // (right), column 1 = +Y (up), column 2 = +Z (the negated
        // forward, since forward = target-eye = -Z).
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
        assert!((m.data[0][0] - 1.0).abs() < 1e-5); // right.x
        assert!(m.data[1][0].abs() < 1e-5);
        assert!(m.data[2][0].abs() < 1e-5);
        assert!(m.data[0][1].abs() < 1e-5);
        assert!((m.data[1][1] - 1.0).abs() < 1e-5); // up.y
        assert!(m.data[2][1].abs() < 1e-5);
        assert!(m.data[0][2].abs() < 1e-5);
        assert!(m.data[1][2].abs() < 1e-5);
        assert!((m.data[2][2] - 1.0).abs() < 1e-5); // -forward.z
    }

    #[test]
    fn test_look_at_parallel_up_does_not_collapse() {
        // Camera looking straight down with up = world up: forward is
        // parallel to up, the naive f.cross(up) collapses. The fallback
        // must pick another axis so the resulting matrix is invertible.
        let eye = Vec3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
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
        // The translation column still equals eye.
        assert_eq!(m.data[0][3], 0.0);
        assert_eq!(m.data[1][3], 5.0);
        assert_eq!(m.data[2][3], 0.0);
        // The basis must be non-degenerate (determinant of the 3x3
        // upper-left ≠ 0). For an orthonormal rotation it should be ±1.
        let det = m.data[0][0] * (m.data[1][1] * m.data[2][2] - m.data[1][2] * m.data[2][1])
            - m.data[0][1] * (m.data[1][0] * m.data[2][2] - m.data[1][2] * m.data[2][0])
            + m.data[0][2] * (m.data[1][0] * m.data[2][1] - m.data[1][1] * m.data[2][0]);
        assert!(det.abs() > 0.5, "look_at basis collapsed (det={det})");
    }

    #[test]
    fn test_look_at_parallel_up_inverse_up() {
        // Looking straight up with up = world up: forward and up are
        // anti-parallel; the same fallback path must engage.
        let eye = Vec3 {
            x: 0.0,
            y: -5.0,
            z: 0.0,
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
        let det = m.data[0][0] * (m.data[1][1] * m.data[2][2] - m.data[1][2] * m.data[2][1])
            - m.data[0][1] * (m.data[1][0] * m.data[2][2] - m.data[1][2] * m.data[2][0])
            + m.data[0][2] * (m.data[1][0] * m.data[2][1] - m.data[1][1] * m.data[2][0]);
        assert!(det.abs() > 0.5, "look_at basis collapsed (det={det})");
    }

    #[test]
    fn test_from_axis_angle_y_90() {
        // (1, 0, 0) rotated 90° around Y → (0, 0, -1) in right-handed.
        let axis = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let m = Mat4::from_axis_angle(&axis, 90.0);
        let v = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r = deref_v(&rc_ref!(&m).mul_vec(&v));
        assert!(approx_eq_vec(
            &r,
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        ));
    }

    #[test]
    fn test_from_axis_angle_normalizes_axis() {
        // A non-unit axis must be normalized internally so the rotation
        // angle stays at the requested 90°.
        let axis = Vec3 {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        };
        let m = Mat4::from_axis_angle(&axis, 90.0);
        let v = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r = deref_v(&rc_ref!(&m).mul_vec(&v));
        assert!(approx_eq_vec(
            &r,
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: -1.0,
            }
        ));
    }

    #[test]
    fn test_from_axis_angle_zero_axis_yields_identity() {
        let axis = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let m = deref(&Mat4::from_axis_angle(&axis, 90.0));
        let id = deref(&Mat4::identity());
        assert!(approx_eq_mat(&m, &id));
    }

    #[test]
    fn test_from_euler_each_axis_matches_from_axis_angle() {
        for (axis, label) in [
            (
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                "X",
            ),
            (
                Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
                "Y",
            ),
            (
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 1.0,
                },
                "Z",
            ),
        ] {
            let euler = Vec3 {
                x: axis.x * 30.0,
                y: axis.y * 30.0,
                z: axis.z * 30.0,
            };
            let m_euler = deref(&Mat4::from_euler(&euler));
            let m_axis = deref(&Mat4::from_axis_angle(&axis, 30.0));
            assert!(
                approx_eq_mat(&m_euler, &m_axis),
                "from_euler vs from_axis_angle disagree on {label}",
            );
        }
    }

    #[test]
    fn test_inverse_singular_returns_identity() {
        // Zero matrix has det=0 — the inverse falls back to identity to
        // keep callers panic-free (cube-design.md performance posture).
        let m = Mat4::from_rows([[0.0; 4]; 4]);
        let inv = deref(&rc_ref!(&m).inverse());
        let id = deref(&Mat4::identity());
        assert!(approx_eq_mat(&inv, &id));
    }

    #[test]
    fn test_to_world_and_to_local_round_trip() {
        let outer = Mat4::from_translation(&Vec3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        });
        let outer_val = *rc_ref!(&outer);
        let inner = Mat4::from_translation(&Vec3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });
        let inner_val = *rc_ref!(&inner);
        let world = inner_val.to_world(&outer_val);
        let world_val = *rc_ref!(&world);
        let pos = deref_v(&world_val.pos());
        assert!((pos.x - 15.0).abs() < 1e-5);
        let back = world_val.to_local(&outer_val);
        let back_val = *rc_ref!(&back);
        let back_pos = deref_v(&back_val.pos());
        assert!((back_pos.x - 5.0).abs() < 1e-5);
    }

    #[test]
    fn test_to_world_dir_ignores_outer_translation() {
        let outer = Mat4::from_translation(&Vec3 {
            x: 10.0,
            y: 0.0,
            z: 0.0,
        });
        let outer_val = *rc_ref!(&outer);
        let inner = Mat4::from_translation(&Vec3 {
            x: 5.0,
            y: 0.0,
            z: 0.0,
        });
        let inner_val = *rc_ref!(&inner);
        // _dir variants drop translation: inner's translation column
        // stays, outer's contribution is the rotation-only part.
        let world = inner_val.to_world_dir(&outer_val);
        let world_val = *rc_ref!(&world);
        let pos = deref_v(&world_val.pos());
        // outer's rotation is identity, so inner's translation passes
        // through unmodified.
        assert!((pos.x - 5.0).abs() < 1e-5);
    }
}
