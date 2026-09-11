use crate::cube::mat4::Mat4;
use crate::cube::quat::Quat;
use crate::cube::vec3::Vec3;

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

// Sampling skips values whose variant does not match the channel target.
#[derive(Clone, Debug, PartialEq)]
pub enum MotionValues {
    CubicTranslations(Vec<CubicVec3Key>),
    CubicRotations(Vec<CubicQuatKey>),
    CubicScales(Vec<CubicVec3Key>),
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
}

impl MotionValues {
    pub(crate) fn len(&self) -> usize {
        match self {
            Self::Translations(values) | Self::Scales(values) => values.len(),
            Self::Rotations(values) => values.len(),
            Self::CubicTranslations(values) | Self::CubicScales(values) => values.len(),
            Self::CubicRotations(values) => values.len(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionTarget {
    Translation,
    Rotation,
    Scale,
}

// Key times in inputs are Pyxel frames, converted from glTF seconds at import.
#[derive(Clone, Debug, PartialEq)]
pub struct MotionChannel {
    pub part_index: usize,
    pub target: MotionTarget,
    pub inputs: Vec<f32>,
    pub values: MotionValues,
    pub interpolation: MotionInterpolation,
}

// Retain authored TRS components: matrices lose scale signs and zero-scale rotation.
// Channels override these per-part components while the clip plays.
#[derive(Clone, Debug, PartialEq)]
pub struct Motion {
    pub name: String,
    pub length: f32,
    pub base_components: Vec<(Vec3, Quat, Vec3)>,
    pub channels: Vec<MotionChannel>,
}

define_rc_type!(RcMotion, Motion);

impl Motion {
    pub fn new(name: String, length: f32, base_components: Vec<(Vec3, Quat, Vec3)>) -> RcMotion {
        new_rc_type!(Self {
            name,
            length,
            base_components,
            channels: Vec::new(),
        })
    }

    pub fn sample(&self, frame: f32, looping: bool) -> Vec<(usize, Mat4)> {
        let frame = self.resolve_frame(frame, looping);
        let mut sampled_parts: Vec<Option<(Vec3, Quat, Vec3)>> =
            vec![None; self.base_components.len()];

        for channel in &self.channels {
            if !channel.is_usable() {
                continue;
            }
            let Some(base) = self.base_components.get(channel.part_index) else {
                continue;
            };

            let (pos, rot, scale) = sampled_parts[channel.part_index].get_or_insert(*base);
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
                    let transform = Mat4::compose_value(&pos, &rot, &scale);
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
        self.inputs.len().min(self.values.len()) > 0
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
        let key_count = self.inputs.len().min(self.values.len());
        if key_count == 0 {
            return None;
        }
        if key_count == 1 || frame <= self.inputs[0] {
            return Some((0, 0, 0.0));
        }
        if frame.is_nan() {
            return Some((key_count - 1, key_count - 1, 0.0));
        }

        let to = self.inputs[..key_count].partition_point(|&time| time <= frame);
        if to == key_count {
            return Some((key_count - 1, key_count - 1, 0.0));
        }

        let from = to - 1;
        let start = self.inputs[from];
        let end = self.inputs[to];
        let t = if end == start {
            0.0
        } else {
            ((frame - start) / (end - start)).clamp(0.0, 1.0)
        };
        Some((from, to, t))
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
            MotionInterpolation::Linear => from.slerp_value(to, t),
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
    quat.normalize_value()
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

    fn identity_components() -> (Vec3, Quat, Vec3) {
        (
            zero_vec3(),
            identity_quat(),
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        )
    }

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
    fn test_cubic_rotation_normalizes_midpoint() {
        let zero = Quat {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        let channel = MotionChannel {
            part_index: 0,
            target: MotionTarget::Rotation,
            inputs: vec![0.0, 2.0],
            values: MotionValues::CubicRotations(vec![
                CubicQuatKey {
                    in_tangent: zero,
                    value: identity_quat(),
                    out_tangent: zero,
                },
                CubicQuatKey {
                    in_tangent: zero,
                    value: Quat {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                        w: 0.0,
                    },
                    out_tangent: zero,
                },
            ]),
            interpolation: MotionInterpolation::CubicSpline,
        };

        let q = channel.sample_quat(1.0);
        assert_eq!((q.x, q.z), (0.0, 0.0));
        assert!((q.y - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6);
        assert!((q.w - std::f32::consts::FRAC_1_SQRT_2).abs() < 1e-6);
    }

    #[test]
    fn test_key_span_handles_boundaries_and_non_finite_frames() {
        let mut channel = translation_channel(MotionInterpolation::Linear);
        channel.inputs = vec![0.0, 10.0, 20.0, 30.0];
        channel.values = MotionValues::Translations(vec![zero_vec3(); 4]);

        assert_eq!(channel.key_span(f32::NEG_INFINITY), Some((0, 0, 0.0)));
        assert_eq!(channel.key_span(10.0), Some((1, 2, 0.0)));
        assert_eq!(channel.key_span(15.0), Some((1, 2, 0.5)));
        assert_eq!(channel.key_span(30.0), Some((3, 3, 0.0)));
        assert_eq!(channel.key_span(f32::INFINITY), Some((3, 3, 0.0)));
        assert_eq!(channel.key_span(f32::NAN), Some((3, 3, 0.0)));
    }

    #[test]
    fn test_mismatched_channel_value_type_is_skipped() {
        let mut base = identity_components();
        base.0.x = 2.0;
        let motion = Motion {
            name: String::from("malformed"),
            length: 30.0,
            base_components: vec![base],
            channels: vec![MotionChannel {
                part_index: 0,
                target: MotionTarget::Translation,
                inputs: vec![0.0, 30.0],
                values: MotionValues::Rotations(vec![identity_quat(), identity_quat()]),
                interpolation: MotionInterpolation::Linear,
            }],
        };

        assert_eq!(motion.sample(15.0, false), [] as [(usize, Mat4); 0]);
    }

    #[test]
    fn test_linear_translation_sampling_interpolates_midpoint() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_components: vec![identity_components()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };
        assert_eq!(sampled_x(&motion, 15.0, false), 0.5);
    }

    #[test]
    fn test_step_translation_sampling_holds_previous_key() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_components: vec![identity_components()],
            channels: vec![translation_channel(MotionInterpolation::Step)],
        };
        assert_eq!(sampled_x(&motion, 15.0, false), 0.0);
    }

    #[test]
    fn test_cubic_translation_sampling_uses_frame_unit_tangents() {
        let motion = Motion {
            name: String::from("smooth_move"),
            length: 30.0,
            base_components: vec![identity_components()],
            channels: vec![cubic_translation_channel()],
        };
        assert!((sampled_x(&motion, 7.5, false) - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_non_looping_frame_clamps_to_end() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_components: vec![identity_components()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };
        assert_eq!(sampled_x(&motion, 99.0, false), 1.0);
    }

    #[test]
    fn test_looping_frame_wraps_into_motion_length() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_components: vec![identity_components()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };
        assert_eq!(sampled_x(&motion, 45.0, true), 0.5);
    }

    #[test]
    fn test_scale_sampling_interpolates_midpoint() {
        let motion = Motion {
            name: String::from("scale"),
            length: 30.0,
            base_components: vec![identity_components()],
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
        assert_eq!((scale.x, scale.y, scale.z), (2.0, 3.0, 4.0));
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
            base_components: vec![identity_components()],
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
