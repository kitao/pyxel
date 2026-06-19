use crate::cube::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionInterpolation {
    Step,
    Linear,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MotionValues {
    Translations(Vec<Vec3>),
    Rotations(Vec<Quat>),
    Scales(Vec<Vec3>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MotionTarget {
    Translation,
    Rotation,
    Scale,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MotionChannel {
    pub part_index: usize,
    pub target: MotionTarget,
    pub inputs: Vec<f32>,
    pub values: MotionValues,
    pub interpolation: MotionInterpolation,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Motion {
    pub name: String,
    pub length: f32,
    pub base_transforms: Vec<Mat4>,
    pub channels: Vec<MotionChannel>,
}

define_rc_type!(RcMotion, Motion);

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
                    *rc_ref!(&base.pos()),
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
                    let transform = *rc_ref!(&Mat4::compose(&pos, &rot, &scale));
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

impl MotionChannel {
    fn is_usable(&self) -> bool {
        self.inputs.len().min(self.value_len()) > 0
            && matches!(
                (&self.target, &self.values),
                (MotionTarget::Translation, MotionValues::Translations(_))
                    | (MotionTarget::Rotation, MotionValues::Rotations(_))
                    | (MotionTarget::Scale, MotionValues::Scales(_))
            )
    }

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
                let t = if end != start {
                    ((frame - start) / (end - start)).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                return Some((index, index + 1, t));
            }
        }

        Some((key_count - 1, key_count - 1, 0.0))
    }

    fn sample_vec3(&self, frame: f32) -> Vec3 {
        let (MotionValues::Translations(values) | MotionValues::Scales(values)) = &self.values
        else {
            return zero_vec3();
        };
        self.sample_vec3_values(frame, values)
    }

    fn sample_quat(&self, frame: f32) -> Quat {
        let MotionValues::Rotations(values) = &self.values else {
            return identity_quat();
        };
        let Some((from, to, t)) = self.key_span(frame) else {
            return identity_quat();
        };
        let (Some(from), Some(to)) = (values.get(from), values.get(to)) else {
            return identity_quat();
        };
        match self.interpolation {
            MotionInterpolation::Step => *from,
            MotionInterpolation::Linear => *rc_ref!(&from.slerp(to, t)),
        }
    }

    fn value_len(&self) -> usize {
        match &self.values {
            MotionValues::Translations(values) | MotionValues::Scales(values) => values.len(),
            MotionValues::Rotations(values) => values.len(),
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
            MotionInterpolation::Step => *from,
            MotionInterpolation::Linear => Vec3 {
                x: from.x + (to.x - from.x) * t,
                y: from.y + (to.y - from.y) * t,
                z: from.z + (to.z - from.z) * t,
            },
        }
    }
}

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
    fn mismatched_channel_value_type_is_skipped() {
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
    fn linear_translation_sampling_interpolates_midpoint() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 15.0, false) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn step_translation_sampling_holds_previous_key() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Step)],
        };

        assert!((sampled_x(&motion, 15.0, false) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn non_looping_frame_clamps_to_end() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 99.0, false) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn looping_frame_wraps_into_motion_length() {
        let motion = Motion {
            name: String::from("move"),
            length: 30.0,
            base_transforms: vec![Mat4::identity_value()],
            channels: vec![translation_channel(MotionInterpolation::Linear)],
        };

        assert!((sampled_x(&motion, 45.0, true) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn scale_sampling_interpolates_midpoint() {
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
    fn rotation_sampling_slerps_midpoint() {
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
        assert!((axis.y.abs() - 1.0).abs() < 1e-5);
        assert!((angle - 45.0).abs() < 1e-4);
    }
}
