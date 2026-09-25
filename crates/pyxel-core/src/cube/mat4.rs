// Matrix formulas use indexed loops and conventional i/j/k and c/s notation.
#![allow(clippy::many_single_char_names, clippy::needless_range_loop)]

use crate::cube::quat::{Quat, RcQuat};
use crate::cube::vec3::{RcVec3, Vec3};

// Row-major storage: data[row][col].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Mat4 {
    pub data: [[f32; 4]; 4],
}

define_rc_type!(RcMat4, Mat4);

impl Mat4 {
    pub fn from_rows(data: [[f32; 4]; 4]) -> RcMat4 {
        new_rc_type!(Mat4 { data })
    }

    pub fn identity() -> RcMat4 {
        Self::from_rows(Self::identity_value().data)
    }

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

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row][col]
    }

    // Decomposed view (assumes affine T * R(XYZ extrinsic) * S)

    pub fn pos(&self) -> RcVec3 {
        let p = self.pos_value();
        Vec3::new(p.x, p.y, p.z)
    }

    pub fn pos_value(&self) -> Vec3 {
        Vec3 {
            x: self.data[0][3],
            y: self.data[1][3],
            z: self.data[2][3],
        }
    }

    pub fn scale_vec(&self) -> RcVec3 {
        let scale = self.scale_vec_value();
        Vec3::new(scale.x, scale.y, scale.z)
    }

    pub(crate) fn scale_vec_value(&self) -> Vec3 {
        // Wider intermediates keep finite f32 scales from underflowing or overflowing.
        let mut columns = [[0.0_f64; 3]; 3];
        let mut scale = [0.0; 3];
        for i in 0..3 {
            for j in 0..3 {
                columns[i][j] = f64::from(self.data[j][i]);
            }
            // Preserve ordinary f32 rounding so existing physics trajectories stay stable.
            let length_squared =
                self.data[0][i].powi(2) + self.data[1][i].powi(2) + self.data[2][i].powi(2);
            scale[i] = if length_squared.is_normal() {
                length_squared.sqrt()
            } else {
                columns[i].iter().map(|v| v * v).sum::<f64>().sqrt() as f32
            };
        }

        // Match GLB matrix decomposition: keep reflection in Z and a proper rotation.
        let [x, y, z] = columns;
        let det = x[0] * (y[1] * z[2] - y[2] * z[1]) - y[0] * (x[1] * z[2] - x[2] * z[1])
            + z[0] * (x[1] * y[2] - x[2] * y[1]);
        if det < 0.0 {
            scale[2] = -scale[2];
        }
        Vec3 {
            x: scale[0],
            y: scale[1],
            z: scale[2],
        }
    }

    pub fn rot(&self) -> RcQuat {
        let rot = self.rot_value();
        Quat::new(rot.x, rot.y, rot.z, rot.w)
    }

    pub(crate) fn rot_value(&self) -> Quat {
        let scale = self.scale_vec_value();
        if scale.x == 0.0 || scale.y == 0.0 || scale.z == 0.0 {
            return Quat {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            };
        }
        let (sx, sy, sz) = (scale.x, scale.y, scale.z);

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

        Quat::from_matrix_value(&rot_only)
    }

    // Operators

    pub fn mul_mat(&self, other: &Self) -> RcMat4 {
        Self::from_rows(self.mul_mat_value(other).data)
    }

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

    // Transform direction vector (ignore translation column).
    pub fn mul_dir_value(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z,
            y: self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z,
            z: self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z,
        }
    }

    // Factories

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
        let zy = rc_ref!(&rz).mul_mat(&rc_ref!(&ry));
        let result = rc_ref!(&zy).mul_mat(&rc_ref!(&rx));
        result
    }

    pub fn from_axis_angle(axis: &Vec3, deg: f32) -> RcMat4 {
        Self::from_rows(Self::from_axis_angle_value(axis, deg).data)
    }

    pub(crate) fn from_axis_angle_value(axis: &Vec3, deg: f32) -> Self {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        let one_minus_c = 1.0 - c;
        let len = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if len < 1e-12 {
            return Self::identity_value();
        }
        let x = axis.x / len;
        let y = axis.y / len;
        let z = axis.z / len;

        Self {
            data: [
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
            ],
        }
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
        new_rc_type!(Self::compose_value(pos, rot, scale))
    }

    pub(crate) fn compose_value(pos: &Vec3, rot: &Quat, scale: &Vec3) -> Self {
        let mut t = Self::identity_value();
        t.data[0][3] = pos.x;
        t.data[1][3] = pos.y;
        t.data[2][3] = pos.z;
        let r = rot.matrix_value();
        let mut s = Self::identity_value();
        s.data[0][0] = scale.x;
        s.data[1][1] = scale.y;
        s.data[2][2] = scale.z;
        t.mul_mat_value(&r).mul_mat_value(&s)
    }

    pub fn look_at(eye: &Vec3, target: &Vec3, up: &Vec3) -> RcMat4 {
        // Camera-to-world transform; right-handed with forward = -Z.
        let f_rc = target.sub(eye);
        let f = rc_ref!(&f_rc).normalize();
        let f = rc_ref!(&f);

        let mut s_rc = f.cross(up);
        if rc_ref!(&s_rc).length_squared() < 1e-12 {
            // Choose a nonparallel up axis when the requested one collapses the basis.
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

        let u_rc = s.cross(&f);
        let u = rc_ref!(&u_rc);
        Self::from_rows([
            [s.x, u.x, -f.x, eye.x],
            [s.y, u.y, -f.y, eye.y],
            [s.z, u.z, -f.z, eye.z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // Transform composition

    pub fn translate(&self, v: &Vec3) -> RcMat4 {
        let t = Self::from_translation(v);
        let result = self.mul_mat(&rc_ref!(&t));
        result
    }

    pub fn rotate(&self, axis: &Vec3, deg: f32) -> RcMat4 {
        let r = Self::from_axis_angle(axis, deg);
        let result = self.mul_mat(&rc_ref!(&r));
        result
    }

    pub fn rotate_x(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_x(deg);
        let result = self.mul_mat(&rc_ref!(&r));
        result
    }

    pub fn rotate_y(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_y(deg);
        let result = self.mul_mat(&rc_ref!(&r));
        result
    }

    pub fn rotate_z(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_z(deg);
        let result = self.mul_mat(&rc_ref!(&r));
        result
    }

    pub fn scale_by(&self, v: &Vec3) -> RcMat4 {
        let s = Self::from_scale(v);
        let result = self.mul_mat(&rc_ref!(&s));
        result
    }

    // Matrix operations

    pub fn inverse(&self) -> RcMat4 {
        Self::from_rows(self.inverse_value().data)
    }

    // Adjugate divided by determinant; near-singular matrices fall back to identity.
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
        for row in &mut inv {
            for value in row {
                *value *= inv_det;
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
        let result = rc_ref!(&inv_rc).mul_mat(self);
        result
    }

    pub fn to_world(&self, mat: &Self) -> RcMat4 {
        mat.mul_mat(self)
    }

    pub fn to_local_dir(&self, mat: &Self) -> RcMat4 {
        let r_only = Self::strip_translation(mat);
        let inv_rc = r_only.inverse();
        let result = rc_ref!(&inv_rc).mul_mat(self);
        result
    }

    pub fn to_world_dir(&self, mat: &Self) -> RcMat4 {
        let r_only = Self::strip_translation(mat);
        r_only.mul_mat(self)
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

    fn strip_translation(mat: &Self) -> Self {
        let mut data = mat.data;
        data[0][3] = 0.0;
        data[1][3] = 0.0;
        data[2][3] = 0.0;
        Self { data }
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

    // 1e-4 exceeds the f32 rounding of these unit-scale values and stays below any expected difference
    fn approx_eq_mat(a: &Mat4, b: &Mat4) -> bool {
        a.data
            .iter()
            .flatten()
            .zip(b.data.iter().flatten())
            .all(|(a, b)| (a - b).abs() <= 1e-4)
    }

    fn approx_eq_vec(a: &Vec3, b: &Vec3) -> bool {
        (a.x - b.x).abs() < 1e-4 && (a.y - b.y).abs() < 1e-4 && (a.z - b.z).abs() < 1e-4
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
        assert_eq!(s, v);
        assert_eq!(m.scale_vec_value(), s);
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

        let result = deref(&m_ref.mul_mat(&rc_ref!(&i)));
        assert_eq!(result, *m_ref);
    }

    #[test]
    fn test_mul_vec_value_applies_translation() {
        let t = Mat4::from_translation(&Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        });
        let r = rc_ref!(&t).mul_vec_value(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        assert_eq!((r.x, r.y, r.z), (1.0, 2.0, 3.0));
    }

    #[test]
    fn test_mul_vec_value_identity_preserves_vec3() {
        let m = Mat4::identity();
        let r = rc_ref!(&m).mul_vec_value(&Vec3 {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        });
        assert_eq!((r.x, r.y, r.z), (4.0, 5.0, 6.0));
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
        assert_eq!(r, v);
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
        assert_eq!(m.data[0][..3], [1.0, 0.0, 0.0]);
        assert_eq!(m.data[1][..3], [0.0, 1.0, 0.0]);
        assert_eq!(m.data[2][..3], [0.0, 0.0, 1.0]);
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
        assert_eq!(m.data[0][3], 0.0);
        assert_eq!(m.data[1][3], 5.0);
        assert_eq!(m.data[2][3], 0.0);

        // The fallback deterministically yields a proper orthonormal
        // rotation: the 3x3 upper-left determinant is exactly +1.
        let det = m.data[0][0] * (m.data[1][1] * m.data[2][2] - m.data[1][2] * m.data[2][1])
            - m.data[0][1] * (m.data[1][0] * m.data[2][2] - m.data[1][2] * m.data[2][0])
            + m.data[0][2] * (m.data[1][0] * m.data[2][1] - m.data[1][1] * m.data[2][0]);
        assert!(
            (det - 1.0).abs() < 1e-5,
            "look_at basis collapsed (det={det})"
        );
    }

    #[test]
    fn test_look_at_parallel_up_inverse_up() {
        // Forward and up both point upward, so the fallback path must engage.
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
        // A proper fallback rotation has determinant +1.
        let det = m.data[0][0] * (m.data[1][1] * m.data[2][2] - m.data[1][2] * m.data[2][1])
            - m.data[0][1] * (m.data[1][0] * m.data[2][2] - m.data[1][2] * m.data[2][0])
            + m.data[0][2] * (m.data[1][0] * m.data[2][1] - m.data[1][1] * m.data[2][0]);
        assert!(
            (det - 1.0).abs() < 1e-5,
            "look_at basis collapsed (det={det})"
        );
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
        assert_eq!(m, id);
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
        let m = Mat4::from_rows([[0.0; 4]; 4]);
        let inv = deref(&rc_ref!(&m).inverse());
        let id = deref(&Mat4::identity());
        assert_eq!(inv, id);
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
        assert_eq!(pos.x, 5.0);
    }
}
