use blip_buf::BlipBuf;

use crate::settings::{
    CLOCK_RATE, MASTER_VOLUME_FACTOR, NOISE_VOLUME_FACTOR, OSCILLATOR_RESOLUTION,
    PULSE_VOLUME_FACTOR, SQUARE_VOLUME_FACTOR, TICK_CLOCK_COUNT, TRIANGLE_VOLUME_FACTOR,
    VIBRATO_DEPTH, VIBRATO_FREQUENCY,
};

const VIBRATO_PERIOD: u32 =
    (CLOCK_RATE as f64 / VIBRATO_FREQUENCY / OSCILLATOR_RESOLUTION as f64) as u32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Effect {
    None,
    Slide,
    Vibrato,
    FadeOut,
}

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

            slide: Slide { pitch: 0.0 },
            vibrato: Vibrato { time: 0, phase: 0 },
            fadeout: FadeOut { volume: 0.0 },
        }
    }

    #[inline]
    pub fn play(&mut self, note: f64, tone: Tone, volume: f64, effect: Effect, duration: u32) {
        let last_pitch = self.pitch;

        self.pitch = Oscillator::note_to_pitch(note);
        self.tone = tone;
        self.volume = volume;
        self.effect = effect;
        self.duration = duration;

        if effect == Effect::Slide {
            self.slide.pitch = (self.pitch - last_pitch) / self.duration as f64;
            self.pitch = last_pitch;
        } else if effect == Effect::FadeOut {
            self.fadeout.volume = -self.volume / self.duration as f64;
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
            * if self.effect == Effect::Vibrato {
                Oscillator::triangle(self.vibrato.phase) * VIBRATO_DEPTH
            } else {
                1.0
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

        match self.effect {
            Effect::None => {}
            Effect::Slide => {
                self.pitch += self.slide.pitch;
            }
            Effect::Vibrato => {
                self.vibrato.time += TICK_CLOCK_COUNT;
                let phases = self.vibrato.time / VIBRATO_PERIOD;
                self.vibrato.phase = (self.vibrato.phase + phases) % OSCILLATOR_RESOLUTION;
                self.vibrato.time %= VIBRATO_PERIOD;
            }
            Effect::FadeOut => {
                self.volume += self.fadeout.volume;
            }
        }

        self.duration -= 1;
        self.time -= TICK_CLOCK_COUNT;
    }

    #[inline]
    fn note_to_pitch(note: f64) -> f64 {
        440.0 * 2.0_f64.powf((note - 33.0) as f64 / 12.0)
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
