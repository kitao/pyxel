use crate::blip_buf::BlipBuf;

const A4_MIDI_NOTE: f64 = 69.0;
const A4_FREQUENCY: f64 = 440.0;

pub struct Oscillator {
    waveform: Vec<f64>,
    waveform_index: usize,

    lfsr: u16,
    tap_bit: u8,

    sample: f64,
}

impl Oscillator {
    fn new() -> Self {
        Self {
            waveform: Vec::new(),
            waveform_index: 0,

            lfsr: 0,
            tap_bit: 0,

            sample: 0.0,
        }
    }

    pub fn set(&mut self, waveform: &[f64]) {
        assert!(!waveform.is_empty());

        if waveform != self.waveform {
            self.waveform = waveform.to_vec();
            self.waveform_index = 0;
        }

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

        if !self.waveform.is_empty() {
            self.waveform.clear();
        }

        self.update();
    }

    fn sample(&self) -> f64 {
        self.sample
    }

    fn cycle_resolution(&self) -> u32 {
        if self.tap_bit == 0 {
            self.waveform.len() as u32
        } else {
            1
        }
    }

    fn advance_sample(&mut self) {
        if self.tap_bit == 0 {
            self.waveform_index = (self.waveform_index + 1) % self.waveform.len();
        } else {
            let feedback = (self.lfsr ^ (self.lfsr >> self.tap_bit)) & 1;
            self.lfsr = ((self.lfsr >> 1) | (feedback << 14)) & 0x7FFF;
        }

        self.update();
    }

    fn update(&mut self) {
        self.sample = if self.tap_bit == 0 {
            self.waveform[self.waveform_index]
        } else {
            if (self.lfsr & 1) == 0 {
                1.0
            } else {
                -1.0
            }
        };
    }
}

struct EnvelopeSegment {
    start_clock: u32,
    start_level: f64,
    slope: f64,
}

pub struct Envelope {
    segments: Vec<EnvelopeSegment>,

    enabled: bool,
    elapsed_clocks: u32,
    level: f64,
}

impl Envelope {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
            enabled: false,
            elapsed_clocks: 0,
            level: 0.0,
        }
    }

    pub fn set(&mut self, initial_level: f64, segments: &[(u32, f64)]) {
        self.segments.clear();

        let mut start_clock = 0;
        let mut start_level = initial_level;

        for &(duration, target_level) in segments {
            assert!(duration > 0);

            let slope = (target_level - start_level) / duration as f64;

            self.segments.insert(
                0,
                EnvelopeSegment {
                    start_clock,
                    start_level,
                    slope,
                },
            );

            start_clock += duration;
            start_level = target_level;
        }

        self.segments.insert(
            0,
            EnvelopeSegment {
                start_clock,
                start_level,
                slope: 0.0,
            },
        );
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn level(&self) -> f64 {
        self.level
    }

    pub fn reset_clock(&mut self) {
        self.elapsed_clocks = 0;
        self.update();
    }

    fn advance_clock(&mut self, clocks: u32) {
        if self.enabled {
            self.elapsed_clocks += clocks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled {
            self.level = 1.0;
            return;
        }

        for segment in &self.segments {
            if self.elapsed_clocks >= segment.start_clock {
                self.level = if segment.slope == 0.0 {
                    segment.start_level
                } else {
                    segment.start_level
                        + segment.slope * (self.elapsed_clocks - segment.start_clock) as f64
                };
                break;
            }
        }
    }
}

pub struct Vibrato {
    delay_clocks: u32,
    period_clocks: u32,
    inv_period_clocks: f64,
    semitone_depth: f64,

    enabled: bool,
    elapsed_clocks: u32,
    pitch_multiplier: f64,
}

impl Vibrato {
    fn new() -> Self {
        Self {
            delay_clocks: 0,
            period_clocks: 1,
            inv_period_clocks: 0.0,
            semitone_depth: 0.0,

            enabled: false,
            elapsed_clocks: 0,
            pitch_multiplier: 1.0,
        }
    }

    pub fn set(&mut self, delay_clocks: u32, period_clocks: u32, semitone_depth: f64) {
        assert!(period_clocks > 0);

        self.delay_clocks = delay_clocks;
        self.semitone_depth = semitone_depth;

        if period_clocks != self.period_clocks {
            self.period_clocks = period_clocks;
            self.inv_period_clocks = 1.0 / period_clocks as f64;
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn pitch_multiplier(&self) -> f64 {
        self.pitch_multiplier
    }

    fn reset_clock(&mut self) {
        if self.delay_clocks > 0 {
            self.elapsed_clocks = 0;
        }

        self.update();
    }

    fn advance_clock(&mut self, clocks: u32) {
        if self.enabled {
            self.elapsed_clocks += clocks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled || self.elapsed_clocks < self.delay_clocks {
            self.pitch_multiplier = 1.0;
            return;
        }

        let phase = (self.elapsed_clocks - self.delay_clocks) as f64 * self.inv_period_clocks;
        let modulation = 1.0 - 4.0 * ((phase + 0.25).fract() - 0.5).abs();
        let semitone_offset = modulation * self.semitone_depth;

        self.pitch_multiplier = 2.0_f64.powf(semitone_offset / 12.0);
    }
}

pub struct Glide {
    semitone_offset: f64,
    duration_clocks: u32,
    semitone_slope: f64,

    enabled: bool,
    elapsed_clocks: u32,
    pitch_multiplier: f64,
}

impl Glide {
    fn new() -> Self {
        Self {
            semitone_offset: 0.0,
            duration_clocks: 0,
            semitone_slope: 0.0,

            enabled: false,
            elapsed_clocks: 0,
            pitch_multiplier: 1.0,
        }
    }

    pub fn set(&mut self, semitone_offset: f64, duration_clocks: u32) {
        assert!(duration_clocks > 0);

        if semitone_offset != self.semitone_offset || duration_clocks != self.duration_clocks {
            self.semitone_offset = semitone_offset;
            self.duration_clocks = duration_clocks;
            self.semitone_slope = -semitone_offset / duration_clocks as f64;
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn pitch_multiplier(&self) -> f64 {
        self.pitch_multiplier
    }

    fn reset_clock(&mut self) {
        self.elapsed_clocks = 0;
        self.update();
    }

    fn advance_clock(&mut self, clocks: u32) {
        if self.enabled {
            self.elapsed_clocks += clocks;
        }

        self.update();
    }

    fn update(&mut self) {
        if !self.enabled || self.elapsed_clocks >= self.duration_clocks {
            self.pitch_multiplier = 1.0;
            return;
        }

        let semitone_offset =
            self.semitone_offset + self.semitone_slope * self.elapsed_clocks as f64;
        self.pitch_multiplier = 2.0_f64.powf(semitone_offset / 12.0);
    }
}

pub struct Voice {
    pub oscillator: Oscillator,
    pub envelope: Envelope,
    pub vibrato: Vibrato,
    pub glide: Glide,

    max_amplitude: f64,
    clock_rate: u32,
    base_frequency: f64,
    velocity: f64,
    duration_clocks: u32,
    sample_clocks: u32,
    carryover_sample_clocks: u32,
    control_interval_clocks: u32,
    control_elapsed_clocks: u32,
    last_amplitude: i32,
}

impl Voice {
    pub fn new(bit_depth: u32, clock_rate: u32, control_rate: u32) -> Self {
        assert!(bit_depth > 0 && clock_rate > 0 && control_rate > 0);

        let max_amplitude = ((1_u32 << (bit_depth - 1)) - 1) as f64;
        let control_interval_clocks = clock_rate / control_rate;

        Self {
            oscillator: Oscillator::new(),
            envelope: Envelope::new(),
            vibrato: Vibrato::new(),
            glide: Glide::new(),

            max_amplitude,
            clock_rate,
            base_frequency: 0.0,
            velocity: 0.0,
            duration_clocks: 0,
            sample_clocks: 0,
            carryover_sample_clocks: 0,
            control_interval_clocks,
            control_elapsed_clocks: 0,
            last_amplitude: 0,
        }
    }

    pub fn play_note(&mut self, midi_note: f64, velocity: f64, duration_clocks: u32) {
        self.base_frequency = A4_FREQUENCY * ((midi_note - A4_MIDI_NOTE) / 12.0).exp2();
        self.velocity = velocity;
        self.duration_clocks = duration_clocks;

        self.reset_control_clock();
    }

    pub fn cancel_note(&mut self) {
        self.duration_clocks = 0;
    }

    pub fn process(&mut self, blip_buf: &mut BlipBuf, clock_offset: u32, clock_count: u32) {
        assert!(clock_count > 0);

        let mut clock_offset = clock_offset + self.carryover_sample_clocks;
        let mut clock_count = clock_count;

        if self.carryover_sample_clocks > 0 {
            let process_clocks = self.carryover_sample_clocks.min(clock_count);
            self.duration_clocks = self.duration_clocks.saturating_sub(process_clocks);
            self.carryover_sample_clocks -= process_clocks;
            clock_count -= process_clocks;

            if self.carryover_sample_clocks > 0 {
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        while self.duration_clocks > 0 && clock_count > 0 {
            let amplitude = (self.oscillator.sample()
                * self.envelope.level()
                * self.velocity
                * self.max_amplitude)
                .round() as i32;
            self.write_sample(blip_buf, clock_offset, amplitude);

            let process_clocks = self.sample_clocks.min(clock_count);
            self.duration_clocks = self.duration_clocks.saturating_sub(process_clocks);
            clock_offset += process_clocks;
            clock_count -= process_clocks;

            if process_clocks < self.sample_clocks {
                self.carryover_sample_clocks = self.sample_clocks - process_clocks;
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        if self.duration_clocks == 0 {
            self.write_sample(blip_buf, clock_offset, 0);
        }
    }

    fn reset_control_clock(&mut self) {
        self.control_elapsed_clocks = 0;

        self.envelope.reset_clock();
        self.vibrato.reset_clock();
        self.glide.reset_clock();

        self.update_sample_clocks();
    }

    fn advance_control_clock(&mut self, clocks: u32) {
        self.control_elapsed_clocks += clocks;

        while self.control_elapsed_clocks >= self.control_interval_clocks {
            self.control_elapsed_clocks -= self.control_interval_clocks;

            self.envelope.advance_clock(self.control_interval_clocks);
            self.vibrato.advance_clock(self.control_interval_clocks);
            self.glide.advance_clock(self.control_interval_clocks);

            self.update_sample_clocks();
        }
    }

    fn update_sample_clocks(&mut self) {
        let frequency =
            self.base_frequency * self.vibrato.pitch_multiplier() * self.glide.pitch_multiplier();
        self.sample_clocks = (self.clock_rate as f64
            / frequency
            / self.oscillator.cycle_resolution() as f64)
            .round() as u32;
    }

    fn write_sample(&mut self, blip_buf: &mut BlipBuf, clock_offset: u32, amplitude: i32) {
        if amplitude != self.last_amplitude {
            blip_buf.add_delta(clock_offset as u64, amplitude - self.last_amplitude);
            self.last_amplitude = amplitude;
        }
    }
}
