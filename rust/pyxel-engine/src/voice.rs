use crate::blip_buf::BlipBuf;
use crate::settings::{CLOCKS_PER_CONTROL, CLOCK_RATE, WAVETABLE_LENGTH, WAVETABLE_LEVELS};

pub type Gain = f64;

pub type WavetableValue = u8;
pub type Wavetable = [WavetableValue; WAVETABLE_LENGTH as usize];

struct Oscillator {
    wavetable: Wavetable,
    waveform: [f64; WAVETABLE_LENGTH as usize],
    lfsr: u16,
    tap_bit: u8,
    gain: Gain,

    use_waveform: bool,
    waveform_index: usize,
    sample: f64,
}

impl Oscillator {
    fn new() -> Self {
        Self {
            wavetable: [0; WAVETABLE_LENGTH as usize],
            waveform: [0.0; WAVETABLE_LENGTH as usize],
            lfsr: 0,
            tap_bit: 0,
            gain: 0.0,

            use_waveform: true,
            waveform_index: 0,
            sample: 0.0,
        }
    }

    pub fn set(&mut self, wavetable: &Wavetable, gain: f64) {
        self.gain = gain;
        self.use_waveform = true;

        if wavetable != &self.wavetable {
            self.wavetable = *wavetable;

            let half_max = (WAVETABLE_LEVELS - 1) as f64 / 2.0;
            self.waveform = wavetable.map(|v| (v as f64 - half_max) / half_max);
        }
    }

    pub fn set_noise(&mut self, short_period: bool, gain: f64) {
        let tap_bit = if short_period { 6 } else { 1 };
        if self.use_waveform || tap_bit != self.tap_bit {
            self.lfsr = if short_period { 0x0201 } else { 0x7001 };
            self.tap_bit = tap_bit;
        }
        // The LFSR originally starts with 0x0001, but to avoid leading zeros, it is advanced first.
        // For short-period noise, it's shifted 15 times (half of the 32-sample period), resulting in 0x0201.
        // For long-period noise, it's shifted 45 times (half of the 93-sample period), resulting in 0x7001.

        self.gain = gain;
        self.use_waveform = false; // Must be after checking for waveform
    }

    fn sample(&self) -> f64 {
        self.sample
    }

    fn advance_sample(&mut self) {
        if self.use_waveform {
            self.waveform_index = (self.waveform_index + 1) % WAVETABLE_LENGTH as usize;
        } else {
            let feedback = (self.lfsr ^ (self.lfsr >> self.tap_bit)) & 1;
            self.lfsr = ((self.lfsr >> 1) | (feedback << 14)) & 0x7FFF;
        }
    }

    fn update(&mut self) {
        self.sample = if self.use_waveform {
            self.waveform[self.waveform_index] * self.gain
        } else {
            if (self.lfsr & 1) == 0 {
                self.gain
            } else {
                -self.gain
            }
        };
    }
}

struct EnvelopeSegment {
    start_clock: u32,
    end_clock: u32,
    start_level: f64,
    slope: f64,
}

struct Envelope {
    points: Vec<(u32, f64)>,
    segments: Vec<EnvelopeSegment>,

    enabled: bool,
    elapsed_clocks: u32,
    level: f64,
}

impl Envelope {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            segments: Vec::new(),

            enabled: false,
            elapsed_clocks: 0,
            level: 0.0,
        }
    }

    pub fn set(&mut self, points: &[(u32, f64)]) {
        assert!(!points.is_empty());

        if points != self.points {
            let (clock, level) = points[0];
            assert!(clock == 0);

            self.points = points.to_vec();
            self.segments.clear();

            let mut prev_clock = clock;
            let mut prev_level = level;

            for &(clock, level) in points.iter().skip(1) {
                assert!(clock >= prev_clock);

                if clock > prev_clock {
                    let slope = (level - prev_level) / (clock - prev_clock) as f64;
                    self.segments.push(EnvelopeSegment {
                        start_clock: prev_clock,
                        end_clock: clock,
                        start_level: prev_level,
                        slope,
                    });
                }

                prev_clock = clock;
                prev_level = level;
            }

            self.segments.push(EnvelopeSegment {
                start_clock: prev_clock,
                end_clock: u32::MAX,
                start_level: prev_level,
                slope: 0.0,
            });
        }
    }

    pub fn set_value(&mut self, level: f64) {
        Self::set(self, &[(0, level)]);
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
            if self.elapsed_clocks < segment.end_clock {
                self.level = segment.start_level
                    + segment.slope * (self.elapsed_clocks - segment.start_clock) as f64;
                break;
            }
        }
    }
}

struct Vibrato {
    delay_clocks: u32,
    period_clocks: u32,
    inv_period_clocks: f64,
    note_amplitude: f64,

    enabled: bool,
    elapsed_clocks: u32,
    note_offset: f64,
}

impl Vibrato {
    fn new() -> Self {
        Self {
            delay_clocks: 0,
            period_clocks: 1,
            inv_period_clocks: 0.0,
            note_amplitude: 0.0,

            enabled: false,
            elapsed_clocks: 0,
            note_offset: 0.0,
        }
    }

    pub fn set(&mut self, delay_clocks: u32, period_clocks: u32, note_amplitude: f64) {
        assert!(period_clocks > 0);

        self.delay_clocks = delay_clocks;
        self.note_amplitude = note_amplitude;

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

    fn note_offset(&self) -> f64 {
        self.note_offset
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
        if !self.enabled {
            self.note_offset = 0.0;
            return;
        }

        self.note_offset = if self.elapsed_clocks < self.delay_clocks {
            0.0
        } else {
            let phase = (self.elapsed_clocks - self.delay_clocks) as f64 * self.inv_period_clocks;
            let modulation = 1.0 - 4.0 * ((phase + 0.25).fract() - 0.5).abs();

            modulation * self.note_amplitude
        };
    }
}

struct Glide {
    note_delta: f64,
    note_slope: f64,
    duration_clocks: u32,

    enabled: bool,
    elapsed_clocks: u32,
    note_offset: f64,
}

impl Glide {
    fn new() -> Self {
        Self {
            note_delta: 0.0,
            note_slope: 0.0,
            duration_clocks: 0,

            enabled: false,
            note_offset: 0.0,
            elapsed_clocks: 0,
        }
    }

    pub fn set(&mut self, note_delta: f64, duration_clocks: u32) {
        assert!(duration_clocks > 0);

        if note_delta != self.note_delta || duration_clocks != self.duration_clocks {
            self.note_delta = note_delta;
            self.duration_clocks = duration_clocks;
            self.note_slope = note_delta / duration_clocks as f64;
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    fn note_offset(&self) -> f64 {
        self.note_offset
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
        if !self.enabled {
            self.note_offset = 0.0;
            return;
        }

        self.note_offset = if self.elapsed_clocks < self.duration_clocks {
            self.note_slope * (self.duration_clocks - self.elapsed_clocks) as f64
        } else {
            0.0
        };
    }
}

pub struct Voice {
    pub oscillator: Oscillator,
    pub envelope: Envelope,
    pub vibrato: Vibrato,
    pub glide: Glide,

    base_note: f64,
    rem_playback_clocks: u32,
    rem_sample_clocks: u32,
    control_timer: u32,
    last_sample: i16,
    cached_note: f64,
    cached_sample_clocks: u32,
}

impl Voice {
    pub fn new() -> Self {
        Self {
            oscillator: Oscillator::new(),
            envelope: Envelope::new(),
            vibrato: Vibrato::new(),
            glide: Glide::new(),

            base_note: 0.0,
            rem_playback_clocks: 0,
            rem_sample_clocks: 0,
            control_timer: 0,
            last_sample: 0,
            cached_note: 0.0,
            cached_sample_clocks: 0,
        }
    }

    pub fn play(&mut self, note: f64, duration_clocks: u32) {
        self.base_note = note;
        self.rem_playback_clocks = duration_clocks;

        self.reset_controls();
    }

    pub fn stop(&mut self) {
        self.rem_playback_clocks = 0;
    }

    pub fn process(&mut self, blip_buf: &mut BlipBuf, clock_offset: u32, clock_count: u32) {
        assert!(clock_count > 0);

        let mut clock_offset = clock_offset + self.rem_sample_clocks;
        let mut rem_process_clocks = clock_count;

        if self.rem_sample_clocks > 0 {
            self.rem_playback_clocks = self.rem_playback_clocks.saturating_sub(rem_process_clocks);

            if self.rem_sample_clocks >= rem_process_clocks {
                self.rem_sample_clocks -= rem_process_clocks;
                return;
            }

            rem_process_clocks -= self.rem_sample_clocks;
            self.rem_sample_clocks = 0;
        }

        loop {
            if self.rem_playback_clocks == 0 {
                self.write_sample(blip_buf, clock_offset, 0);
                return;
            }

            let sample =
                (self.oscillator.sample() * self.envelope.level() * i16::MAX as f64) as i16;
            let note = self.base_note + self.vibrato.note_offset() + self.glide.note_offset();
            let sample_clocks = self.note_to_sample_clocks(note);

            self.write_sample(blip_buf, clock_offset, sample);

            clock_offset += sample_clocks;
            self.rem_playback_clocks = self.rem_playback_clocks.saturating_sub(sample_clocks);
            self.update_controls(sample_clocks);

            if sample_clocks >= rem_process_clocks {
                self.rem_sample_clocks = sample_clocks - rem_process_clocks;
                return;
            }

            rem_process_clocks -= sample_clocks;
        }
    }

    fn reset_controls(&mut self) {
        self.control_timer = 0;

        self.envelope.reset_clock();
        self.vibrato.reset_clock();
        self.glide.reset_clock();
    }

    fn update_controls(&mut self, delta_clocks: u32) {
        self.control_timer += delta_clocks;

        if self.control_timer >= CLOCKS_PER_CONTROL {
            self.control_timer -= CLOCKS_PER_CONTROL;

            self.envelope.advance_clock(self.control_timer);
            self.vibrato.advance_clock(self.control_timer);
            self.glide.advance_clock(self.control_timer);
        }
    }

    fn write_sample(&mut self, blip_buf: &mut BlipBuf, clock_offset: u32, sample: i16) {
        if sample != self.last_sample {
            blip_buf.add_delta(clock_offset as u64, (sample - self.last_sample) as i32);
            self.last_sample = sample;
        }
    }

    fn note_to_sample_clocks(&mut self, note: f64) -> u32 {
        if note == self.cached_note {
            return self.cached_sample_clocks;
        }

        let semitone_offset = (note - 69.0) / 12.0;
        let pitch = 440.0 * semitone_offset.exp2();
        let sample_clocks = (CLOCK_RATE as f64 / pitch / WAVETABLE_LENGTH as f64).round() as u32;

        self.cached_note = note;
        self.cached_sample_clocks = sample_clocks;

        sample_clocks
    }
}
