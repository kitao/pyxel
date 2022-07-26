use blip_buf::BlipBuf;

use crate::settings::{
    CLOCK_RATE, EFFECT_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, NOISE_VOLUME_FACTOR,
    NUM_CLOCKS_PER_TICK, OSCILLATOR_RESOLUTION, PULSE_VOLUME_FACTOR, SQUARE_VOLUME_FACTOR,
    TONE_NOISE, TONE_PULSE, TONE_SQUARE, TONE_TRIANGLE, TRIANGLE_VOLUME_FACTOR, VIBRATO_DEPTH,
    VIBRATO_FREQUENCY,
};
use crate::types::{Effect, Tone};

const VIBRATO_PERIOD: u32 =
    (CLOCK_RATE as f64 / VIBRATO_FREQUENCY / OSCILLATOR_RESOLUTION as f64) as u32;

struct Slide {
    pitch: f64,
}

struct Vibrato {
    time: u32,
    phase: u32,
}

struct FadeOut {
    volume: f64,
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
    slide: Slide,
    vibrato: Vibrato,
    fadeout: FadeOut,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            pitch: Self::note_to_pitch(0.0),
            tone: TONE_TRIANGLE,
            volume: 0.0,
            effect: EFFECT_NONE,
            duration: 0,
            time: 0,
            phase: 0,
            amplitude: 0,
            noise: 1,
            slide: Slide { pitch: 0.0 },
            vibrato: Vibrato { time: 0, phase: 0 },
            fadeout: FadeOut { volume: 0.0 },
        }
    }

    pub fn play(&mut self, note: f64, tone: Tone, volume: f64, effect: Effect, duration: u32) {
        let last_pitch = self.pitch;
        self.pitch = Self::note_to_pitch(note);
        self.tone = tone;
        self.volume = volume;
        self.effect = effect;
        self.duration = duration;
        if effect == EFFECT_SLIDE {
            self.slide.pitch = (self.pitch - last_pitch) / self.duration as f64;
            self.pitch = last_pitch;
        } else if effect == EFFECT_FADEOUT {
            self.fadeout.volume = -self.volume / self.duration as f64;
        }
    }

    pub fn stop(&mut self) {
        self.duration = 0;
    }

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
            + if self.effect == EFFECT_VIBRATO {
                self.pitch * Self::triangle(self.vibrato.phase) * VIBRATO_DEPTH
            } else {
                0.0
            };
        let period = (CLOCK_RATE as f64 / pitch / OSCILLATOR_RESOLUTION as f64) as u32;

        while self.time < NUM_CLOCKS_PER_TICK {
            let last_amplitude = self.amplitude;
            self.phase = (self.phase + 1) % OSCILLATOR_RESOLUTION;
            self.amplitude = (match self.tone {
                TONE_TRIANGLE => Self::triangle(self.phase) * TRIANGLE_VOLUME_FACTOR,
                TONE_SQUARE => Self::square(self.phase) * SQUARE_VOLUME_FACTOR,
                TONE_PULSE => Self::pulse(self.phase) * PULSE_VOLUME_FACTOR,
                TONE_NOISE => self.noise(self.phase) * NOISE_VOLUME_FACTOR,
                _ => panic!("Invalid tone '{}'", self.tone),
            } * self.volume
                * i16::MAX as f64) as i16;
            blip_buf.add_delta(self.time, self.amplitude as i32 - last_amplitude as i32);
            self.time += period;
        }

        match self.effect {
            EFFECT_NONE => {}
            EFFECT_SLIDE => {
                self.pitch += self.slide.pitch;
            }
            EFFECT_VIBRATO => {
                self.vibrato.time += NUM_CLOCKS_PER_TICK;
                self.vibrato.phase = (self.vibrato.phase + self.vibrato.time / VIBRATO_PERIOD)
                    % OSCILLATOR_RESOLUTION;
                self.vibrato.time %= VIBRATO_PERIOD;
            }
            EFFECT_FADEOUT => {
                self.volume += self.fadeout.volume;
            }
            _ => panic!("Invalid effect '{}'", self.effect),
        }

        self.duration -= 1;
        self.time -= NUM_CLOCKS_PER_TICK;
    }

    fn note_to_pitch(note: f64) -> f64 {
        440.0 * ((note - 33.0) / 12.0).exp2()
    }

    fn triangle(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 2 {
            phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64 - 1.0
        } else {
            3.0 - phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64
        }
    }

    fn square(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 2 {
            1.0
        } else {
            -1.0
        }
    }

    fn pulse(phase: u32) -> f64 {
        if phase < OSCILLATOR_RESOLUTION / 4 {
            1.0
        } else {
            -1.0
        }
    }

    fn noise(&mut self, phase: u32) -> f64 {
        if phase % 8 == 0 {
            let feedback = (self.noise & 1) ^ (self.noise >> 1 & 1);
            self.noise >>= 1;
            self.noise |= feedback << 14;
        }
        (self.noise & 1) as f64 * 2.0 - 1.0
    }
}
