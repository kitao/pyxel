use crate::blippers::BlipBuf;

const A4_MIDI_NOTE: f32 = 69.0;
const A4_FREQUENCY: f32 = 440.0;

pub struct Oscillator {
    waveform: Vec<f32>,
    waveform_index: usize,

    lfsr: u16,
    tap_bit: u8,

    sample: f32,
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

    pub fn set(&mut self, waveform: &[f32]) {
        assert!(!waveform.is_empty());

        if waveform != self.waveform {
            self.waveform = waveform.to_vec();
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

        if !self.waveform.is_empty() {
            self.waveform.clear();
        }

        self.update();
    }

    fn sample(&self) -> f32 {
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
        } else if (self.lfsr & 1) == 0 {
            1.0
        } else {
            -1.0
        };
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

    enabled: bool,
    elapsed_ticks: u32,
    level: f32,
}

impl Envelope {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
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

        for segment in &self.segments {
            if self.elapsed_ticks >= segment.start_tick {
                self.level = if segment.slope == 0.0 {
                    segment.start_level
                } else {
                    segment.start_level
                        + segment.slope * (self.elapsed_ticks - segment.start_tick) as f32
                };
                break;
            }
        }
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

        self.pitch_multiplier = 2.0_f32.powf(semitone_offset / 12.0);
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
        self.pitch_multiplier = 2.0_f32.powf(semitone_offset / 12.0);
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
    playback_clocks: u32,
    sample_clocks: u32,
    carryover_sample_clocks: u32,
    control_interval_clocks: u32,
    control_elapsed_clocks: u32,
    last_amplitude: i32,
}

impl Voice {
    pub fn new(clock_rate: u32, control_rate: u32) -> Self {
        assert!(clock_rate > 0 && control_rate > 0);

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
            playback_clocks: 0,
            sample_clocks: 0,
            carryover_sample_clocks: 0,
            control_interval_clocks,
            control_elapsed_clocks: 0,
            last_amplitude: 0,
        }
    }

    pub fn set_clocks_per_tick(&mut self, clocks_per_tick: u32) {
        assert!(clocks_per_tick > 0);

        self.clocks_per_tick = clocks_per_tick;
    }

    pub fn play_note(&mut self, midi_note: f32, velocity: f32, playback_clocks: u32) {
        self.base_frequency = A4_FREQUENCY * ((midi_note - A4_MIDI_NOTE) / 12.0).exp2();
        self.velocity = velocity;
        self.playback_clocks = playback_clocks;

        self.reset_control_clock();
    }

    pub fn cancel_note(&mut self) {
        self.playback_clocks = 0;
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
            self.playback_clocks = self.playback_clocks.saturating_sub(process_clocks);
            self.carryover_sample_clocks -= process_clocks;
            clock_count -= process_clocks;

            if self.carryover_sample_clocks > 0 {
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        while self.playback_clocks > 0 && clock_count > 0 {
            let amplitude = (self.oscillator.sample()
                * self.envelope.level()
                * self.velocity
                * i16::MAX as f32)
                .round() as i32;
            self.write_sample(blip_buf.as_deref_mut(), clock_offset, amplitude);

            let process_clocks = self.sample_clocks.min(clock_count);
            self.playback_clocks = self.playback_clocks.saturating_sub(process_clocks);
            clock_offset += process_clocks;
            clock_count -= process_clocks;

            if process_clocks < self.sample_clocks {
                self.carryover_sample_clocks = self.sample_clocks - process_clocks;
                return;
            }

            self.oscillator.advance_sample();
            self.advance_control_clock(self.sample_clocks);
        }

        /*if self.playback_clocks == 0 && clock_count > 0 {
            self.write_sample(blip_buf, clock_offset, 0);
        }*/
    }

    fn reset_control_clock(&mut self) {
        self.control_elapsed_clocks = 0;

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
                blip_buf.add_delta_fast(clock_offset as u64, amplitude - self.last_amplitude);
                self.last_amplitude = amplitude;
            }
        }
    }
}
