use once_cell::sync::Lazy;
use parking_lot::Mutex;

use crate::blip_buf::BlipBuf;
use crate::pyxel::TONES;
use crate::settings::{
    CLOCK_RATE, EFFECT_FADEOUT, EFFECT_NONE, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_NOISE_REG,
    NUM_CLOCKS_PER_TICK, OSCILLATOR_RESOLUTION, TONE_TRIANGLE, VIBRATO_DEPTH, VIBRATO_FREQUENCY,
};

pub type Gain = f64;
pub type Effect = u8;

const VIBRATO_PERIOD: u32 =
    (CLOCK_RATE as f64 / VIBRATO_FREQUENCY / OSCILLATOR_RESOLUTION as f64) as u32;

static RNG_STATE: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

struct Slide {
    pitch: f64,
}

struct Vibrato {
    time: u32,
    phase: u32,
}

struct FadeOut {
    gain: Gain,
}

pub struct Oscillator {
    pitch: f64,
    tone: u32,
    gain: Gain,
    effect: Effect,
    duration: u32,
    time: u32,
    phase: u32,
    amplitude: i16,
    noise_reg: u16,
    slide: Slide,
    vibrato: Vibrato,
    fadeout: FadeOut,
}

impl Oscillator {
    pub fn new() -> Self {
        Self {
            pitch: Self::note_to_pitch(0.0),
            tone: TONE_TRIANGLE,
            gain: 0.0,
            effect: EFFECT_NONE,
            duration: 0,
            time: 0,
            phase: 0,
            amplitude: 0,
            noise_reg: INITIAL_NOISE_REG,
            slide: Slide { pitch: 0.0 },
            vibrato: Vibrato { time: 0, phase: 0 },
            fadeout: FadeOut { gain: 0.0 },
        }
    }

    pub fn play(&mut self, note: f64, tone: u32, gain: Gain, effect: Effect, duration: u32) {
        let last_pitch = self.pitch;
        self.pitch = Self::note_to_pitch(note);
        self.tone = tone;
        self.gain = gain;
        self.effect = effect;
        self.duration = duration;
        if self.time == 0 {
            let mut rng_state = RNG_STATE.lock();
            *rng_state = rng_state.wrapping_mul(214013).wrapping_add(2531011);
            self.phase = (*rng_state >> 16) & 31;
        }
        if effect == EFFECT_SLIDE {
            self.slide.pitch = (self.pitch - last_pitch) / self.duration as f64;
            self.pitch = last_pitch;
        } else if effect == EFFECT_FADEOUT {
            self.fadeout.gain = -self.gain / self.duration as f64;
        }
    }

    pub fn stop(&mut self) {
        self.duration = 0;
    }

    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        if self.duration == 0 {
            self.time = 0;
            return;
        }
        let pitch = self.pitch
            + if self.effect == EFFECT_VIBRATO {
                self.pitch
                    * (if self.vibrato.phase < OSCILLATOR_RESOLUTION / 2 {
                        self.vibrato.phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64 - 1.0
                    } else {
                        3.0 - self.vibrato.phase as f64 / (OSCILLATOR_RESOLUTION / 4) as f64
                    })
                    * VIBRATO_DEPTH
            } else {
                0.0
            };
        let period = (CLOCK_RATE as f64 / pitch / OSCILLATOR_RESOLUTION as f64) as u32;
        let tones = TONES.lock();
        let tone = tones[self.tone as usize].lock();
        while self.time < NUM_CLOCKS_PER_TICK {
            let last_amplitude = self.amplitude;
            self.phase = (self.phase + 1) % OSCILLATOR_RESOLUTION;
            self.amplitude = (tone.amplitude(self.phase, &mut self.noise_reg)
                * self.gain
                * i16::MAX as f64) as i16;
            blip_buf.add_delta(
                self.time as u64,
                self.amplitude as i32 - last_amplitude as i32,
            );
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
                self.gain += self.fadeout.gain;
            }
            _ => panic!("Invalid effect '{}'", self.effect),
        }
        self.duration -= 1;
        self.time -= NUM_CLOCKS_PER_TICK;
    }

    fn note_to_pitch(note: f64) -> f64 {
        440.0 * ((note - 33.0) / 12.0).exp2()
    }
}
