use blip_buf::BlipBuf;

use crate::settings::{
    CLOCK_RATE, MASTER_VOLUME_FACTOR, NOISE_VOLUME_FACTOR, PULSE_VOLUME_FACTOR,
    SQUARE_VOLUME_FACTOR, TICK_CLOCK_COUNT, TRIANGLE_VOLUME_FACTOR,
};

pub const WAVE_RESOLUTION: u32 = 32;

pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

pub enum Effect {
    None,
    Slide { pitch: f64, duration: u32 },
    Vibrato { depth: f64, period: u32 },
    FadeOut { duration: f64 },
}

pub struct Oscillator {
    pitch: f64,
    tone: Tone,
    volume: f64,
    time: u32,
    phase: u32,
    amplitude: i16,
    noise: u32,

    slide_speed: f64,
    vibrato_depth: f64,
    vibrato_period: u32,
    vibrato_time: u32,
    vibrato_phase: u32,
    fadeout_speed: f64,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            pitch: 0.0,
            tone: Tone::Triangle,
            volume: 0.0,
            time: 0,
            phase: 0,
            amplitude: 0,
            noise: 0x8000,

            slide_speed: 0.0,
            vibrato_depth: 0.0,
            vibrato_period: 0,
            vibrato_time: 0,
            vibrato_phase: 0,
            fadeout_speed: 0.0,
        }
    }
    #[inline]
    pub fn play(&mut self, pitch: f64, tone: Tone, volume: f64, effect: Effect) {
        self.pitch = pitch;
        self.tone = tone;
        self.volume = volume;

        match effect {
            Effect::None => (),

            Effect::Slide {
                pitch: target_pitch,
                duration,
            } => self.slide_speed = (target_pitch - pitch) / duration as f64,

            Effect::Vibrato { depth, period } => {
                self.vibrato_depth = depth;
                self.vibrato_period = period / WAVE_RESOLUTION;
            }

            Effect::FadeOut { duration } => self.fadeout_speed = volume / duration as f64,
        };
    }

    #[inline]
    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        let pitch = self.pitch
            + if self.vibrato_period > 0 {
                Oscillator::triangle(self.vibrato_phase) * self.vibrato_depth
            } else {
                0.0
            };
        let period = (CLOCK_RATE as f64 / pitch / WAVE_RESOLUTION as f64) as u32;

        while self.time < TICK_CLOCK_COUNT {
            let last_amplitude = self.amplitude;

            self.phase = (self.phase + 1) % WAVE_RESOLUTION;
            self.amplitude = (match self.tone {
                Tone::Triangle => Oscillator::triangle(self.phase) * TRIANGLE_VOLUME_FACTOR,
                Tone::Square => Oscillator::square(self.phase) * SQUARE_VOLUME_FACTOR,
                Tone::Pulse => Oscillator::pulse(self.phase) * PULSE_VOLUME_FACTOR,
                Tone::Noise => self.noise() * NOISE_VOLUME_FACTOR,
            } * self.volume
                * MASTER_VOLUME_FACTOR
                * i16::MAX as f64) as i16;

            blip_buf.add_delta(self.time, self.amplitude as i32 - last_amplitude as i32);

            self.time += period;
        }

        self.time -= TICK_CLOCK_COUNT;

        // slide
        self.pitch = (self.pitch + self.slide_speed).clamp(1.0, 10000.0);

        // vibrato
        if self.vibrato_period > 0 {
            self.vibrato_time += TICK_CLOCK_COUNT;
            let phases = self.vibrato_time / self.vibrato_period;
            self.vibrato_phase = (self.vibrato_phase + phases) % WAVE_RESOLUTION;
            self.vibrato_time %= self.vibrato_period;
        }

        // fadeout
        self.volume = (self.volume - self.fadeout_speed).clamp(0.0, 1.0);
    }

    #[inline]
    fn triangle(phase: u32) -> f64 {
        if phase < WAVE_RESOLUTION / 2 {
            phase as f64 / (WAVE_RESOLUTION / 4) as f64 - 1.0
        } else {
            3.0 - phase as f64 / (WAVE_RESOLUTION / 4) as f64
        }
    }

    #[inline]
    fn square(phase: u32) -> f64 {
        if phase < WAVE_RESOLUTION / 2 {
            1.0
        } else {
            -1.0
        }
    }

    #[inline]
    fn pulse(phase: u32) -> f64 {
        if phase < WAVE_RESOLUTION / 4 {
            1.0
        } else {
            -1.0
        }
    }

    #[inline]
    fn noise(&mut self) -> f64 {
        self.noise >>= 1;
        self.noise |= ((self.noise ^ (self.noise >> 1)) & 1) << 15;

        (self.noise & 1) as f64 * 2.0 - 1.0
    }
}
