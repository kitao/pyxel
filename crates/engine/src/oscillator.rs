use blip_buf::BlipBuf;

use crate::settings::{
    CLOCK_RATE, MASTER_VOLUME_FACTOR, NOISE_VOLUME_FACTOR, OSCILLATOR_RESOLUTION,
    PULSE_VOLUME_FACTOR, SQUARE_VOLUME_FACTOR, TICK_CLOCK_COUNT, TRIANGLE_VOLUME_FACTOR,
};

#[derive(Copy, Clone)]
pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

#[derive(Copy, Clone)]
pub enum Effect {
    None,
    Slide { target: f64 },
    Vibrato { depth: f64, frequency: f64 },
    FadeOut,
}

pub struct Oscillator {
    pitch: f64,
    tone: Tone,
    volume: f64,
    effect: Effect,
    duration: u32,
    time: u32,
    phase: u32,
    amplitude: i16,
    noise: u32,

    slide_change: f64,
    vibrato_period: u32,
    vibrato_time: u32,
    vibrato_phase: u32,
    fadeout_change: f64,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            pitch: 0.0,
            tone: Tone::Triangle,
            volume: 0.0,
            effect: Effect::None,
            duration: 0,
            time: 0,
            phase: 0,
            amplitude: 0,
            noise: 0x8000,

            slide_change: 0.0,
            vibrato_period: 0,
            vibrato_time: 0,
            vibrato_phase: 0,
            fadeout_change: 0.0,
        }
    }
    #[inline]
    pub fn play(&mut self, pitch: f64, tone: Tone, volume: f64, effect: Effect, duration: u32) {
        self.pitch = pitch;
        self.tone = tone;
        self.volume = volume;
        self.effect = effect;
        self.duration = duration;

        match effect {
            Effect::None => {}
            Effect::Slide { target } => {
                self.slide_change = (target - pitch) / duration as f64;
            }
            Effect::Vibrato { frequency, .. } => {
                self.vibrato_period =
                    (CLOCK_RATE as f64 / frequency / OSCILLATOR_RESOLUTION as f64) as u32;
            }
            Effect::FadeOut => {
                self.fadeout_change = -volume / duration as f64;
            }
        }
    }

    #[inline]
    pub fn stop(&mut self) {
        self.duration = 0;
    }

    #[inline]
    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        if self.duration == 0 {
            if self.amplitude != 0 {
                blip_buf.add_delta(0, -(self.amplitude as i32));
            }

            self.time = 0;
            self.amplitude = 0;

            return;
        }

        let pitch = self.pitch
            + if let Effect::Vibrato { depth, .. } = self.effect {
                Oscillator::triangle(self.vibrato_phase) * depth
            } else {
                0.0
            };
        let period = (CLOCK_RATE as f64 / pitch / OSCILLATOR_RESOLUTION as f64) as u32;

        while self.time < TICK_CLOCK_COUNT {
            let last_amplitude = self.amplitude;

            self.phase = (self.phase + 1) % OSCILLATOR_RESOLUTION;
            self.amplitude = (match self.tone {
                Tone::Triangle => Oscillator::triangle(self.phase) * TRIANGLE_VOLUME_FACTOR,
                Tone::Square => Oscillator::square(self.phase) * SQUARE_VOLUME_FACTOR,
                Tone::Pulse => Oscillator::pulse(self.phase) * PULSE_VOLUME_FACTOR,
                Tone::Noise => self.noise() * NOISE_VOLUME_FACTOR,
            } * MASTER_VOLUME_FACTOR
                * self.volume
                * i16::MAX as f64) as i16;

            blip_buf.add_delta(self.time, self.amplitude as i32 - last_amplitude as i32);

            self.time += period;
        }

        self.duration -= 1;
        self.time -= TICK_CLOCK_COUNT;

        match self.effect {
            Effect::None => {}
            Effect::Slide { .. } => {
                self.pitch += self.slide_change;
            }
            Effect::Vibrato { .. } => {
                self.vibrato_time += TICK_CLOCK_COUNT;
                let phases = self.vibrato_time / self.vibrato_period;
                self.vibrato_phase = (self.vibrato_phase + phases) % OSCILLATOR_RESOLUTION;
                self.vibrato_time %= self.vibrato_period;
            }
            Effect::FadeOut => {
                self.volume += self.fadeout_change;
            }
        }
    }

    #[inline]
    fn triangle(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 2 {
            phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64 - 1.0
        } else {
            3.0 - phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64
        }
    }

    #[inline]
    fn square(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 2 {
            1.0
        } else {
            -1.0
        }
    }

    #[inline]
    fn pulse(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 4 {
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
