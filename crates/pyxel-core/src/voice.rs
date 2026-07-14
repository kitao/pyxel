use std::sync::OnceLock;

use blip_buf::BlipBuf;

use crate::settings::{AUDIO_GAIN_SCALE, AUDIO_GAIN_SHIFT};
use crate::tone::{RcTone, ToneMode};

const A4_MIDI_NOTE: f32 = 69.0;
const A4_FREQUENCY: f32 = 440.0;
// Fixed-point Q14 scaling for voice gain multiplication
// Half-unit bias rounds fixed-point gain away from zero.
const VOICE_GAIN_ROUND_BIAS: i64 = 1_i64 << (AUDIO_GAIN_SHIFT - 1);
const PITCH_LUT_MIN_SEMITONE: f32 = -96.0;
const PITCH_LUT_MAX_SEMITONE: f32 = 96.0;
const PITCH_LUT_STEPS_PER_SEMITONE: usize = 64;
const PITCH_LUT_SIZE: usize =
    ((PITCH_LUT_MAX_SEMITONE - PITCH_LUT_MIN_SEMITONE) as usize * PITCH_LUT_STEPS_PER_SEMITONE) + 1;
static PITCH_RATIO_LUT: OnceLock<Box<[f32]>> = OnceLock::new();

pub struct Oscillator {
    waveform_samples: Vec<i16>,
    waveform_index: usize,
    #[cfg(test)]
    waveform_set_count: usize,

    lfsr: u16,
    tap_bit: u8,

    sample: i32,
}

impl Oscillator {
    // Constructors

    fn new() -> Self {
        Self {
            waveform_samples: Vec::new(),
            waveform_index: 0,
            #[cfg(test)]
            waveform_set_count: 0,

            lfsr: 0,
            tap_bit: 0,

            sample: 0,
        }
    }

    // Waveform setup

    pub fn set(&mut self, waveform: &[f32]) {
        #[cfg(test)]
        {
            self.waveform_set_count += 1;
        }
        if self.waveform_samples.len() == waveform.len() {
            for (dst, &src) in self.waveform_samples.iter_mut().zip(waveform.iter()) {
                *dst = Self::quantize_sample(src);
            }
        } else {
            self.waveform_samples.clear();
            self.waveform_samples.reserve(waveform.len());
            for &s in waveform {
                self.waveform_samples.push(Self::quantize_sample(s));
            }
        }
        if self.waveform_index >= self.waveform_samples.len() {
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
        self.update();
    }

    // Sample generation

    fn sample(&self) -> i32 {
        self.sample
    }

    fn samples_per_cycle(&self) -> u32 {
        if self.tap_bit == 0 {
            (self.waveform_samples.len() as u32).max(1)
        } else {
            1
        }
    }

    fn advance_sample(&mut self) {
        if self.tap_bit == 0 {
            let len = self.waveform_samples.len();
            if len > 0 {
                self.waveform_index = (self.waveform_index + 1) % len;
            }
        } else {
            let feedback = (self.lfsr ^ (self.lfsr >> self.tap_bit)) & 1;
            self.lfsr = ((self.lfsr >> 1) | (feedback << 14)) & 0x7FFF;
        }
        self.update();
    }

    // Helpers

    fn update(&mut self) {
        self.sample = if self.tap_bit == 0 {
            if self.waveform_samples.is_empty() {
                0
            } else {
                self.waveform_samples[self.waveform_index] as i32
            }
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
    start_tick: f32,
    start_level: f32,
    slope: f32,
}

pub struct Envelope {
    segments: Vec<EnvelopeSegment>,
    segment_index: usize,
    inv_clocks_per_tick: f32,
    enabled: bool,
}

impl Envelope {
    // Constructors

    fn new() -> Self {
        Self {
            segments: vec![EnvelopeSegment {
                start_tick: 0.0,
                start_level: 1.0,
                slope: 0.0,
            }],
            segment_index: 0,
            inv_clocks_per_tick: 1.0,
            enabled: false,
        }
    }

    // Envelope shape

    pub fn set(&mut self, initial_level: f32, segments: &[(u32, f32)]) {
        self.segments.clear();

        let mut start_tick = 0_u64;
        let mut start_level = initial_level;

        for &(duration, target_level) in segments {
            let slope = if duration > 0 {
                (target_level - start_level) / duration as f32
            } else {
                0.0
            };

            self.segments.push(EnvelopeSegment {
                start_tick: start_tick as f32,
                start_level,
                slope,
            });

            start_tick += u64::from(duration);
            start_level = target_level;
        }

        self.segments.push(EnvelopeSegment {
            start_tick: start_tick as f32,
            start_level,
            slope: 0.0,
        });

        self.segment_index = 0;
    }

    pub fn set_clocks_per_tick(&mut self, clocks_per_tick: u32) {
        self.inv_clocks_per_tick = 1.0 / clocks_per_tick as f32;
    }

    // State controls

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    // Level evaluation

    fn reset(&mut self) {
        self.segment_index = 0;
    }

    // Level at note-relative elapsed clocks; the segment cursor only walks
    // forward, so callers reset() when elapsed rewinds
    fn level_at(&mut self, elapsed_clocks: u64) -> f32 {
        if !self.enabled {
            return 1.0;
        }

        let elapsed_ticks = elapsed_clocks as f32 * self.inv_clocks_per_tick;
        while self.segment_index + 1 < self.segments.len()
            && elapsed_ticks >= self.segments[self.segment_index + 1].start_tick
        {
            self.segment_index += 1;
        }

        let segment = &self.segments[self.segment_index];
        segment.start_level + segment.slope * (elapsed_ticks - segment.start_tick)
    }
}

pub struct Vibrato {
    delay_ticks: u32,
    period_ticks: u32,
    inv_period_ticks: f64,
    semitone_depth: f32,

    enabled: bool,
    pitch_multiplier: f32,
}

impl Vibrato {
    // Constructors

    fn new() -> Self {
        Self {
            delay_ticks: 0,
            period_ticks: 1,
            inv_period_ticks: 1.0,
            semitone_depth: 0.0,

            enabled: false,
            pitch_multiplier: 1.0,
        }
    }

    // Vibrato controls

    pub fn set(&mut self, delay_ticks: u32, period_ticks: u32, semitone_depth: f32) {
        self.delay_ticks = delay_ticks;
        self.semitone_depth = semitone_depth;

        if period_ticks != self.period_ticks {
            self.period_ticks = period_ticks;
            self.inv_period_ticks = if period_ticks > 0 {
                1.0 / f64::from(period_ticks)
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

    // Pitch evaluation

    // A delayed vibrato traces its curve per note; without a delay the phase
    // follows playback time so the modulation stays continuous across notes
    fn update_at(&mut self, note_ticks: f64, playback_ticks: f64) {
        if !self.enabled {
            self.pitch_multiplier = 1.0;
            return;
        }

        let elapsed_ticks = if self.delay_ticks > 0 {
            note_ticks - f64::from(self.delay_ticks)
        } else {
            playback_ticks
        };
        if elapsed_ticks < 0.0 {
            self.pitch_multiplier = 1.0;
            return;
        }

        let phase = elapsed_ticks * self.inv_period_ticks;
        let modulation = 1.0 - 4.0 * ((phase + 0.25).fract() - 0.5).abs();
        let semitone_offset = modulation as f32 * self.semitone_depth;

        self.pitch_multiplier = semitone_to_pitch_multiplier(semitone_offset);
    }
}

pub struct Glide {
    semitone_offset: f32,
    duration_ticks: u32,
    semitone_slope: f32,

    enabled: bool,
    pitch_multiplier: f32,
}

impl Glide {
    // Constructors

    fn new() -> Self {
        Self {
            semitone_offset: 0.0,
            duration_ticks: 0,
            semitone_slope: 0.0,

            enabled: false,
            pitch_multiplier: 1.0,
        }
    }

    // Glide controls

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

    // Pitch evaluation

    fn update_at(&mut self, note_ticks: f32) {
        if !self.enabled || note_ticks >= self.duration_ticks as f32 {
            self.pitch_multiplier = 1.0;
            return;
        }

        let semitone_offset = self.semitone_offset + self.semitone_slope * note_ticks;
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
    velocity_base: f32,
    current_tone: Option<RcTone>,
    current_tone_mode: Option<ToneMode>,
    current_tone_revision: Option<u64>,
    current_velocity_cache: f32,
    remaining_note_clocks: u64,
    elapsed_note_clocks: u64,
    playback_ticks: f64,
    next_modulator_tick_clock: u64,
    next_gain_update_clock: u64,
    sample_clocks: u32,
    sample_remaining_clocks: u32,
    carryover_event_clocks: u32,
    control_interval_clocks: u32,
    control_elapsed_clocks: u32,
    last_amplitude: i32,

    interp_clocks: u32,
    interp_start_gain: Option<i32>,
    interp_end_gain: Option<i32>,
    last_gain: i32,
}

impl Voice {
    // Constructors

    pub fn new(clock_rate: u32, control_rate: u32, interp_clocks: u32) -> Self {
        assert!(clock_rate > 0 && control_rate > 0 && interp_clocks > 0);
        // Build the pitch-ratio table before audio processing can reach it.
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
            velocity_base: 0.0,
            current_tone: None,
            current_tone_mode: None,
            current_tone_revision: None,
            current_velocity_cache: 0.0,
            remaining_note_clocks: 0,
            elapsed_note_clocks: 0,
            playback_ticks: 0.0,
            next_modulator_tick_clock: 0,
            next_gain_update_clock: 0,
            sample_clocks: 0,
            sample_remaining_clocks: 0,
            carryover_event_clocks: 0,
            control_interval_clocks,
            control_elapsed_clocks: 0,
            last_amplitude: 0,

            interp_clocks,
            interp_start_gain: None,
            interp_end_gain: None,
            last_gain: 0,
        }
    }

    // Tone controls

    pub fn set_tone(&mut self, tone: RcTone) {
        self.current_tone = Some(tone);
        self.current_tone_mode = None;
        self.current_tone_revision = None;
        self.refresh_tone_state();
    }

    fn refresh_tone_state(&mut self) {
        if let Some(tone_rc) = self.current_tone.as_ref() {
            let mut tone = audio_mut!(&tone_rc);
            let mode = tone.mode;
            match mode {
                ToneMode::Wavetable => {
                    let (waveform, revision) = tone.waveform();
                    if self.current_tone_mode != Some(mode)
                        || self.current_tone_revision != Some(revision)
                    {
                        self.oscillator.set(waveform);
                        self.current_tone_revision = Some(revision);
                    }
                }
                ToneMode::ShortPeriodNoise => {
                    if self.current_tone_mode != Some(mode) {
                        self.oscillator.set_noise(true);
                    }
                    self.current_tone_revision = None;
                }
                ToneMode::LongPeriodNoise => {
                    if self.current_tone_mode != Some(mode) {
                        self.oscillator.set_noise(false);
                    }
                    self.current_tone_revision = None;
                }
            }
            self.current_tone_mode = Some(mode);
            self.current_velocity_cache = self.velocity_base * tone.gain;
        }
    }

    pub fn set_clocks_per_tick(&mut self, clocks_per_tick: u32) {
        assert!(clocks_per_tick > 0);

        self.clocks_per_tick = clocks_per_tick;
        self.envelope.set_clocks_per_tick(clocks_per_tick);
    }

    // Playback controls

    pub fn play_note(&mut self, midi_note: f32, velocity_base: f32, duration_clocks: u64) {
        self.base_frequency = A4_FREQUENCY * ((midi_note - A4_MIDI_NOTE) / 12.0).exp2();
        self.velocity_base = velocity_base;
        self.remaining_note_clocks = duration_clocks.saturating_add(u64::from(self.interp_clocks));
        self.elapsed_note_clocks = 0;
        self.next_modulator_tick_clock = u64::from(self.clocks_per_tick);
        self.next_gain_update_clock = u64::from(self.interp_clocks);
        self.sample_remaining_clocks = 0;
        self.carryover_event_clocks = 0;
        self.interp_start_gain = None;
        self.interp_end_gain = None;

        self.reset_control_clock();
        self.sample_remaining_clocks = self.sample_clocks;
    }

    pub(crate) fn reset_playback_clock(&mut self) {
        self.playback_ticks = 0.0;
    }

    fn current_velocity(&self) -> f32 {
        self.current_velocity_cache
    }

    pub fn cancel_note(&mut self) {
        self.remaining_note_clocks = self
            .remaining_note_clocks
            .min(u64::from(self.interp_clocks));
        self.sample_remaining_clocks = self.sample_clocks;
        self.carryover_event_clocks = 0;
    }

    pub(crate) fn needs_processing(&self) -> bool {
        self.remaining_note_clocks > 0
            || self.carryover_event_clocks > 0
            || self.last_amplitude != 0
    }

    // Audio processing

    pub fn process(&mut self, blip_buf: Option<&mut BlipBuf>, clock_offset: u32, clock_count: u32) {
        if clock_count == 0 {
            return;
        }

        let mut blip_buf = blip_buf;
        let mut clock_offset = clock_offset;
        let mut clock_count = clock_count;

        // Finish a split synthesis event from the previous process chunk.
        if self.carryover_event_clocks > 0 {
            let process_clocks = self.carryover_event_clocks.min(clock_count);
            let event_completed = process_clocks == self.carryover_event_clocks;
            self.advance_event_clock(process_clocks, event_completed);
            self.carryover_event_clocks -= process_clocks;
            clock_offset += process_clocks;
            clock_count -= process_clocks;

            if self.carryover_event_clocks > 0 || clock_count == 0 {
                return;
            }
        }

        // Phase 1: Head crossfade (elapsed < interp, but yield to tail when remaining < interp)
        if self.remaining_note_clocks > 0
            && clock_count > 0
            && self.elapsed_note_clocks < u64::from(self.interp_clocks)
            && self.remaining_note_clocks >= u64::from(self.interp_clocks)
        {
            let start_gain = *self.interp_start_gain.get_or_insert(self.last_gain);
            let interp = self.interp_clocks as i64;

            while self.remaining_note_clocks > 0
                && clock_count > 0
                && self.elapsed_note_clocks < u64::from(self.interp_clocks)
                && self.remaining_note_clocks >= u64::from(self.interp_clocks)
            {
                let mut gain = Self::gain_to_fixed(
                    self.envelope.level_at(self.elapsed_note_clocks) * self.current_velocity(),
                );
                let elapsed = self.elapsed_note_clocks as i64;
                gain =
                    ((start_gain as i64 * (interp - elapsed) + gain as i64 * elapsed + interp / 2)
                        / interp) as i32;

                let amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
                self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);
                self.last_gain = gain;

                let event_clocks = self.event_clocks();
                let process_clocks = event_clocks.min(clock_count);
                self.advance_event_clock(process_clocks, process_clocks == event_clocks);
                clock_offset += process_clocks;
                clock_count -= process_clocks;

                if process_clocks < event_clocks {
                    self.carryover_event_clocks = event_clocks - process_clocks;
                    return;
                }
            }
        }

        // Phase 2: Bulk (no interpolation)
        while self.remaining_note_clocks > u64::from(self.interp_clocks) && clock_count > 0 {
            let gain = Self::gain_to_fixed(
                self.envelope.level_at(self.elapsed_note_clocks) * self.current_velocity(),
            );
            let amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
            self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);
            self.last_gain = gain;

            let event_clocks = self.event_clocks();
            let process_clocks = event_clocks.min(clock_count);
            self.advance_event_clock(process_clocks, process_clocks == event_clocks);
            clock_offset += process_clocks;
            clock_count -= process_clocks;

            if process_clocks < event_clocks {
                self.carryover_event_clocks = event_clocks - process_clocks;
                return;
            }
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

                let event_clocks = self.event_clocks();
                let process_clocks = event_clocks.min(clock_count);
                self.advance_event_clock(process_clocks, process_clocks == event_clocks);
                clock_offset += process_clocks;
                clock_count -= process_clocks;

                if process_clocks < event_clocks {
                    self.carryover_event_clocks = event_clocks - process_clocks;
                    return;
                }
            }
        }

        if self.remaining_note_clocks == 0 && clock_count > 0 {
            self.write_sample(blip_buf, clock_offset, 0);
            self.last_gain = 0;
        }
    }

    // Fixed-point helpers

    #[inline]
    fn event_clocks(&self) -> u32 {
        // Synthesis controls use their own clock boundaries so low notes
        // cannot skip short envelopes or modulation.
        let bounded_delta = |target: u64| {
            let delta = target.saturating_sub(self.elapsed_note_clocks);
            if delta == 0 {
                u32::MAX
            } else {
                delta.min(u64::from(u32::MAX)) as u32
            }
        };
        let clocks_until_gain = if self.elapsed_note_clocks < u64::from(self.interp_clocks)
            || self.envelope.enabled
            || self.vibrato.enabled
            || self.glide.enabled
        {
            bounded_delta(self.next_gain_update_clock)
        } else {
            u32::MAX
        };
        self.sample_remaining_clocks
            .min(self.remaining_note_clocks.min(u64::from(u32::MAX)) as u32)
            .min(clocks_until_gain)
            .min(self.control_interval_clocks - self.control_elapsed_clocks)
    }

    #[inline]
    fn advance_event_clock(&mut self, clocks: u32, event_completed: bool) {
        let previous_sample_clocks = self.sample_clocks;
        self.remaining_note_clocks = self.remaining_note_clocks.saturating_sub(u64::from(clocks));
        self.elapsed_note_clocks = self.elapsed_note_clocks.saturating_add(u64::from(clocks));
        self.playback_ticks += f64::from(clocks) / f64::from(self.clocks_per_tick);
        self.sample_remaining_clocks -= clocks;
        let oscillator_boundary = self.sample_remaining_clocks == 0;

        if oscillator_boundary && self.remaining_note_clocks > 0 {
            self.oscillator.advance_sample();
        }

        if self.elapsed_note_clocks >= self.next_gain_update_clock {
            let interval = u64::from(self.interp_clocks);
            self.next_gain_update_clock = (self.elapsed_note_clocks / interval)
                .saturating_add(1)
                .saturating_mul(interval);
        }
        if event_completed {
            self.advance_modulator_clock();
        }
        self.advance_control_clock(clocks);

        if self.remaining_note_clocks == 0 {
            self.sample_remaining_clocks = 0;
        } else if oscillator_boundary {
            self.sample_remaining_clocks = self.sample_clocks;
        } else if self.sample_clocks != previous_sample_clocks {
            let scaled_remaining = u64::from(self.sample_remaining_clocks)
                .saturating_mul(u64::from(self.sample_clocks))
                .div_ceil(u64::from(previous_sample_clocks));
            self.sample_remaining_clocks = scaled_remaining.clamp(1, u64::from(u32::MAX)) as u32;
        }
    }

    #[inline]
    fn gain_to_fixed(gain: f32) -> i32 {
        (gain * AUDIO_GAIN_SCALE as f32).round() as i32
    }

    #[inline]
    fn apply_gain_fixed(sample: i32, gain: i32) -> i32 {
        // Round half away from zero; negating before the shift keeps the
        // floor-division rounding symmetric for negative products
        let product = sample as i64 * gain as i64;
        if product >= 0 {
            ((product + VOICE_GAIN_ROUND_BIAS) >> AUDIO_GAIN_SHIFT) as i32
        } else {
            -(((-product + VOICE_GAIN_ROUND_BIAS) >> AUDIO_GAIN_SHIFT) as i32)
        }
    }

    // Control clock

    fn reset_control_clock(&mut self) {
        self.envelope.reset();
        self.update_modulators();
        self.update_sample_clocks();
    }

    // Evaluate the modulators at the last crossed note-relative tick boundary
    // so the curve values never depend on the oscillator sample grid
    fn update_modulators(&mut self) {
        let note_ticks = (self.elapsed_note_clocks / u64::from(self.clocks_per_tick)) as f64;
        self.vibrato.update_at(note_ticks, self.playback_ticks);
        self.glide.update_at(note_ticks as f32);
    }

    fn advance_modulator_clock(&mut self) {
        if self.elapsed_note_clocks >= self.next_modulator_tick_clock {
            let elapsed_ticks = self.elapsed_note_clocks / u64::from(self.clocks_per_tick);
            self.next_modulator_tick_clock = elapsed_ticks
                .saturating_add(1)
                .saturating_mul(u64::from(self.clocks_per_tick));

            self.update_modulators();
            self.update_sample_clocks();
        }
    }

    fn advance_control_clock(&mut self, clocks: u32) {
        // Tone edits are polled at the control rate
        let clocks_until_control = self.control_interval_clocks - self.control_elapsed_clocks;
        if clocks >= clocks_until_control {
            self.control_elapsed_clocks =
                (clocks - clocks_until_control) % self.control_interval_clocks;
            self.update_sample_clocks();
        } else {
            self.control_elapsed_clocks += clocks;
        }
    }

    fn update_sample_clocks(&mut self) {
        self.refresh_tone_state();

        let frequency =
            self.base_frequency * self.vibrato.pitch_multiplier() * self.glide.pitch_multiplier();
        // Floor at one clock so processing always advances
        self.sample_clocks =
            (self.clock_rate as f32 / frequency / self.oscillator.samples_per_cycle() as f32)
                .round()
                .max(1.0) as u32;
    }

    // Blip output

    fn write_sample(&mut self, blip_buf: Option<&mut BlipBuf>, clock_offset: u32, amplitude: i32) {
        if let Some(blip_buf) = blip_buf {
            if amplitude != self.last_amplitude {
                blip_buf.add_delta(clock_offset, amplitude - self.last_amplitude);
                self.last_amplitude = amplitude;
            }
        }
    }
}

// Pitch helpers

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
        let waveform = [1.0, 0.5, 0.0, -1.0];
        osc.set(&waveform);
        assert_eq!(osc.samples_per_cycle(), 4);

        let expected: Vec<i32> = waveform
            .iter()
            .map(|&s| Oscillator::quantize_sample(s) as i32)
            .collect();
        for i in 0..=waveform.len() {
            assert_eq!(osc.sample(), expected[i % waveform.len()], "sample {i}");
            osc.advance_sample();
        }
    }

    #[test]
    fn test_oscillator_set_updates_on_change() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, -1.0]);
        assert_eq!(osc.sample(), i16::MAX as i32);

        osc.set(&[0.0, -1.0]);
        assert_eq!(osc.sample(), Oscillator::quantize_sample(0.0) as i32);
    }

    #[test]
    fn test_oscillator_set_keeps_index_when_unchanged() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, 0.0, -1.0]);
        osc.advance_sample();
        assert_eq!(osc.waveform_index, 1);

        // Identical waveform preserves index
        osc.set(&[1.0, 0.0, -1.0]);
        assert_eq!(osc.waveform_index, 1);
    }

    #[test]
    fn test_oscillator_set_resets_index_when_out_of_range() {
        let mut osc = Oscillator::new();
        osc.set(&[1.0, 0.0, -1.0]);
        osc.advance_sample();
        osc.advance_sample();
        assert_eq!(osc.waveform_index, 2);

        // Shorter waveform: previous index is now out of range, reset to 0
        osc.set(&[0.5, -0.5]);
        assert_eq!(osc.waveform_index, 0);
    }

    #[test]
    fn test_oscillator_noise() {
        let mut osc = Oscillator::new();
        osc.set_noise(false);
        assert_eq!(osc.samples_per_cycle(), 1);

        // set_noise(true) deterministically seeds the LFSR to 0x0201 (bit 0 set)
        osc.set_noise(true);
        assert_eq!(osc.sample(), -(i16::MAX as i32));
    }

    #[test]
    fn test_oscillator_noise_mode_switch() {
        let mut osc = Oscillator::new();

        osc.set_noise(true);
        let short_samples: Vec<i32> = (0..10)
            .map(|_| {
                let s = osc.sample();
                osc.advance_sample();
                s
            })
            .collect();

        osc.set_noise(false);
        let long_samples: Vec<i32> = (0..10)
            .map(|_| {
                let s = osc.sample();
                osc.advance_sample();
                s
            })
            .collect();

        // Both sequences follow exactly from the documented LFSR seeds and taps;
        // a silent seed/tap change would alter existing users' sound assets
        let max = i16::MAX as i32;
        assert_eq!(
            short_samples,
            [-max, max, max, max, max, max, max, max, max, -max]
        );
        assert_eq!(
            long_samples,
            [-max, max, max, max, max, max, max, max, max, max]
        );
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
    fn test_oscillator_empty_waveform_silent() {
        let mut osc = Oscillator::new();
        osc.set(&[]);
        assert_eq!(osc.sample(), 0);
        assert_eq!(osc.samples_per_cycle(), 1);
        osc.advance_sample();
        assert_eq!(osc.sample(), 0);
    }

    #[test]
    fn test_oscillator_quantize_clamps() {
        assert_eq!(Oscillator::quantize_sample(2.0), i16::MAX);
        assert_eq!(Oscillator::quantize_sample(-2.0), i16::MIN);
        assert_eq!(Oscillator::quantize_sample(0.0), 0);
    }

    // Envelope

    #[test]
    fn test_envelope_lifecycle() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0)]);

        // Disabled envelope level returns 1.0.
        assert!(
            approx_eq(env.level_at(0), 1.0),
            "disabled: {}",
            env.level_at(0)
        );

        // Enabled envelope follows the attack ramp.
        env.enable();
        assert!(
            approx_eq(env.level_at(0), 0.0),
            "start: {}",
            env.level_at(0)
        );
        assert!(
            approx_eq(env.level_at(5), 0.5),
            "midpoint: {}",
            env.level_at(5)
        );
        assert!(
            approx_eq(env.level_at(10), 1.0),
            "end: {}",
            env.level_at(10)
        );

        // Envelope reset restarts from the beginning.
        env.reset();
        assert!(
            approx_eq(env.level_at(0), 0.0),
            "after reset: {}",
            env.level_at(0)
        );
    }

    #[test]
    fn test_envelope_attack_decay_sustain() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0), (10, 0.5)]);
        env.enable();

        assert!(
            approx_eq(env.level_at(10), 1.0),
            "after attack: {}",
            env.level_at(10)
        );
        assert!(
            approx_eq(env.level_at(15), 0.75),
            "mid decay: {}",
            env.level_at(15)
        );
        assert!(
            approx_eq(env.level_at(20), 0.5),
            "sustain: {}",
            env.level_at(20)
        );
        assert!(
            approx_eq(env.level_at(120), 0.5),
            "sustained hold: {}",
            env.level_at(120)
        );
    }

    #[test]
    fn test_envelope_zero_duration_segment() {
        // Zero-duration segment jumps instantly to its target level
        let mut env = Envelope::new();
        env.set(0.0, &[(0, 1.0), (10, 0.5)]);
        env.enable();
        assert!(
            approx_eq(env.level_at(0), 1.0),
            "at tick 0: {}",
            env.level_at(0)
        );
        assert!(
            approx_eq(env.level_at(10), 0.5),
            "after decay: {}",
            env.level_at(10)
        );
    }

    #[test]
    fn test_envelope_disable_reenable() {
        let mut env = Envelope::new();
        env.set(0.0, &[(10, 1.0)]);
        env.enable();
        assert!(approx_eq(env.level_at(5), 0.5), "mid: {}", env.level_at(5));

        // Disabled envelope level returns to 1.0.
        env.disable();
        assert!(
            approx_eq(env.level_at(6), 1.0),
            "disabled: {}",
            env.level_at(6)
        );

        env.enable();
        assert!(
            approx_eq(env.level_at(7), 0.7),
            "re-enabled should continue: {}",
            env.level_at(7)
        );
    }

    #[test]
    fn test_envelope_initial_level_only() {
        // No segments beyond initial level
        let mut env = Envelope::new();
        env.set(0.8, &[]);
        env.enable();
        assert!(
            approx_eq(env.level_at(0), 0.8),
            "initial: {}",
            env.level_at(0)
        );
        assert!(
            approx_eq(env.level_at(100), 0.8),
            "sustained: {}",
            env.level_at(100)
        );
    }

    #[test]
    fn test_envelope_fractional_ticks() {
        let mut env = Envelope::new();
        env.set_clocks_per_tick(100);
        env.set(1.0, &[(2, 0.0)]);
        env.enable();

        assert_eq!(env.level_at(0), 1.0);
        assert_eq!(env.level_at(50), 0.75);
        assert_eq!(env.level_at(100), 0.5);
        assert_eq!(env.level_at(150), 0.25);
        assert_eq!(env.level_at(200), 0.0);
    }

    // Vibrato

    #[test]
    fn test_vibrato_behavior() {
        let mut vib = Vibrato::new();
        vib.set(0, 10, 1.0);

        // Disabled vibrato multiplier returns 1.0.
        vib.update_at(5.0, 5.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "disabled: {}",
            vib.pitch_multiplier()
        );

        // A delayed vibrato follows note time and stays neutral within its
        // delay even when playback time is far ahead.
        vib.set(10, 20, 2.0);
        vib.enable();
        vib.update_at(5.0, 1000.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "within delay: {}",
            vib.pitch_multiplier()
        );

        // Vibrato modulates after its delay.
        let mut vib = Vibrato::new();
        vib.set(0, 40, 2.0);
        vib.enable();
        vib.update_at(0.0, 0.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "at start: {}",
            vib.pitch_multiplier()
        );

        vib.update_at(0.0, 10.0);
        // Quarter period is the triangle peak: full +2 semitone depth
        let expected = 2.0_f32.powf(2.0 / 12.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), expected),
            "at quarter period: expected {expected}, got {}",
            vib.pitch_multiplier()
        );
    }

    #[test]
    fn test_vibrato_triangle_wave_shape() {
        // Verify the triangle wave has correct symmetry over a full period
        let mut vib = Vibrato::new();
        let period = 100.0;
        vib.set(0, 100, 2.0);
        vib.enable();

        // At start: multiplier = 1.0 (zero crossing)
        vib.update_at(0.0, 0.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "start: {}",
            vib.pitch_multiplier()
        );

        // At half period: should return to ~1.0 (zero crossing)
        vib.update_at(0.0, period / 2.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "half period: {}",
            vib.pitch_multiplier()
        );

        // At three-quarter period: trough of the triangle = full -2 semitone depth
        vib.update_at(0.0, period * 0.75);
        let expected = 2.0_f32.powf(-2.0 / 12.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), expected),
            "three-quarter period: expected {expected}, got {}",
            vib.pitch_multiplier()
        );

        // Back at the zero crossing after a full period
        vib.update_at(0.0, period);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "full period: {}",
            vib.pitch_multiplier()
        );
    }

    #[test]
    fn test_vibrato_zero_period() {
        // period=0 -> inv_period_ticks=0 pins the phase to 0, which is the
        // triangle wave's zero crossing, so the multiplier stays 1.0
        let mut vib = Vibrato::new();
        vib.set(0, 0, 2.0);
        vib.enable();
        vib.update_at(0.0, 10.0);
        assert!(
            approx_eq(vib.pitch_multiplier(), 1.0),
            "zero period: {}",
            vib.pitch_multiplier()
        );
    }

    #[test]
    fn test_vibrato_zero_depth() {
        let mut vib = Vibrato::new();
        // Zero depth disables modulation even while vibrato is enabled.
        vib.set(0, 20, 0.0);
        vib.enable();
        vib.update_at(0.0, 10.0);
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

        // Disabled glide multiplier returns 1.0.
        glide.update_at(0.0);
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "disabled: {}",
            glide.pitch_multiplier()
        );

        // Glide starts at its offset and converges to 1.0.
        glide.enable();
        glide.update_at(0.0);
        assert!(
            approx_eq(glide.pitch_multiplier(), 2.0),
            "start: {}",
            glide.pitch_multiplier()
        );

        glide.update_at(50.0);
        let expected = 2.0_f32.powf(6.0 / 12.0);
        assert!(
            (glide.pitch_multiplier() - expected).abs() < 0.01,
            "midpoint: expected ~{expected}, got {}",
            glide.pitch_multiplier()
        );

        glide.update_at(100.0);
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "end: {}",
            glide.pitch_multiplier()
        );
    }

    #[test]
    fn test_glide_zero_duration() {
        let mut glide = Glide::new();
        glide.set(12.0, 0);
        glide.enable();
        glide.update_at(0.0);
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
        glide.update_at(0.0);
        assert!(
            approx_eq(glide.pitch_multiplier(), 0.5),
            "start at -12 semitones: {}",
            glide.pitch_multiplier()
        );

        glide.update_at(100.0);
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

        glide.update_at(100.0);
        assert!(
            approx_eq(glide.pitch_multiplier(), 1.0),
            "past duration: {}",
            glide.pitch_multiplier()
        );
    }

    // Voice

    use crate::tone::{Tone, ToneSample};

    fn make_tone(sample_bits: u32, wavetable: Vec<ToneSample>) -> RcTone {
        let tone = Tone::new();
        {
            let mut t = audio_mut!(&tone);
            t.sample_bits = sample_bits;
            t.wavetable = wavetable;
        }
        tone
    }

    #[test]
    fn test_voice_new_initial_state() {
        let voice = Voice::new(44100, 60, 512);
        assert_eq!(voice.clock_rate, 44100);
        assert_eq!(voice.clocks_per_tick, 1);
        assert_eq!(voice.base_frequency, 0.0);
        assert_eq!(voice.velocity_base, 0.0);
        assert_eq!(voice.remaining_note_clocks, 0);
        assert_eq!(voice.last_amplitude, 0);
        assert_eq!(voice.control_interval_clocks, 44100 / 60);
        assert!(!voice.needs_processing());
    }

    #[test]
    fn test_voice_play_note_sets_state() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.play_note(69.0, 1.0, 1000);

        assert!(
            approx_eq(voice.base_frequency, 440.0),
            "A4 should be 440Hz, got {}",
            voice.base_frequency
        );
        assert!(voice.needs_processing());
        assert_eq!(voice.velocity_base, 1.0);
    }

    #[test]
    fn test_voice_play_note_preserves_long_duration() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        let duration_clocks = u64::from(u32::MAX) + 1;

        voice.play_note(69.0, 1.0, duration_clocks);

        assert_eq!(
            voice.remaining_note_clocks,
            duration_clocks + u64::from(voice.interp_clocks)
        );
    }

    #[test]
    fn test_voice_play_note_frequencies() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));

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
    fn test_voice_glide_advances_before_slow_oscillator_edge() {
        let mut voice = Voice::new(44100, 60, 10);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.set_clocks_per_tick(100);
        voice.glide.set(12.0, 8);
        voice.glide.enable();
        voice.play_note(69.0, 1.0, 800);
        let initial_multiplier = voice.glide.pitch_multiplier();
        voice.sample_clocks = 1000;
        voice.sample_remaining_clocks = 1000;

        voice.process(None, 0, 400);

        assert!(voice.glide.pitch_multiplier() < initial_multiplier);
    }

    #[test]
    fn test_voice_short_note_ends_before_slow_oscillator_edge() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.play_note(69.0, 1.0, 100);
        voice.sample_clocks = 1000;
        voice.sample_remaining_clocks = 1000;

        voice.process(None, 0, 100 + voice.interp_clocks + 1);

        assert_eq!(voice.remaining_note_clocks, 0);
        assert_eq!(voice.carryover_event_clocks, 0);
        assert_eq!(voice.sample_remaining_clocks, 0);
        assert!(!voice.needs_processing());
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

        // Rounding is symmetric around zero
        let gain = Voice::gain_to_fixed(1.0);
        let result = Voice::apply_gain_fixed(-1000, gain);
        assert_eq!(result, -1000);

        // Half-unit products round away from zero on both sides
        let gain = Voice::gain_to_fixed(0.5);
        assert_eq!(Voice::apply_gain_fixed(1, gain), 1);
        assert_eq!(Voice::apply_gain_fixed(-1, gain), -1);
    }

    #[test]
    fn test_voice_process_without_note_is_noop() {
        // With no note playing, processing must add no deltas to the blip buffer
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        let mut blip_buf = BlipBuf::new(4096);
        blip_buf.set_rates(44100.0, 22050.0);
        voice.process(Some(&mut blip_buf), 0, 1000);
        blip_buf.end_frame(1000);
        let mut samples = [0_i16; 4096];
        let count = blip_buf.read_samples(&mut samples, false);
        assert!(
            samples[..count].iter().all(|&s| s == 0),
            "expected silence without a note"
        );
    }

    #[test]
    fn test_voice_cancel_note_limits_remaining() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.play_note(69.0, 1.0, 10000);
        voice.sample_clocks = 1000;
        voice.sample_remaining_clocks = 1000;
        voice.process(None, 0, 1);
        assert!(voice.carryover_event_clocks > 0);

        voice.cancel_note();

        assert_eq!(voice.remaining_note_clocks, u64::from(voice.interp_clocks));
        assert_eq!(voice.carryover_event_clocks, 0);
    }

    #[test]
    fn test_voice_needs_processing_transitions() {
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        assert!(!voice.needs_processing(), "initially idle");

        voice.play_note(69.0, 1.0, 100);
        assert!(voice.needs_processing(), "after play_note");

        voice.process(None, 0, 100 + voice.interp_clocks + 1);
        assert_eq!(voice.remaining_note_clocks, 0);
        assert_eq!(voice.carryover_event_clocks, 0);
        assert!(!voice.needs_processing());
    }

    #[test]
    fn test_voice_applies_tone_gain_change_per_control() {
        let mut voice = Voice::new(44100, 60, 512);
        let tone = make_tone(1, vec![1, 0]);
        voice.set_tone(tone.clone());
        voice.play_note(60.0, 1.0, 44100);

        let control_clocks = voice.control_interval_clocks + 1;
        voice.process(None, 0, control_clocks);
        let before = voice.current_velocity_cache;

        audio_mut!(&tone).gain = 0.25;
        voice.process(None, 0, control_clocks);

        assert!(approx_eq(voice.current_velocity_cache, 0.25));
        assert!((voice.current_velocity_cache - before).abs() > APPROX_EPSILON);
    }

    #[test]
    fn test_voice_applies_tone_wavetable_change_per_control() {
        let mut voice = Voice::new(44100, 60, 512);
        let tone = make_tone(4, vec![15, 0]);
        voice.set_tone(tone.clone());
        voice.play_note(60.0, 1.0, 44100);

        let control_clocks = voice.control_interval_clocks + 1;
        voice.process(None, 0, control_clocks);
        let before = voice.oscillator.waveform_samples.clone();

        audio_mut!(&tone).wavetable[0] = 8;
        voice.process(None, 0, control_clocks);

        assert_ne!(voice.oscillator.waveform_samples, before);
        let max = ((1u32 << 4) - 1) as f32;
        let expected = Oscillator::quantize_sample((8.0 / max) * 2.0 - 1.0);
        assert_eq!(voice.oscillator.waveform_samples[0], expected);
    }

    #[test]
    fn test_voice_skips_unchanged_wavetable_refresh() {
        let mut voice = Voice::new(44100, 60, 512);
        let tone = make_tone(4, vec![15, 0]);
        voice.set_tone(tone.clone());
        voice.play_note(60.0, 1.0, 44100);
        assert_eq!(voice.oscillator.waveform_set_count, 1);

        let control_clocks = voice.control_interval_clocks + 1;
        voice.process(None, 0, control_clocks);
        assert_eq!(voice.oscillator.waveform_set_count, 1);

        audio_mut!(&tone).wavetable[0] = 8;
        voice.process(None, 0, control_clocks);
        assert_eq!(voice.oscillator.waveform_set_count, 2);
    }

    fn collect_gain_per_sample(voice: &mut Voice, samples: usize) -> Vec<i32> {
        (0..samples)
            .map(|_| {
                let clocks = voice.sample_clocks;
                voice.process(None, 0, clocks);
                voice.last_gain
            })
            .collect()
    }

    #[test]
    fn test_voice_process_split_invariant() {
        // The audio thread drives process() in arbitrary clock chunks; event
        // carryover must make any chunking match one full pass.
        let make = || {
            let mut voice = Voice::new(44100, 60, 512);
            voice.set_tone(make_tone(1, vec![1, 0]));
            voice.set_clocks_per_tick(100);
            voice.envelope.set(1.0, &[(20, 0.25)]);
            voice.envelope.enable();
            voice.vibrato.set(2, 12, 1.0);
            voice.vibrato.enable();
            voice.glide.set(4.0, 20);
            voice.glide.enable();
            voice.play_note(69.0, 1.0, 4410);
            let mut blip_buf = BlipBuf::new(4096);
            blip_buf.set_rates(44100.0, 22050.0);
            (voice, blip_buf)
        };
        let chunks = [7, 13, 1, 500, 29, 3000, 2450];
        let total: u32 = chunks.iter().sum();

        let (mut voice1, mut blip_buf1) = make();
        voice1.process(Some(&mut blip_buf1), 0, total);

        let (mut voice2, mut blip_buf2) = make();
        let mut clock_offset = 0;
        for chunk in chunks {
            voice2.process(Some(&mut blip_buf2), clock_offset, chunk);
            clock_offset += chunk;
        }

        blip_buf1.end_frame(total);
        blip_buf2.end_frame(total);
        let mut samples1 = [0_i16; 4096];
        let mut samples2 = [0_i16; 4096];
        let count1 = blip_buf1.read_samples(&mut samples1, false);
        let count2 = blip_buf2.read_samples(&mut samples2, false);
        assert!(count1 > 0);
        assert_eq!(count1, count2);
        assert_eq!(samples1[..count1], samples2[..count2]);
    }

    #[test]
    fn test_voice_head_crossfade_gain_ramps_up() {
        // Starting from silence, gain must rise monotonically from near zero and
        // reach the full note gain once interp_clocks have elapsed (anti-click).
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.play_note(69.0, 1.0, 44100);

        let head_samples = (voice.interp_clocks / voice.sample_clocks + 2) as usize;
        let gains = collect_gain_per_sample(&mut voice, head_samples);
        let target = Voice::gain_to_fixed(voice.current_velocity());

        assert!(
            gains.windows(2).all(|w| w[0] <= w[1]),
            "gains not monotonic: {gains:?}"
        );
        assert_eq!(
            *gains.first().unwrap(),
            0,
            "head crossfade must start from silence: {gains:?}"
        );
        assert_eq!(
            *gains.last().unwrap(),
            target,
            "did not reach target: {gains:?}"
        );
    }

    #[test]
    fn test_voice_tail_fade_gain_ramps_down() {
        // After cancel_note, gain must fall monotonically and land exactly at 0
        // (anti-click fade-out).
        let mut voice = Voice::new(44100, 60, 512);
        voice.set_tone(make_tone(1, vec![1, 0]));
        voice.play_note(69.0, 1.0, 44100);

        // Reach the steady bulk phase, then cancel
        voice.process(None, 0, 2048);
        voice.cancel_note();

        let tail_samples = (voice.interp_clocks / voice.sample_clocks + 4) as usize;
        let gains = collect_gain_per_sample(&mut voice, tail_samples);

        assert!(
            gains.windows(2).all(|w| w[0] >= w[1]),
            "gains not monotonic: {gains:?}"
        );
        assert_eq!(
            *gains.last().unwrap(),
            0,
            "did not fade to silence: {gains:?}"
        );
    }

    #[test]
    fn test_voice_modulators_are_note_relative() {
        // Delayed vibrato and glide must trace the same per-note value curve
        // no matter how much audio played before the note. Application timing
        // may shift by up to one oscillator sample, so distinct values are
        // compared instead of per-sample readings.
        let collect = |pre_notes: u32| {
            let mut voice = Voice::new(44100, 60, 22);
            voice.set_tone(make_tone(1, vec![1, 0]));
            voice.set_clocks_per_tick(100);
            voice.vibrato.set(2, 8, 12.0);
            voice.vibrato.enable();
            voice.glide.set(12.0, 8);
            voice.glide.enable();

            for _ in 0..pre_notes {
                voice.play_note(69.0, 1.0, 517);
                voice.process(None, 0, 517);
            }

            voice.play_note(69.0, 1.0, 1000);
            let mut values = Vec::new();
            for _ in 0..24 {
                voice.process(None, 0, 50);
                let value = (
                    voice.vibrato.pitch_multiplier(),
                    voice.glide.pitch_multiplier(),
                );
                if values.last() != Some(&value) {
                    values.push(value);
                }
            }
            values
        };

        let baseline = collect(0);
        assert_eq!(baseline, collect(1));
        assert_eq!(baseline, collect(3));
    }

    #[test]
    fn test_voice_vibrato_phase_remains_precise_after_long_history() {
        let multiplier_after_history = |history_ticks| {
            let mut voice = Voice::new(44100, 60, 22);
            voice.set_tone(make_tone(1, vec![1, 0]));
            voice.set_clocks_per_tick(100);
            voice.vibrato.set(0, 16, 12.0);
            voice.vibrato.enable();
            voice.playback_ticks = history_ticks + 5.0;

            voice.play_note(69.0, 1.0, 400);
            voice.vibrato.pitch_multiplier()
        };

        assert_eq!(
            multiplier_after_history(0.0),
            multiplier_after_history(16_777_216.0)
        );
    }
}
