use std::sync::OnceLock;

use blip_buf::BlipBuf;

const A4_MIDI_NOTE: f32 = 69.0;
const A4_FREQUENCY: f32 = 440.0;
const VOICE_GAIN_SHIFT: u32 = 14;
const VOICE_GAIN_SCALE: i64 = 1_i64 << VOICE_GAIN_SHIFT;
const VOICE_GAIN_ROUNDING: i64 = 1_i64 << (VOICE_GAIN_SHIFT - 1);
const PITCH_LUT_MIN_SEMITONE: f32 = -96.0;
const PITCH_LUT_MAX_SEMITONE: f32 = 96.0;
const PITCH_LUT_STEPS_PER_SEMITONE: usize = 64;
const PITCH_LUT_SIZE: usize =
    ((PITCH_LUT_MAX_SEMITONE - PITCH_LUT_MIN_SEMITONE) as usize * PITCH_LUT_STEPS_PER_SEMITONE) + 1;
static PITCH_RATIO_LUT: OnceLock<Box<[f32]>> = OnceLock::new();

fn pitch_ratio_lut() -> &'static [f32] {
    PITCH_RATIO_LUT
        .get_or_init(|| {
            (0..PITCH_LUT_SIZE)
                .map(|index| {
                    let semitone_offset =
                        PITCH_LUT_MIN_SEMITONE + index as f32 / PITCH_LUT_STEPS_PER_SEMITONE as f32;
                    2.0_f32.powf(semitone_offset / 12.0)
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
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

        let quantized_samples: Vec<i16> = waveform
            .iter()
            .map(|&sample| Self::quantize_sample(sample))
            .collect();

        if quantized_samples != self.waveform_samples {
            self.waveform_samples = quantized_samples;
            self.waveform_index = 0;
        }

        self.tap_bit = 0;

        self.update();
    }

    pub fn set_noise(&mut self, short_period: bool) {
        let tap_bit = if short_period { 6 } else { 1 };
        if tap_bit != self.tap_bit {
            self.lfsr = if short_period { 0x0201 } else { 0x7001 };
            self.tap_bit = tap_bit;
        }
        // The LFSR originally starts with 0x0001, but to avoid leading zeros, it is advanced first.
        // For short-period noise, it's shifted 15 times (half of the 32-sample period), resulting in 0x0201.
        // For long-period noise, it's shifted 45 times (half of the 93-sample period), resulting in 0x7001.

        if !self.waveform_samples.is_empty() {
            self.waveform_samples.clear();
        }

        self.update();
    }

    fn sample(&self) -> i32 {
        self.sample
    }

    fn cycle_resolution(&self) -> u32 {
        if self.tap_bit == 0 {
            self.waveform_samples.len() as u32
        } else {
            1
        }
    }

    fn advance_sample(&mut self) {
        if self.tap_bit == 0 {
            self.waveform_index += 1;
            if self.waveform_index >= self.waveform_samples.len() {
                self.waveform_index = 0;
            }
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

            self.segments.insert(
                0,
                EnvelopeSegment {
                    start_tick,
                    start_level,
                    slope,
                },
            );

            start_tick += duration;
            start_level = target_level;
        }

        self.segments.insert(
            0,
            EnvelopeSegment {
                start_tick,
                start_level,
                slope: 0.0,
            },
        );

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
        self.segment_index = self.segments.len() - 1;
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

    note_interp_clocks: u32,
    interp_start_amplitude: Option<i32>,
    interp_end_amplitude: Option<i32>,
}

impl Voice {
    pub fn new(clock_rate: u32, control_rate: u32, note_interp_clocks: u32) -> Self {
        assert!(clock_rate > 0 && control_rate > 0 && note_interp_clocks > 0);
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

            note_interp_clocks,
            interp_start_amplitude: None,
            interp_end_amplitude: None,
        }
    }

    pub fn set_clocks_per_tick(&mut self, clocks_per_tick: u32) {
        assert!(clocks_per_tick > 0);

        self.clocks_per_tick = clocks_per_tick;
    }

    pub fn play_note(&mut self, midi_note: f32, velocity: f32, duration_clocks: u32) {
        self.base_frequency = A4_FREQUENCY * ((midi_note - A4_MIDI_NOTE) / 12.0).exp2();
        self.velocity = velocity;
        self.remaining_note_clocks = duration_clocks + self.note_interp_clocks;
        self.elapsed_note_clocks = 0;
        self.interp_start_amplitude = None;
        self.interp_end_amplitude = None;

        self.reset_control_clock();
    }

    pub fn cancel_note(&mut self) {
        self.remaining_note_clocks = self.remaining_note_clocks.min(self.note_interp_clocks);
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

        while self.remaining_note_clocks > 0 && clock_count > 0 {
            // Calculate amplitude and write sample
            let gain = Self::gain_to_fixed(self.envelope.level() * self.velocity);

            let target_amplitude = Self::apply_gain_fixed(self.oscillator.sample(), gain);
            let amplitude = if self.remaining_note_clocks < self.note_interp_clocks {
                if self.interp_end_amplitude.is_none() {
                    self.interp_end_amplitude = Some(self.last_amplitude);
                }
                let interp_end_amplitude = self.interp_end_amplitude.unwrap();
                ((interp_end_amplitude as i64 * self.remaining_note_clocks as i64
                    + self.note_interp_clocks as i64 / 2)
                    / self.note_interp_clocks as i64) as i32
            } else if self.elapsed_note_clocks < self.note_interp_clocks {
                if self.interp_start_amplitude.is_none() {
                    self.interp_start_amplitude = Some(self.last_amplitude);
                }
                let interp_start_amplitude = self.interp_start_amplitude.unwrap();
                let note_interp_clocks = self.note_interp_clocks as i64;
                let elapsed_note_clocks = self.elapsed_note_clocks as i64;
                ((interp_start_amplitude as i64 * (note_interp_clocks - elapsed_note_clocks)
                    + target_amplitude as i64 * elapsed_note_clocks
                    + note_interp_clocks / 2)
                    / note_interp_clocks) as i32
            } else {
                target_amplitude
            };

            self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);

            // Advance oscillator and control clock
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

        if self.remaining_note_clocks == 0 && clock_count > 0 {
            self.write_sample(blip_buf, clock_offset, 0);
        }
    }

    fn gain_to_fixed(gain: f32) -> i32 {
        (gain * VOICE_GAIN_SCALE as f32).round() as i32
    }

    fn apply_gain_fixed(sample: i32, gain: i32) -> i32 {
        let product = sample as i64 * gain as i64;
        if product >= 0 {
            ((product + VOICE_GAIN_ROUNDING) >> VOICE_GAIN_SHIFT) as i32
        } else {
            ((product - VOICE_GAIN_ROUNDING) >> VOICE_GAIN_SHIFT) as i32
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
        self.sample_clocks = (self.clock_rate as f32
            / frequency
            / self.oscillator.cycle_resolution() as f32)
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
