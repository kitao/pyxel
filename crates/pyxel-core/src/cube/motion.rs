use crate::cube::mat4::Mat4;
use crate::cube::quat::Quat;
use crate::cube::vec3::Vec3;

// Keyframe interpolation mode (the glTF sampler modes cube supports).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionInterpolation {
    CubicSpline,
    Step,
    Linear,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicVec3Key {
    pub in_tangent: Vec3,
    pub value: Vec3,
    pub out_tangent: Vec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicQuatKey {
    pub in_tangent: Quat,
    pub value: Quat,
    pub out_tangent: Quat,
}

// Keyframe payload of a channel. The variant must match the channel's
// target; sampling skips mismatched channels.

#[derive(Clone, Debug, PartialEq)]
pub enum MotionValues {
    CubicTranslations(Vec<CubicVec3Key>),
    CubicRotations(Vec<CubicQuatKey>),
    CubicScales(Vec<CubicVec3Key>),
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
}

// Transform component a channel animates.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionTarget {
    Translation,
    Rotation,
    Scale,
}

// Keyframe track animating one transform component of one mesh part.
// `inputs` holds the key times in Pyxel frames (glTF seconds × fps at
// import).

#[derive(Clone, Debug, PartialEq)]
pub struct MotionChannel {
    pub part_index: usize,
    pub target: MotionTarget,
    pub inputs: Vec<f32>,
    pub values: MotionValues,
    pub interpolation: MotionInterpolation,
}

// Animation clip imported from a GLB: per-part base transforms plus the
// keyframe channels that override them while the clip plays.

#[derive(Clone, Debug, PartialEq)]
pub struct Motion {
    pub name: String,
    pub length: f32,
    pub base_transforms: Vec<Mat4>,
    pub channels: Vec<MotionChannel>,
}

define_rc_type!(RcMotion, Motion);

// Clip sampling

impl Motion {
    pub fn new(name: String, length: f32, base_transforms: Vec<Mat4>) -> RcMotion {
        new_rc_type!(Self {
            name,
            length,
            base_transforms,
            channels: Vec::new(),
        })
    }

    pub fn sample(&self, frame: f32, looping: bool) -> Vec<(usize, Mat4)> {
        let frame = self.resolve_frame(frame, looping);
        let mut sampled_parts: Vec<Option<(Vec3, Quat, Vec3)>> =
            vec![None; self.base_transforms.len()];

        for channel in &self.channels {
            if !channel.is_usable() {
                continue;
            }
            let Some(base) = self.base_transforms.get(channel.part_index) else {
                continue;
            };
            let (pos, rot, scale) = sampled_parts[channel.part_index].get_or_insert_with(|| {
                (
                    base.pos_value(),
                    *rc_ref!(&base.rot()),
                    *rc_ref!(&base.scale_vec()),
                )
            });
            match channel.target {
                MotionTarget::Translation => *pos = channel.sample_vec3(frame),
                MotionTarget::Rotation => *rot = channel.sample_quat(frame),
                MotionTarget::Scale => *scale = channel.sample_vec3(frame),
            }
        }

        sampled_parts
            .into_iter()
            .enumerate()
            .filter_map(|(part_index, components)| {
                components.map(|(pos, rot, scale)| {
                    let transform = compose_value(&pos, &rot, &scale);
                    (part_index, transform)
                })
            })
            .collect()
    }

    pub fn resolve_frame(&self, frame: f32, looping: bool) -> f32 {
        if self.length <= 0.0 {
            return 0.0;
        }
        if looping {
            frame.rem_euclid(self.length)
        } else {
            frame.clamp(0.0, self.length)
        }
    }
}

// Channel keyframe sampling

impl MotionChannel {
    fn is_usable(&self) -> bool {
        self.inputs.len().min(self.value_len()) > 0
            && matches!(
                (&self.target, &self.values),
                (
                    MotionTarget::Translation,
                    MotionValues::Translations(_) | MotionValues::CubicTranslations(_),
                ) | (
                    MotionTarget::Rotation,
                    MotionValues::Rotations(_) | MotionValues::CubicRotations(_),
                ) | (
                    MotionTarget::Scale,
                    MotionValues::Scales(_) | MotionValues::CubicScales(_),
                )
            )
    }

    // Assumes `inputs` ascend — the glTF sampler contract, upheld by
    // glb_parser (the only channel producer).
    fn key_span(&self, frame: f32) -> Option<(usize, usize, f32)> {
        let key_count = self.inputs.len().min(self.value_len());
        if key_count == 0 {
            return None;
        }
        if key_count == 1 || frame <= self.inputs[0] {
            return Some((0, 0, 0.0));
        }

        for index in 0..(key_count - 1) {
            let start = self.inputs[index];
            let end = self.inputs[index + 1];
            if frame < end {
                let t = if end == start {
                    0.0
                } else {
                    ((frame - start) / (end - start)).clamp(0.0, 1.0)
                };
                return Some((index, index + 1, t));
            }
        }

        Some((key_count - 1, key_count - 1, 0.0))
    }

    fn sample_vec3(&self, frame: f32) -> Vec3 {
        match &self.values {
            MotionValues::Translations(values) | MotionValues::Scales(values) => {
                self.sample_vec3_values(frame, values)
            }
            MotionValues::CubicTranslations(keys) | MotionValues::CubicScales(keys) => {
                self.sample_cubic_vec3_keys(frame, keys)
            }
            MotionValues::Rotations(_) | MotionValues::CubicRotations(_) => zero_vec3(),
        }
    }

    fn sample_quat(&self, frame: f32) -> Quat {
        match &self.values {
            MotionValues::Rotations(values) => self.sample_quat_values(frame, values),
            MotionValues::CubicRotations(keys) => self.sample_cubic_quat_keys(frame, keys),
            MotionValues::Translations(_)
            | MotionValues::CubicTranslations(_)
            | MotionValues::Scales(_)
            | MotionValues::CubicScales(_) => identity_quat(),
        }
    }

    fn value_len(&self) -> usize {
        match &self.values {
            MotionValues::Translations(values) | MotionValues::Scales(values) => values.len(),
            MotionValues::Rotations(values) => values.len(),
            MotionValues::CubicTranslations(values) | MotionValues::CubicScales(values) => {
                values.len()
            }
            MotionValues::CubicRotations(values) => values.len(),
        }
    }

    fn sample_cubic_vec3_keys(&self, frame: f32, keys: &[CubicVec3Key]) -> Vec3 {
        let Some((from, to, t)) = self.key_span(frame) else {
            return zero_vec3();
        };
        let (Some(from_key), Some(to_key)) = (keys.get(from), keys.get(to)) else {
            return zero_vec3();
        };
        if from == to {
            return from_key.value;
        }
        let dt = self.inputs[to] - self.inputs[from];
        cubic_vec3(
            &from_key.value,
            &from_key.out_tangent,
            &to_key.value,
            &to_key.in_tangent,
            t,
            dt,
        )
    }

    fn sample_cubic_quat_keys(&self, frame: f32, keys: &[CubicQuatKey]) -> Quat {
        let Some((from, to, t)) = self.key_span(frame) else {
            return identity_quat();
        };
        let (Some(from_key), Some(to_key)) = (keys.get(from), keys.get(to)) else {
            return identity_quat();
        };
        if from == to {
            return from_key.value;
        }
        let dt = self.inputs[to] - self.inputs[from];
        cubic_quat(
            &from_key.value,
            &from_key.out_tangent,
            &to_key.value,
            &to_key.in_tangent,
            t,
            dt,
        )
    }

    fn sample_quat_values(&self, frame: f32, values: &[Quat]) -> Quat {
        let Some((from, to, t)) = self.key_span(frame) else {
            return identity_quat();
        };
        let (Some(from), Some(to)) = (values.get(from), values.get(to)) else {
            return identity_quat();
        };
        match self.interpolation {
            MotionInterpolation::CubicSpline | MotionInterpolation::Step => *from,
            MotionInterpolation::Linear => *rc_ref!(&from.slerp(to, t)),
        }
    }

    fn sample_vec3_values(&self, frame: f32, values: &[Vec3]) -> Vec3 {
        let Some((from, to, t)) = self.key_span(frame) else {
            return zero_vec3();
        };
        let (Some(from), Some(to)) = (values.get(from), values.get(to)) else {
            return zero_vec3();
        };
        match self.interpolation {
            MotionInterpolation::CubicSpline | MotionInterpolation::Step => *from,
            MotionInterpolation::Linear => Vec3 {
                x: from.x + (to.x - from.x) * t,
                y: from.y + (to.y - from.y) * t,
                z: from.z + (to.z - from.z) * t,
            },
        }
    }
}

fn cubic_basis(t: f32) -> (f32, f32, f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    (
        2.0 * t3 - 3.0 * t2 + 1.0,
        t3 - 2.0 * t2 + t,
        -2.0 * t3 + 3.0 * t2,
        t3 - t2,
    )
}

fn cubic_vec3(
    from: &Vec3,
    out_tangent: &Vec3,
    to: &Vec3,
    in_tangent: &Vec3,
    t: f32,
    dt: f32,
) -> Vec3 {
    let (h00, h10, h01, h11) = cubic_basis(t);
    Vec3 {
        x: h00 * from.x + h10 * dt * out_tangent.x + h01 * to.x + h11 * dt * in_tangent.x,
        y: h00 * from.y + h10 * dt * out_tangent.y + h01 * to.y + h11 * dt * in_tangent.y,
        z: h00 * from.z + h10 * dt * out_tangent.z + h01 * to.z + h11 * dt * in_tangent.z,
    }
}

fn cubic_quat(
    from: &Quat,
    out_tangent: &Quat,
    to: &Quat,
    in_tangent: &Quat,
    t: f32,
    dt: f32,
) -> Quat {
    let (h00, h10, h01, h11) = cubic_basis(t);
    let quat = Quat {
        x: h00 * from.x + h10 * dt * out_tangent.x + h01 * to.x + h11 * dt * in_tangent.x,
        y: h00 * from.y + h10 * dt * out_tangent.y + h01 * to.y + h11 * dt * in_tangent.y,
        z: h00 * from.z + h10 * dt * out_tangent.z + h01 * to.z + h11 * dt * in_tangent.z,
        w: h00 * from.w + h10 * dt * out_tangent.w + h01 * to.w + h11 * dt * in_tangent.w,
    };
    *rc_ref!(&quat.normalize())
}

// Non-Rc transform composition for the per-frame sampling path

// Mirrors Mat4::compose (T * R * S) through the value-typed operator
// cores, computing a bit-identical transform without the factory
// chain's Rc temporaries. The quaternion-to-matrix factor keeps its Rc:
// Quat exposes no value-typed core to build on.
fn compose_value(pos: &Vec3, rot: &Quat, scale: &Vec3) -> Mat4 {
    let mut t = Mat4::identity_value();
    t.data[0][3] = pos.x;
    t.data[1][3] = pos.y;
    t.data[2][3] = pos.z;
    let r = *rc_ref!(&rot.to_matrix());
    let mut s = Mat4::identity_value();
    s.data[0][0] = scale.x;
    s.data[1][1] = scale.y;
    s.data[2][2] = scale.z;
    t.mul_mat_value(&r).mul_mat_value(&s)
}

// Fallback value helpers

fn zero_vec3() -> Vec3 {
    Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    }
}

fn identity_quat() -> Quat {
    Quat {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn translation_channel(interpolation: MotionInterpolation) -> MotionChannel {
        MotionChannel {
            part_index: 0,
            target: MotionTarget::Translation,
            inputs: vec![0.0, 30.0],
            values: MotionValues::Translations(vec![
                Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vec3 {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ]),
            interpolation,
        }
    }

    fn cubic_translation_channel() -> MotionChannel {
        let per_frame = Vec3 {
            x: 1.0 / 30.0,
            y: 0.0,
            z: 0.0,
        };
        MotionChannel {
            part_index: 0,
            target: MotionTarget::Translation,
            inputs: vec![0.0, 30.0],
            values: MotionValues::CubicTranslations(vec![
                CubicVec3Key {
                    in_tangent: per_frame,
                    value: zero_vec3(),
                    out_tangent: per_frame,
                },
                CubicVec3Key {
                    in_tangent: per_frame,
                    value: Vec3 {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    out_tangent: per_frame,
                },
            ]),
            interpolation: MotionInterpolation::CubicSpline,
        }
    }

    fn sampled_x(motion: &Motion, frame: f32, looping: bool) -> f32 {
        let sampled = motion.sample(frame, looping);
        let (_, mat) = sampled
            .iter()
            .find(|(part_index, _)| *part_index == 0)
            .expect("part 0 should be sampled");
        rc_ref!(&mat.pos()).x
    }

    fn sampled_part_transform(motion: &Motion, frame: f32, looping: bool) -> Mat4 {
        let sampled = motion.sample(frame, looping);
        sampled
            .iter()
            .find(|(part_index, _)| *part_index == 0)
            .map(|(_, mat)| *mat)
            .expect("part 0 should be sampled")
    }

    #[test]
    fn test_mismatched_channel_value_type_is_skipped() {
        let base = *rc_ref!(&Mat4::from_translation(&Vec3 {
            x: 2.0,
            y: 0.0,
            z: 0.0,
        }));
        let motion = Motion {
            name: String::from("malformed"),
            length: 30.0,
            base_transforms: vec![base],
            channels: vec![MotionChannel {
                part_index: 0,
                target: MotionTarget::Translation,
                inputs: vec![0.0, 30.0],
                values: MotionValues::Rotations(vec![identity_quat(), identity_quat()]),
                interpolation: MotionInterpolation::Linear,
            }],
        };

        assert!(motion.sample(15.0, false).is_empty());
    }

    #[test]
    fn test_linear_translation_sampling_interpolates_midpoint() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 15.0, false) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_step_translation_sampling_holds_previous_key() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Step)],
        };

        assert!((sampled_x(&motion, 15.0, false) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_cubic_translation_sampling_uses_frame_unit_tangents() {
        let motion = Motion {
            name: String::from("smooth_move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![cubic_translation_channel()],
        };

        assert!((sampled_x(&motion, 7.5, false) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_non_looping_frame_clamps_to_end() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 99.0, false) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_looping_frame_wraps_into_motion_length() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 45.0, true) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_scale_sampling_interpolates_midpoint() {
        let motion = Motion {
            name: String::from("scale"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![MotionChannel {
                part_index: 0,
                target: MotionTarget::Scale,
                inputs: vec![0.0, 30.0],
                values: MotionValues::Scales(vec![
                    Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
                    Vec3 {
                        x: 3.0,
                        y: 5.0,
                        z: 7.0,
                    },
                ]),
                interpolation: MotionInterpolation::Linear,
            }],
        };

        let sampled = sampled_part_transform(&motion, 15.0, false);
        let scale_rc = sampled.scale_vec();
        let scale = rc_ref!(&scale_rc);
        assert!((scale.x - 2.0).abs() < 1e-6);
        assert!((scale.y - 3.0).abs() < 1e-6);
        assert!((scale.z - 4.0).abs() < 1e-6);
    }

    #[test]
    fn test_rotation_sampling_slerps_midpoint() {
        let y_axis = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let motion = Motion {
            name: String::from("rotate"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![MotionChannel {
                part_index: 0,
                target: MotionTarget::Rotation,
                inputs: vec![0.0, 30.0],
                values: MotionValues::Rotations(vec![
                    identity_quat(),
                    *rc_ref!(&Quat::from_axis_angle(&y_axis, 90.0)),
                ]),
                interpolation: MotionInterpolation::Linear,
            }],
        };

        let sampled = sampled_part_transform(&motion, 15.0, false);
        let rot = sampled.rot();
        let (axis, angle) = rc_ref!(&rot).to_axis_angle();
        let axis = rc_ref!(&axis);
        // The midpoint is deterministically +Y (w = cos 22.5° > 0, so
        // to_axis_angle cannot flip the axis to -Y).
        assert!((axis.y - 1.0).abs() < 1e-5);
        assert!((angle - 45.0).abs() < 1e-4);
    }
}
