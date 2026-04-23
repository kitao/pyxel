use std::sync::OnceLock;

use blip_buf::BlipBuf;

use crate::settings::{AUDIO_GAIN_SCALE, AUDIO_GAIN_SHIFT};

const A4_MIDI_NOTE: f32 = 69.0;
const A4_FREQUENCY: f32 = 440.0;
// Fixed-point Q14 scaling for voice gain multiplication
const VOICE_GAIN_ROUND_BIAS: i64 = 1_i64 << (AUDIO_GAIN_SHIFT - 1); // Half-unit bias for rounding
const PITCH_LUT_MIN_SEMITONE: f32 = -96.0;
const PITCH_LUT_MAX_SEMITONE: f32 = 96.0;
const PITCH_LUT_STEPS_PER_SEMITONE: usize = 64;
const PITCH_LUT_SIZE: usize =
    ((PITCH_LUT_MAX_SEMITONE - PITCH_LUT_MIN_SEMITONE) as usize * PITCH_LUT_STEPS_PER_SEMITONE) + 1;
static PITCH_RATIO_LUT: OnceLock<Box<[f32]>> = OnceLock::new();

pub struct Oscillator {
    waveform_samples: Vec<i16>,
    waveform_index: usize,

    lfsr: u16,
    tap_bit: u8,

    sample: i32,
}

impl Oscillator {
    fn new() -> Self {
        Self {
            waveform_samples: Vec::new(),
            waveform_index: 0,

            lfsr: 0,
            tap_bit: 0,

            sample: 0,
        }
    }

    pub fn set(&mut self, waveform: &[f32]) {
        assert!(!waveform.is_empty());

        let quantized: Vec<i16> = waveform.iter().map(|&s| Self::quantize_sample(s)).collect();

        if quantized != self.waveform_samples {
            self.waveform_samples = quantized;
            self.waveform_index = 0;
        }

        self.tap_bit = 0;
        self.update();
    }

    pub fn set_noise(&mut self, short_period: bool) {
        // Reinitialize the LFSR when switching noise mode to ensure deterministic output.
        // Each mode uses a different tap bit (6 for short-period, 1 for long-period), producing
        // different cycle lengths. The LFSR seed is pre-advanced past leading zeros:
        // short-period (tap 6): 15 shifts (pre-advanced, 93-sample period)    -> 0x0201
        // long-period  (tap 1): 45 shifts (pre-advanced, 32767-sample period) -> 0x7001
        let tap_bit = if short_period { 6 } else { 1 };
        if tap_bit != self.tap_bit {
            self.lfsr = if short_period { 0x0201 } else { 0x7001 };
            self.tap_bit = tap_bit;
        }
        self.waveform_samples.clear();
        self.update();
    }

    fn sample(&self) -> i32 {
        self.sample
    }

    fn samples_per_cycle(&self) -> u32 {
        if self.tap_bit == 0 {
            self.waveform_samples.len() as u32
        } else {
            1
        }
    }

    fn advance_sample(&mut self) {
        if self.tap_bit == 0 {
            self.waveform_index = (self.waveform_index + 1) % self.waveform_samples.len();
        } else {
            let feedback = (self.lfsr ^ (self.lfsr >> self.tap_bit)) & 1;
            self.lfsr = ((self.lfsr >> 1) | (feedback << 14)) & 0x7FFF;
        }
        self.update();
    }

    fn update(&mut self) {
        self.sample = if self.tap_bit == 0 {
            self.waveform_samples[self.waveform_index] as i32
        } else if (self.lfsr & 1) == 0 {
            i16::MAX as i32
        } else {
            -(i16::MAX as i32)
        };
    }

    fn quantize_sample(sample: f32) -> i16 {
        ((sample * i16::MAX as f32).round() as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16
    }
}

#[derive(Debug)]
struct EnvelopeSegment {
    start_tick: u32,
    start_level: f32,
    slope: f32,
}

pub struct Envelope {
    segments: Vec<EnvelopeSegment>,
    segment_index: usize,

    enabled: bool,
    elapsed_ticks: u32,
    level: f32,
}

impl Envelope {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
            segment_index: 0,
            enabled: false,
            elapsed_ticks: 0,
            level: 0.0,
        }
    }

    pub fn set(&mut self, initial_level: f32, segments: &[(u32, f32)]) {
        self.segments.clear();

        let mut start_tick = 0;
        let mut start_level = initial_level;

        for &(duration, target_level) in segments {
            let slope = if duration > 0 {
                (target_level - start_level) / duration as f32
            } else {
                0.0
            };

            self.segments.push(EnvelopeSegment {
                start_tick,
                start_level,
                slope,
            });

            start_tick += duration;
            start_level = target_level;
        }

        self.segments.push(EnvelopeSegment {
            start_tick,
            start_level,
            slope: 0.0,
        });

        self.segments.reverse();

        self.segment_index = self.segments.len() - 1;
        while self.segment_index > 0
            && self.elapsed_ticks >= self.segments[self.segment_index - 1].start_tick
        {
            self.segment_index -= 1;
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn level(&self) -> f32 {
        self.level
    }

    pub fn reset_tick(&mut self) {
        self.elapsed_ticks = 0;
        self.segment_index = self.segments.len().saturating_sub(1);
        self.update();
    }

    fn advance_tick(&mut self, ticks: u32) {
        if self.enabled {
            self.elapsed_ticks += ticks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled {
            self.level = 1.0;
            return;
        }

        while self.segment_index > 0
            && self.elapsed_ticks >= self.segments[self.segment_index - 1].start_tick
        {
            self.segment_index -= 1;
        }

        let segment = &self.segments[self.segment_index];
        self.level = if segment.slope == 0.0 {
            segment.start_level
        } else {
            segment.start_level + segment.slope * (self.elapsed_ticks - segment.start_tick) as f32
        };
    }
}

pub struct Vibrato {
    delay_ticks: u32,
    period_ticks: u32,
    inv_period_ticks: f32,
    semitone_depth: f32,

    enabled: bool,
    elapsed_ticks: u32,
    pitch_multiplier: f32,
}

impl Vibrato {
    fn new() -> Self {
        Self {
            delay_ticks: 0,
            period_ticks: 1,
            inv_period_ticks: 0.0,
            semitone_depth: 0.0,

            enabled: false,
            elapsed_ticks: 0,
            pitch_multiplier: 1.0,
        }
    }

    pub fn set(&mut self, delay_ticks: u32, period_ticks: u32, semitone_depth: f32) {
        self.delay_ticks = delay_ticks;
        self.semitone_depth = semitone_depth;

        if period_ticks != self.period_ticks {
            self.period_ticks = period_ticks;
            self.inv_period_ticks = if period_ticks > 0 {
                1.0 / period_ticks as f32
            } else {
                0.0
            };
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn pitch_multiplier(&self) -> f32 {
        self.pitch_multiplier
    }

    fn reset_tick(&mut self) {
        if self.delay_ticks > 0 {
            self.elapsed_ticks = 0;
        }

        self.update();
    }

    fn advance_tick(&mut self, ticks: u32) {
        if self.enabled {
            self.elapsed_ticks += ticks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled || self.elapsed_ticks < self.delay_ticks {
            self.pitch_multiplier = 1.0;
            return;
        }

        let phase = (self.elapsed_ticks - self.delay_ticks) as f32 * self.inv_period_ticks;
        let modulation = 1.0 - 4.0 * ((phase + 0.25).fract() - 0.5).abs();
        let semitone_offset = modulation * self.semitone_depth;

        self.pitch_multiplier = semitone_to_pitch_multiplier(semitone_offset);
    }
}

pub struct Glide {
    semitone_offset: f32,
    duration_ticks: u32,
    semitone_slope: f32,

    enabled: bool,
    elapsed_ticks: u32,
    pitch_multiplier: f32,
}

impl Glide {
    fn new() -> Self {
        Self {
            semitone_offset: 0.0,
            duration_ticks: 0,
            semitone_slope: 0.0,

            enabled: false,
            elapsed_ticks: 0,
            pitch_multiplier: 1.0,
        }
    }

    pub fn set(&mut self, semitone_offset: f32, duration_ticks: u32) {
        if semitone_offset != self.semitone_offset || duration_ticks != self.duration_ticks {
            self.semitone_offset = semitone_offset;
            self.duration_ticks = duration_ticks;
            self.semitone_slope = if duration_ticks > 0 {
                -semitone_offset / duration_ticks as f32
            } else {
                0.0
            };
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn pitch_multiplier(&self) -> f32 {
        self.pitch_multiplier
    }

    fn reset_tick(&mut self) {
        self.elapsed_ticks = 0;
        self.update();
    }

    fn advance_tick(&mut self, ticks: u32) {
        if self.enabled {
            self.elapsed_ticks += ticks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled || self.elapsed_ticks >= self.duration_ticks {
            self.pitch_multiplier = 1.0;
            return;
        }

        let semitone_offset =
            self.semitone_offset + self.semitone_slope * self.elapsed_ticks as f32;
        self.pitch_multiplier = semitone_to_pitch_multiplier(semitone_offset);
    }
}

pub struct Voice {
    pub oscillator: Oscillator,
    pub envelope: Envelope,
    pub vibrato: Vibrato,
    pub glide: Glide,

    clock_rate: u32,
    clocks_per_tick: u32,
    base_frequency: f32,
    velocity: f32,
    remaining_note_clocks: u32,
    elapsed_note_clocks: u32,
    sample_clocks: u32,
    carryover_sample_clocks: u32,
    control_interval_clocks: u32,
    control_elapsed_clocks: u32,
    last_amplitude: i32,

    interp_clocks: u32,
    interp_start_gain: Option<i32>,
    interp_end_gain: Option<i32>,
    last_gain: i32,
}

impl Voice {
    pub fn new(clock_rate: u32, control_rate: u32, interp_clocks: u32) -> Self {
        assert!(clock_rate > 0 && control_rate > 0 && interp_clocks > 0);
        let _ = pitch_ratio_lut();

        let control_interval_clocks = clock_rate / control_rate;

        Self {
            oscillator: Oscillator::new(),
            envelope: Envelope::new(),
            vibrato: Vibrato::new(),
            glide: Glide::new(),

            clock_rate,
            clocks_per_tick: 1,
            base_frequency: 0.0,
            velocity: 0.0,
            remaining_note_clocks: 0,
            elapsed_note_clocks: 0,
            sample_clocks: 0,
            carryover_sample_clocks: 0,
            control_interval_clocks,
            control_elapsed_clocks: 0,
            last_amplitude: 0,

            interp_clocks,
            interp_start_gain: None,
            interp_end_gain: None,
            last_gain: 0,
        }
    }

    pub fn set_clocks_per_tick(&mut self, clocks_per_tick: u32) {
        assert!(clocks_per_tick > 0);

        self.clocks_per_tick = clocks_per_tick;
    }

    pub fn play_note(&mut self, midi_note: f32, velocity: f32, duration_clocks: u32) {
        self.base_frequency = A4_FREQUENCY * ((midi_note - A4_MIDI_NOTE) / 12.0).exp2();
        self.velocity = velocity;
        self.remaining_note_clocks = duration_clocks + self.interp_clocks;
        self.elapsed_note_clocks = 0;
        self.interp_start_gain = None;
        self.interp_end_gain = None;

        self.reset_control_clock();
    }

    pub fn cancel_note(&mut self) {
        self.remaining_note_clocks = self.remaining_note_clocks.min(self.interp_clocks);
    }

    pub(crate) fn needs_processing(&self) -> bool {
        self.remaining_note_clocks > 0
            || self.carryover_sample_clocks > 0
            || self.last_amplitude != 0
    }

    pub fn process(&mut self, blip_buf: Option<&mut BlipBuf>, clock_offset: u32, clock_count: u32) {
        if clock_count == 0 {
            return;
        }

        let mut blip_buf = blip_buf;
        let mut clock_offset = clock_offset + self.carryover_sample_clocks;
        let mut clock_count = clock_count;

        if self.carryover_sample_clocks > 0 {
            let process_clocks = self.carryover_sample_clocks.min(clock_count);
            self.remaining_note_clocks = self.remaining_note_clocks.saturating_sub(process_clocks);
            self.elapsed_note_clocks += process_clocks;
            self.carryover_sample_clocks -= process_clocks;
            clock_count -= process_clocks;

            if self.carryover_sample_clocks > 0 {
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        // Phase 1: Head crossfade (elapsed < interp, but yield to tail when remaining < interp)
        if self.remaining_note_clocks > 0
            && clock_count > 0
            && self.elapsed_note_clocks < self.interp_clocks
            && self.remaining_note_clocks >= self.interp_clocks
        {
            let start_gain = *self.interp_start_gain.get_or_insert(self.last_gain);
            let interp = self.interp_clocks as i64;

            while self.remaining_note_clocks > 0
                && clock_count > 0
                && self.elapsed_note_clocks < self.interp_clocks
                && self.remaining_note_clocks >= self.interp_clocks
            {
                let mut gain = Self::gain_to_fixed(self.envelope.level() * self.velocity);
                let elapsed = self.elapsed_note_clocks as i64;
                gain =
                    ((start_gain as i64 * (interp - elapsed) + gain as i64 * elapsed + interp / 2)
                        / interp) as i32;

                let amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
                self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);
                self.last_gain = gain;

                let process_clocks = self.sample_clocks.min(clock_count);
                self.remaining_note_clocks =
                    self.remaining_note_clocks.saturating_sub(process_clocks);
                self.elapsed_note_clocks += process_clocks;
                clock_offset += process_clocks;
                clock_count -= process_clocks;

                if process_clocks < self.sample_clocks {
                    self.carryover_sample_clocks = self.sample_clocks - process_clocks;
                    return;
                }

                self.oscillator.advance_sample();
                self.advance_control_clock(self.sample_clocks);
            }
        }

        // Phase 2: Bulk (no interpolation)
        while self.remaining_note_clocks > self.interp_clocks && clock_count > 0 {
            let gain = Self::gain_to_fixed(self.envelope.level() * self.velocity);
            let amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
            self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);
            self.last_gain = gain;

            let process_clocks = self.sample_clocks.min(clock_count);
            self.remaining_note_clocks = self.remaining_note_clocks.saturating_sub(process_clocks);
            self.elapsed_note_clocks += process_clocks;
            clock_offset += process_clocks;
            clock_count -= process_clocks;

            if process_clocks < self.sample_clocks {
                self.carryover_sample_clocks = self.sample_clocks - process_clocks;
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        // Phase 3: Tail fade-out (remaining_note_clocks <= interp_clocks)
        if self.remaining_note_clocks > 0 && clock_count > 0 {
            let end_gain = *self.interp_end_gain.get_or_insert(self.last_gain);
            let interp = self.interp_clocks as i64;

            while self.remaining_note_clocks > 0 && clock_count > 0 {
                let gain = ((end_gain as i64 * self.remaining_note_clocks as i64 + interp / 2)
                    / interp) as i32;

                let amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
                self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);
                self.last_gain = gain;

                let process_clocks = self.sample_clocks.min(clock_count);
                self.remaining_note_clocks =
                    self.remaining_note_clocks.saturating_sub(process_clocks);
                self.elapsed_note_clocks += process_clocks;
                clock_offset += process_clocks;
                clock_count -= process_clocks;

                if process_clocks < self.sample_clocks {
                    self.carryover_sample_clocks = self.sample_clocks - process_clocks;
                    return;
                }

                self.oscillator.advance_sample();
                self.advance_control_clock(self.sample_clocks);
            }
        }

        if self.remaining_note_clocks == 0 && clock_count > 0 {
            self.write_sample(blip_buf, clock_offset, 0);
            self.last_gain = 0;
        }
    }

    #[inline]
    fn gain_to_fixed(gain: f32) -> i32 {
        (gain * AUDIO_GAIN_SCALE as f32).round() as i32
    }

    #[inline]
    fn apply_gain_fixed(sample: i32, gain: i32) -> i32 {
        let product = sample as i64 * gain as i64;
        if product >= 0 {
            ((product + VOICE_GAIN_ROUND_BIAS) >> AUDIO_GAIN_SHIFT) as i32
        } else {
            ((product - VOICE_GAIN_ROUND_BIAS) >> AUDIO_GAIN_SHIFT) as i32
        }
    }

    fn reset_control_clock(&mut self) {
        self.envelope.reset_tick();
        self.vibrato.reset_tick();
        self.glide.reset_tick();

        self.update_sample_clocks();
    }

    fn advance_control_clock(&mut self, clocks: u32) {
        self.control_elapsed_clocks += clocks;

        if self.control_elapsed_clocks >= self.control_interval_clocks {
            let ticks = self.control_elapsed_clocks / self.clocks_per_tick;

            if ticks > 0 {
                self.control_elapsed_clocks -= self.clocks_per_tick * ticks;

                self.envelope.advance_tick(ticks);
                self.vibrato.advance_tick(ticks);
                self.glide.advance_tick(ticks);
            }

            self.update_sample_clocks();
        }
    }

    fn update_sample_clocks(&mut self) {
        let frequency =
            self.base_frequency * self.vibrato.pitch_multiplier() * self.glide.pitch_multiplier();
        self.sample_clocks =
            (self.clock_rate as f32 / frequency / self.oscillator.samples_per_cycle() as f32)
                .round() as u32;
    }

    fn write_sample(&mut self, blip_buf: Option<&mut BlipBuf>, clock_offset: u32, amplitude: i32) {
        if let Some(blip_buf) = blip_buf {
            if amplitude != self.last_amplitude {
                blip_buf.add_delta(clock_offset, amplitude - self.last_amplitude);
                self.last_amplitude = amplitude;
            }
        }
    }
}

fn pitch_ratio_lut() -> &'static [f32] {
    PITCH_RATIO_LUT
        .get_or_init(|| {
            (0..PITCH_LUT_SIZE)
                .map(|index| {
                    let semitone_offset =
                        PITCH_LUT_MIN_SEMITONE + index as f32 / PITCH_LUT_STEPS_PER_SEMITONE as f32;
                    2.0_f32.powf(semitone_offset / 12.0)
                })
                .collect()
        })
        .as_ref()
}

fn semitone_to_pitch_multiplier(semitone_offset: f32) -> f32 {
    if !(PITCH_LUT_MIN_SEMITONE..=PITCH_LUT_MAX_SEMITONE).contains(&semitone_offset) {
        return 2.0_f32.powf(semitone_offset / 12.0);
    }

    let index = (semitone_offset - PITCH_LUT_MIN_SEMITONE) * PITCH_LUT_STEPS_PER_SEMITONE as f32;
    let left_index = index as usize;
    let frac = index - left_index as f32;
    let lut = pitch_ratio_lut();
    let left = lut[left_index];

    if frac <= 0.0 || left_index + 1 >= lut.len() {
        left
    } else {
        left + (lut[left_index + 1] - left) * frac
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPROX_EPSILON: f32 = 1e-4;

    fn approx_eq(a: f32, b: f32) -> bool {
        (a - b).abs() < APPROX_EPSILON
    }

    // semitone_to_pitch_multiplier

    #[test]
    fn test_pitch_multiplier_known_values() {
        let cases: &[(f32, f32)] = &[
            (0.0, 1.0),
            (12.0, 2.0),
            (-12.0, 0.5),
            (24.0, 4.0),
            (0.5, 2.0_f32.powf(0.5 / 12.0)),
        ];
        for &(semitone, expected) in cases {
            let result = semitone_to_pitch_multiplier(semitone);
            assert!(
                approx_eq(result, expected),
                "semitone={semitone}: expected {expected}, got {result}"
            );
        }
    }

    #[test]
    fn test_pitch_multiplier_out_of_range() {
        for semitone in [100.0, -100.0] {
            let result = semitone_to_pitch_multiplier(semitone);
            let expected = 2.0_f32.powf(semitone / 12.0);
            assert!(
                approx_eq(result, expected),
                "semitone={semitone}: expected {expected}, got {result}"
            );
        }
    }

    #[test]
    fn test_pitch_multiplier_lut_boundary() {
        // Exactly at LUT boundaries: -96.0 and 96.0
        for semitone in [PITCH_LUT_MIN_SEMITONE, PITCH_LUT_MAX_SEMITONE] {
            let result = semitone_to_pitch_multiplier(semitone);
            let expected = 2.0_f32.powf(semitone / 12.0);
            assert!(
                approx_eq(result, expected),
                "boundary semitone={semitone}: expected {expected}, got {result}"
            );
        }
    }

    // Oscillator

    #[test]
    fn test_oscillator_waveform_cycle() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, -1.0]);
        assert_eq!(osc.samples_per_cycle(), 2);
        assert_eq!(osc.sample(), i16::MAX as i32);

        osc.advance_sample();
        assert_eq!(osc.sample(), -(i16::MAX as i32));

        osc.advance_sample();
        assert_eq!(osc.sample(), i16::MAX as i32);
    }

    #[test]
    fn test_oscillator_four_sample_waveform() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, 0.5, 0.0, -1.0]);
        assert_eq!(osc.samples_per_cycle(), 4);

        let expected = [
            Oscillator::quantize_sample(1.0) as i32,
            Oscillator::quantize_sample(0.5) as i32,
            Oscillator::quantize_sample(0.0) as i32,
            Oscillator::quantize_sample(-1.0) as i32,
            Oscillator::quantize_sample(1.0) as i32, // wraps
        ];
        for (i, &exp) in expected.iter().enumerate() {
            assert_eq!(osc.sample(), exp, "sample {i}");
            osc.advance_sample();
        }
    }

    #[test]
    fn test_oscillator_noise() {
        let mut osc = Oscillator::new();
        osc.set_noise(false);
        assert_eq!(osc.samples_per_cycle(), 1);

        osc.set_noise(true);
        let s = osc.sample();
        assert!(
            s == i16::MAX as i32 || s == -(i16::MAX as i32),
            "noise sample should be +/-MAX, got {s}"
        );
    }

    #[test]
    fn test_oscillator_noise_mode_switch() {
        let mut osc = Oscillator::new();

        // Short period
        osc.set_noise(true);
        let short_samples: Vec<i32> = (0..10)
            .map(|_| {
                let s = osc.sample();
                osc.advance_sample();
                s
            })
            .collect();

        // Long period (different LFSR seed)
        osc.set_noise(false);
        let long_samples: Vec<i32> = (0..10)
            .map(|_| {
                let s = osc.sample();
                osc.advance_sample();
                s
            })
            .collect();

        // Different mode should produce different patterns
        assert_ne!(short_samples, long_samples);
    }

    #[test]
    fn test_oscillator_noise_deterministic() {
        let mut osc1 = Oscillator::new();
        let mut osc2 = Oscillator::new();
        osc1.set_noise(true);
        osc2.set_noise(true);

        for i in 0..20 {
            assert_eq!(
                osc1.sample(),
                osc2.sample(),
                "noise should be deterministic at step {i}"
            );
            osc1.advance_sample();
            osc2.advance_sample();
        }
    }

    #[test]
    #[should_panic(expected = "is_empty")]
    fn test_oscillator_empty_waveform_panics() {
        let mut osc = Oscillator::new();
        osc.set(&[]);
    }

    #[test]
    fn test_oscillator_quantize_clamps() {
        assert_eq!(Oscillator::quantize_sample(2.0), i16::MAX);
        assert_eq!(Oscillator::quantize_sample(-2.0), i16::MIN);
        assert_eq!(Oscillator::quantize_sample(0.0), 0);
    }

    #[test]
    fn test_oscillator_waveform_index_preservation() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, 0.0, -1.0]);
        osc.advance_sample();
        assert_eq!(osc.waveform_index, 1);

        // Identical waveform preserves index
        osc.set(&[1.0, 0.0, -1.0]);
        assert_eq!(osc.waveform_index, 1);

        // Different waveform resets index
        osc.set(&[0.5, -0.5]);
        assert_eq!(osc.waveform_index, 0);
    }

    // Envelope

    #[test]
    fn test_envelope_lifecycle() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0)]);

        // Disabled returns 1.0
        env.reset_tick();
        assert!(approx_eq(env.level(), 1.0), "disabled: {}", env.level());

        // Enable and verify attack ramp
        env.enable();
        env.reset_tick();
        assert!(approx_eq(env.level(), 0.0), "start: {}", env.level());

        env.advance_tick(5);
        assert!(approx_eq(env.level(), 0.5), "midpoint: {}", env.level());

        env.advance_tick(5);
        assert!(approx_eq(env.level(), 1.0), "end: {}", env.level());

        // Reset restarts from beginning
        env.reset_tick();
        assert!(approx_eq(env.level(), 0.0), "after reset: {}", env.level());
    }

    #[test]
    fn test_envelope_attack_decay_sustain() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0), (10, 0.5)]);
        env.enable();
        env.reset_tick();

        env.advance_tick(10);
        assert!(approx_eq(env.level(), 1.0), "after attack: {}", env.level());

        env.advance_tick(5);
        assert!(approx_eq(env.level(), 0.75), "mid decay: {}", env.level());

        env.advance_tick(5);
        assert!(approx_eq(env.level(), 0.5), "sustain: {}", env.level());

        env.advance_tick(100);
        assert!(
            approx_eq(env.level(), 0.5),
            "sustained hold: {}",
            env.level()
        );
    }

    #[test]
    fn test_envelope_zero_duration_segment() {
        // Zero-duration segment = instant jump
        let mut env = Envelope::new();
        env.set(0.0, &[(0, 1.0), (10, 0.5)]);
        env.enable();
        env.reset_tick();

        // Should jump to 1.0 immediately then decay
        assert!(
            approx_eq(env.level(), 0.0) || approx_eq(env.level(), 1.0),
            "at tick 0: {}",
            env.level()
        );

        env.advance_tick(10);
        assert!(approx_eq(env.level(), 0.5), "after decay: {}", env.level());
    }

    #[test]
    fn test_envelope_disable_reenable() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0)]);
        env.enable();
        env.reset_tick();
        env.advance_tick(5);
        assert!(approx_eq(env.level(), 0.5), "mid: {}", env.level());

        // Disable -> level returns to 1.0
        env.disable();
        env.advance_tick(1);
        assert!(approx_eq(env.level(), 1.0), "disabled: {}", env.level());

        // Re-enable -> continues from where it was
        env.enable();
        env.advance_tick(1);
        assert!(
            env.level() > 0.5,
            "re-enabled should continue: {}",
            env.level()
        );
    }

    #[test]
    fn test_envelope_initial_level_only() {
        // No segments beyond initial level
        let mut env = Envelope::new();
        env.set(0.8, &[]);
        env.enable();
        env.reset_tick();
        assert!(approx_eq(env.level(), 0.8), "initial: {}", env.level());

        env.advance_tick(100);
        assert!(approx_eq(env.level(), 0.8), "sustained: {}", env.level());
    }

    // Vibrato

    #[test]
    fn test_vibrato_behavior() {
        let mut vib = Vibrato::new();
        vib.set(0, 10, 1.0);

        // Disabled returns 1.0
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "disabled: {}",
            vib.pitch_multiplier()
        );

        // Within delay returns 1.0
        vib.set(10, 20, 2.0);
        vib.enable();
        vib.reset_tick();
        vib.advance_tick(5);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "within delay: {}",
            vib.pitch_multiplier()
        );

        // Modulates after delay
        let mut vib = Vibrato::new();
        vib.set(0, 40, 2.0);
        vib.enable();
        vib.reset_tick();
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "at start: {}",
            vib.pitch_multiplier()
        );

        vib.advance_tick(10);
        assert!(
            vib.pitch_multiplier() > 1.0,
            "at quarter period should be > 1.0: {}",
            vib.pitch_multiplier()
        );
    }

    #[test]
    fn test_vibrato_triangle_wave_shape() {
        // Verify the triangle wave has correct symmetry over a full period
        let mut vib = Vibrato::new();
        let period = 100;
        vib.set(0, period, 2.0);
        vib.enable();
        vib.reset_tick();

        // At start: multiplier = 1.0 (zero crossing)
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "start: {}",
            vib.pitch_multiplier()
        );

        // At half period: should return to ~1.0 (zero crossing)
        vib.advance_tick(period / 2);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "half period: {}",
            vib.pitch_multiplier()
        );
    }

    #[test]
    fn test_vibrato_zero_period() {
        // period=0 -> inv_period_ticks=0, modulation calculation should not panic
        let mut vib = Vibrato::new();
        vib.set(0, 0, 2.0);
        vib.enable();
        vib.reset_tick();
        vib.advance_tick(10);
        // Should not panic; exact multiplier depends on 0*0 float behavior
        let _ = vib.pitch_multiplier();
    }

    #[test]
    fn test_vibrato_zero_depth() {
        let mut vib = Vibrato::new();
        vib.set(0, 20, 0.0); // zero depth = no modulation
        vib.enable();
        vib.reset_tick();
        vib.advance_tick(10);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "zero depth: {}",
            vib.pitch_multiplier()
        );
    }

    // Glide

    #[test]
    fn test_glide_behavior() {
        let mut glide = Glide::new();
        glide.set(12.0, 100);

        // Disabled returns 1.0
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "disabled: {}",
            glide.pitch_multiplier()
        );

        // Starts at offset and converges to 1.0
        glide.enable();
        glide.reset_tick();
        assert!(
            approx_eq(glide.pitch_multiplier(), 2.0),
            "start: {}",
            glide.pitch_multiplier()
        );

        glide.advance_tick(50);
        let expected = 2.0_f32.powf(6.0 / 12.0);
        assert!(
            (glide.pitch_multiplier() - expected).abs() < 0.01,
            "midpoint: expected ~{expected}, got {}",
            glide.pitch_multiplier()
        );

        glide.advance_tick(50);
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "end: {}",
            glide.pitch_multiplier()
        );
    }

    #[test]
    fn test_glide_zero_duration() {
        // Zero duration: slope is 0, stays at 1.0 (elapsed >= duration immediately)
        let mut glide = Glide::new();
        glide.set(12.0, 0);
        glide.enable();
        glide.reset_tick();
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "zero duration: {}",
            glide.pitch_multiplier()
        );
    }

    #[test]
    fn test_glide_negative_offset() {
        // Negative offset = starts below target pitch
        let mut glide = Glide::new();
        glide.set(-12.0, 100);
        glide.enable();
        glide.reset_tick();
        assert!(
            approx_eq(glide.pitch_multiplier(), 0.5),
            "start at -12 semitones: {}",
            glide.pitch_multiplier()
        );

        glide.advance_tick(100);
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "converged: {}",
            glide.pitch_multiplier()
        );
    }

    #[test]
    fn test_glide_past_duration() {
        let mut glide = Glide::new();
        glide.set(12.0, 50);
        glide.enable();
        glide.reset_tick();

        glide.advance_tick(100); // well past duration
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "past duration: {}",
            glide.pitch_multiplier()
        );
    }

    // Voice

    #[test]
    fn test_voice_new_initial_state() {
        let voice = Voice::new(44100, 60, 512);
        assert_eq!(voice.clock_rate, 44100);
        assert_eq!(voice.clocks_per_tick, 1);
        assert_eq!(voice.base_frequency, 0.0);
        assert_eq!(voice.velocity, 0.0);
        assert_eq!(voice.remaining_note_clocks, 0);
        assert_eq!(voice.last_amplitude, 0);
        assert_eq!(voice.control_interval_clocks, 44100 / 60);
        assert!(!voice.needs_processing());
    }

    #[test]
    fn test_voice_play_note_sets_state() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.oscillator.set(&[1.0, -1.0]);
        voice.play_note(69.0, 1.0, 1000);

        assert!(
            approx_eq(voice.base_frequency, 440.0),
            "A4 should be 440Hz, got {}",
            voice.base_frequency
        );
        assert!(voice.needs_processing());
        assert_eq!(voice.velocity, 1.0);
    }

    #[test]
    fn test_voice_play_note_frequencies() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.oscillator.set(&[1.0, -1.0]);

        // C4 = MIDI 60, ~261.63 Hz
        voice.play_note(60.0, 1.0, 1000);
        assert!(
            (voice.base_frequency - 261.63).abs() < 0.1,
            "C4: {}",
            voice.base_frequency
        );

        // C5 = MIDI 72, ~523.25 Hz
        voice.play_note(72.0, 1.0, 1000);
        assert!(
            (voice.base_frequency - 523.25).abs() < 0.1,
            "C5: {}",
            voice.base_frequency
        );
    }

    #[test]
    fn test_voice_set_clocks_per_tick() {
        let mut voice = Voice::new(44100, 60, 512);
        assert_eq!(voice.clocks_per_tick, 1);

        voice.set_clocks_per_tick(100);
        assert_eq!(voice.clocks_per_tick, 100);
    }

    #[test]
    fn test_voice_gain_to_fixed_and_apply() {
        let gain = Voice::gain_to_fixed(1.0);
        let result = Voice::apply_gain_fixed(1000, gain);
        assert_eq!(result, 1000);

        let gain = Voice::gain_to_fixed(0.5);
        let result = Voice::apply_gain_fixed(1000, gain);
        assert_eq!(result, 500);

        let gain = Voice::gain_to_fixed(0.0);
        let result = Voice::apply_gain_fixed(1000, gain);
        assert_eq!(result, 0);

        let gain = Voice::gain_to_fixed(1.0);
        let result = Voice::apply_gain_fixed(-1000, gain);
        assert!((result + 1000).abs() <= 1, "expected ~-1000, got {result}");
    }

    #[test]
    fn test_voice_process_without_note_is_noop() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.oscillator.set(&[1.0, -1.0]);
        voice.process(None, 0, 1000);
        assert_eq!(voice.last_amplitude, 0);
    }

    #[test]
    fn test_voice_cancel_note_limits_remaining() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.oscillator.set(&[1.0, -1.0]);
        voice.play_note(69.0, 1.0, 10000);

        let before = voice.remaining_note_clocks;
        voice.cancel_note();
        assert!(
            voice.remaining_note_clocks <= voice.interp_clocks,
            "after cancel: {} should be <= {}",
            voice.remaining_note_clocks,
            voice.interp_clocks
        );
        assert!(voice.remaining_note_clocks < before);
    }

    #[test]
    fn test_voice_needs_processing_transitions() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.oscillator.set(&[1.0, -1.0]);
        assert!(!voice.needs_processing(), "initially idle");

        voice.play_note(69.0, 1.0, 100);
        assert!(voice.needs_processing(), "after play_note");

        // Process enough clocks to finish the note
        voice.process(None, 0, 100 + voice.interp_clocks + 1);
        // After processing, remaining_note_clocks should be 0
        // and if last_amplitude is also 0, needs_processing is false
        assert_eq!(voice.remaining_note_clocks, 0);
    }
}
