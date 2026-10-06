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

        // Retain ordinary f32 accumulation; only lost-range results need recovery.
        if result
            .iter()
            .flatten()
            .fold(true, |finite, value| finite & value.is_finite())
        {
            Self { data: result }
        } else {
            self.recover_matrix_result(other, result)
        }
    }

    #[cold]
    fn recover_matrix_result(&self, other: &Self, mut result: [[f32; 4]; 4]) -> Self {
        for i in 0..4 {
            for j in 0..4 {
                if !result[i][j].is_finite()
                    && self.data[i].iter().all(|value| value.is_finite())
                    && (0..4).all(|k| other.data[k][j].is_finite())
                {
                    let mut sum = 0.0_f64;
                    for k in 0..4 {
                        sum += f64::from(self.data[i][k]) * f64::from(other.data[k][j]);
                    }
                    result[i][j] = sum as f32;
                }
            }
        }
        Self { data: result }
    }

    pub fn mul_vec(&self, v: &Vec3) -> RcVec3 {
        let r = self.mul_vec_value(v);
        Vec3::new(r.x, r.y, r.z)
    }

    pub fn mul_vec_value(&self, v: &Vec3) -> Vec3 {
        let result = Vec3 {
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
        };
        if result.is_finite() {
            result
        } else {
            self.recover_vector_result(v, result, true)
        }
    }

    pub fn mul_dir(&self, v: &Vec3) -> RcVec3 {
        let r = self.mul_dir_value(v);
        Vec3::new(r.x, r.y, r.z)
    }

    // Transform direction vector (ignore translation column).
    pub fn mul_dir_value(&self, v: &Vec3) -> Vec3 {
        let result = Vec3 {
            x: self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2] * v.z,
            y: self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2] * v.z,
            z: self.data[2][0] * v.x + self.data[2][1] * v.y + self.data[2][2] * v.z,
        };
        if result.is_finite() {
            result
        } else {
            self.recover_vector_result(v, result, false)
        }
    }

    #[cold]
    fn recover_vector_result(&self, v: &Vec3, result: Vec3, translation: bool) -> Vec3 {
        if !v.is_finite() {
            return result;
        }
        let mut components = [result.x, result.y, result.z];
        for i in 0..3 {
            let row = self.data[i];
            let used_columns = if translation { 4 } else { 3 };
            if !components[i].is_finite()
                && row[..used_columns].iter().all(|value| value.is_finite())
            {
                let sum = f64::from(row[0]) * f64::from(v.x)
                    + f64::from(row[1]) * f64::from(v.y)
                    + f64::from(row[2]) * f64::from(v.z);
                components[i] = (if translation {
                    sum + f64::from(row[3])
                } else {
                    sum
                }) as f32;
            }
        }
        Vec3 {
            x: components[0],
            y: components[1],
            z: components[2],
        }
    }

    // Factories

    pub fn from_translation(pos: &Vec3) -> RcMat4 {
        new_rc_type!(Self::from_translation_value(pos))
    }

    fn from_translation_value(pos: &Vec3) -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, pos.x],
                [0.0, 1.0, 0.0, pos.y],
                [0.0, 0.0, 1.0, pos.z],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_euler(euler: &Vec3) -> RcMat4 {
        let rx = Self::rotation_x(euler.x);
        let ry = Self::rotation_y(euler.y);
        let rz = Self::rotation_z(euler.z);
        // XYZ extrinsic: result = Rz * Ry * Rx
        let zy = rz.mul_mat_value(&ry);
        new_rc_type!(zy.mul_mat_value(&rx))
    }

    pub fn from_axis_angle(axis: &Vec3, deg: f32) -> RcMat4 {
        Self::from_rows(Self::from_axis_angle_value(axis, deg).data)
    }

    pub(crate) fn from_axis_angle_value(axis: &Vec3, deg: f32) -> Self {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        let one_minus_c = 1.0 - c;
        let len_sq = axis.length_squared();
        let len = len_sq.sqrt();
        if len < 1e-12 {
            return Self::identity_value();
        }
        let (x, y, z) = if let Some(len) = axis.wide_length(len_sq) {
            (
                (f64::from(axis.x) / len) as f32,
                (f64::from(axis.y) / len) as f32,
                (f64::from(axis.z) / len) as f32,
            )
        } else {
            (axis.x / len, axis.y / len, axis.z / len)
        };

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
        new_rc_type!(Self::from_scale_value(scale))
    }

    fn from_scale_value(scale: &Vec3) -> Self {
        Self {
            data: [
                [scale.x, 0.0, 0.0, 0.0],
                [0.0, scale.y, 0.0, 0.0],
                [0.0, 0.0, scale.z, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
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
        let f = if !rc_ref!(&f_rc).is_finite() && eye.is_finite() && target.is_finite() {
            let (x, y, z) = (
                f64::from(target.x) - f64::from(eye.x),
                f64::from(target.y) - f64::from(eye.y),
                f64::from(target.z) - f64::from(eye.z),
            );
            let len = (x * x + y * y + z * z).sqrt();
            Vec3::new((x / len) as f32, (y / len) as f32, (z / len) as f32)
        } else {
            rc_ref!(&f_rc).normalize()
        };
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
        let s = if !rc_ref!(&s_rc).is_finite() && f.is_finite() && up.is_finite() {
            let right = f.cross_normalized_wide(up);
            Vec3::new(right.x, right.y, right.z)
        } else {
            rc_ref!(&s_rc).normalize()
        };
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
        let t = Self::from_translation_value(v);
        self.mul_mat(&t)
    }

    pub fn rotate(&self, axis: &Vec3, deg: f32) -> RcMat4 {
        let r = Self::from_axis_angle_value(axis, deg);
        self.mul_mat(&r)
    }

    pub fn rotate_x(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_x(deg);
        self.mul_mat(&r)
    }

    pub fn rotate_y(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_y(deg);
        self.mul_mat(&r)
    }

    pub fn rotate_z(&self, deg: f32) -> RcMat4 {
        let r = Self::rotation_z(deg);
        self.mul_mat(&r)
    }

    pub fn scale_by(&self, v: &Vec3) -> RcMat4 {
        let s = Self::from_scale_value(v);
        self.mul_mat(&s)
    }

    // Matrix operations

    pub fn inverse(&self) -> RcMat4 {
        Self::from_rows(self.inverse_value().data)
    }

    #[must_use]
    pub fn inverse_value(&self) -> Self {
        self.try_inverse_value()
            .unwrap_or_else(Self::identity_value)
    }

    // Use wider intermediates for non-affine matrices, determinant cancellation,
    // or f32 range loss. Matrix magnitude alone does not establish rank loss.
    pub(crate) fn try_inverse_value(&self) -> Option<Self> {
        let m = &self.data;
        if m[3] != [0.0, 0.0, 0.0, 1.0] {
            return self.inverse_wide();
        }
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

        // Bound cancellation in the affine determinant's six signed products.
        // The 16-epsilon margin covers their f32 products and sums; unlike an
        // absolute cutoff, it scales with the matrix and only selects precision.
        let a = m.map(|row| row.map(f64::from));
        let determinant_bound = a[0][0].abs()
            * ((a[1][1] * a[2][2]).abs() + (a[1][2] * a[2][1]).abs())
            + a[0][1].abs() * ((a[1][0] * a[2][2]).abs() + (a[1][2] * a[2][0]).abs())
            + a[0][2].abs() * ((a[1][0] * a[2][1]).abs() + (a[1][1] * a[2][0]).abs());
        if det.is_normal()
            && f64::from(det).abs() > 16.0 * f64::from(f32::EPSILON) * determinant_bound
        {
            let inv_det = 1.0 / det;
            for row in &mut inv {
                for value in row {
                    *value *= inv_det;
                }
            }
            if inv.iter().flatten().all(|value| value.is_finite()) {
                return Some(Self { data: inv });
            }
        }

        self.inverse_wide()
    }

    fn inverse_wide(&self) -> Option<Self> {
        let mut a = self.data.map(|row| row.map(f64::from));
        let mut inverse = Self::identity_value().data.map(|row| row.map(f64::from));
        let mut scales = [0.0_f64; 4];
        for i in 0..4 {
            for value in a[i] {
                if !value.is_finite() {
                    return None;
                }
                scales[i] = scales[i].max(value.abs());
            }
            if scales[i] == 0.0 {
                return None;
            }
        }

        if self.has_zero_determinant() {
            return None;
        }

        // Scaled partial pivoting compares rows independently of their units.
        // A nonzero pivot is retained even for a small but invertible scale.
        for col in 0..4 {
            let mut pivot = col;
            for row in col + 1..4 {
                if a[row][col].abs() / scales[row] > a[pivot][col].abs() / scales[pivot] {
                    pivot = row;
                }
            }
            if a[pivot][col] == 0.0 {
                return None;
            }

            a.swap(col, pivot);
            inverse.swap(col, pivot);
            scales.swap(col, pivot);
            let divisor = a[col][col];
            for i in 0..4 {
                a[col][i] /= divisor;
                inverse[col][i] /= divisor;
            }

            for row in 0..4 {
                if row == col {
                    continue;
                }
                let factor = a[row][col];
                for i in 0..4 {
                    a[row][i] -= factor * a[col][i];
                    inverse[row][i] -= factor * inverse[col][i];
                }
            }
        }

        let data = inverse.map(|row| row.map(|value| value as f32));
        data.iter()
            .flatten()
            .all(|value| value.is_finite())
            .then_some(Self { data })
    }

    // Preserve exact rank loss without a tolerance that rejects small independent axes.
    // Products of four finite f32 values fit f64 range. The first pair is exact;
    // FMA residuals and error-free sums retain all 24 determinant terms (at most 96 parts).
    fn has_zero_determinant(&self) -> bool {
        let m = self.data.map(|row| row.map(f64::from));
        let mut expansion = [0.0_f64; 96];
        let mut len = 0;
        for i in 0..4 {
            for j in 0..4 {
                if j == i {
                    continue;
                }
                for k in 0..4 {
                    if k == i || k == j {
                        continue;
                    }
                    let l = 6 - i - j - k;
                    let [a, b, c, d] = [m[0][i], m[1][j], m[2][k], m[3][l]];
                    if a == 0.0 || b == 0.0 || c == 0.0 || d == 0.0 {
                        continue;
                    }
                    let negative = !((i > j) as u8
                        + (i > k) as u8
                        + (i > l) as u8
                        + (j > k) as u8
                        + (j > l) as u8
                        + (k > l) as u8)
                        .is_multiple_of(2);
                    let ab = a * b;
                    let abc = ab * c;
                    let abc_error = ab.mul_add(c, -abc);
                    let high = abc * d;
                    let low = abc_error * d;
                    for term in [abc_error.mul_add(d, -low), low, abc.mul_add(d, -high), high] {
                        if term == 0.0 {
                            continue;
                        }
                        let mut sum = if negative { -term } else { term };
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
                        if sum != 0.0 {
                            expansion[next_len] = sum;
                            next_len += 1;
                        }
                        len = next_len;
                    }
                }
            }
        }
        len == 0
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
        macro_rules! determinant {
            ($m:expr) => {{
                let m = $m;
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
            }};
        }

        // Four-factor terms and their sums stay within f32 range in this band.
        // Outside it, a lost minor can corrupt even a representable determinant.
        let needs_wide = self.data.iter().flatten().any(|v| {
            let magnitude = v.abs();
            magnitude != 0.0 && !(f32::powi(2.0, -31)..=f32::powi(2.0, 30)).contains(&magnitude)
        });
        if needs_wide && self.data.iter().flatten().all(|v| v.is_finite()) {
            let m = self.data.map(|row| row.map(f64::from));
            return determinant!(&m) as f32;
        }
        determinant!(&self.data)
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

    fn rotation_x(deg: f32) -> Self {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, c, -s, 0.0],
                [0.0, s, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn rotation_y(deg: f32) -> Self {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self {
            data: [
                [c, 0.0, s, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [-s, 0.0, c, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn rotation_z(deg: f32) -> Self {
        let r = deg.to_radians();
        let c = r.cos();
        let s = r.sin();
        Self {
            data: [
                [c, -s, 0.0, 0.0],
                [s, c, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
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
    fn test_determinant_retains_mixed_axis_scales() {
        let small = 2.0_f32.powi(-80);
        let large = 2.0_f32.powi(80);
        for large_axis in 0..3 {
            let mut axes = [small; 3];
            axes[large_axis] = large;
            let matrix = Mat4::from_scale(&Vec3 {
                x: axes[0],
                y: axes[1],
                z: axes[2],
            });
            // The diagonal product is exactly 2^-80 in every axis order.
            assert_eq!(rc_ref!(&matrix).determinant(), small);
        }
    }

    #[test]
    fn test_determinant_of_collapsed_axis_is_zero_at_finite_scales() {
        for exponent in [-80, 0, 80] {
            for zero_axis in 0..3 {
                let mut axes = [2.0_f32.powi(exponent); 3];
                axes[zero_axis] = 0.0;
                let matrix = Mat4::from_scale(&Vec3 {
                    x: axes[0],
                    y: axes[1],
                    z: axes[2],
                });
                assert_eq!(rc_ref!(&matrix).determinant(), 0.0);
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
    fn test_try_inverse_distinguishes_singular_from_extreme_axis_scales() {
        for scale in [1e-20, 1e20] {
            let matrix = deref(&Mat4::from_scale(&Vec3 {
                x: scale,
                y: -scale,
                z: scale,
            }));
            let inverse = matrix.try_inverse_value().unwrap();
            assert!(approx_eq_mat(
                &matrix.mul_mat_value(&inverse),
                &Mat4::identity_value()
            ));
        }

        let singular = Mat4 {
            data: [
                [1.0, 2.0, 3.0, 4.0],
                [2.0, 4.0, 6.0, 8.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        assert!(singular.try_inverse_value().is_none());
        assert_eq!(singular.inverse_value(), Mat4::identity_value());
    }

    #[test]
    fn test_inverse_rejects_exact_row_sum_without_rejecting_nearby_full_rank() {
        let mut matrix = Mat4 {
            data: [
                [1.0, 1.0, 2.0, 0.0],
                [2.0, 3.0, 1.0, 0.0],
                [3.0, 4.0, 3.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        };
        // Exact row addition establishes singularity without relying on an inverse algorithm.
        for col in 0..4 {
            assert_eq!(
                matrix.data[2][col],
                matrix.data[0][col] + matrix.data[1][col]
            );
        }
        assert!(matrix.try_inverse_value().is_none());
        assert_eq!(matrix.inverse_value(), Mat4::identity_value());

        matrix.data[2][2] = f32::from_bits(3.0_f32.to_bits() + 1);
        // One stored-f32 step breaks the row dependency: det = 2^-22, not zero.
        assert!(matrix.try_inverse_value().is_some());
    }

    #[test]
    fn test_inverse_retries_when_f32_determinant_hides_dependent_rows() {
        let rotation = deref(&Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 0.0,
            z: 45.0,
        }));
        let scale = deref(&Mat4::from_scale(&Vec3 {
            x: 0.0,
            y: 100.0,
            z: 333.0,
        }));
        let local = deref(&Mat4::from_euler(&Vec3 {
            x: 0.0,
            y: 33.0,
            z: 72.0,
        }));
        let singular = rotation.mul_mat_value(&scale).mul_mat_value(&local);
        // Opposite rows establish rank loss independently of a computed determinant.
        for col in 0..4 {
            assert_eq!(singular.data[0][col], -singular.data[1][col]);
        }
        assert!(singular.try_inverse_value().is_none());
        assert_eq!(singular.inverse_value(), Mat4::identity_value());
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
