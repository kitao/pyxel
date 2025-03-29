use crate::blip_buf::BlipBuf;
use crate::pyxel::TONES;
use crate::settings::{
    CLOCKS_PER_TICK, CLOCK_RATE, EFFECT_FADEOUT, EFFECT_HALF_FADEOUT, EFFECT_NONE,
    EFFECT_QUARTER_FADEOUT, EFFECT_SLIDE, EFFECT_VIBRATO, INITIAL_NOISE_REG, OSCILLATOR_RESOLUTION,
    TONE_TRIANGLE, VIBRATO_DEPTH, VIBRATO_FREQUENCY,
};

pub type Gain = f64;
pub type ToneIndex = u16;
pub type Effect = u16;

const VIBRATO_PERIOD: u32 =
    (CLOCK_RATE as f64 / VIBRATO_FREQUENCY / OSCILLATOR_RESOLUTION as f64) as u32;

struct Slide {
    pitch: f64,
}

struct Vibrato {
    clock: u32,
    phase: u32,
}

struct FadeOut {
    duration: u32,
    delta: Gain,
}

pub struct Oscillator {
    pitch: f64,
    tone: ToneIndex,
    gain: Gain,
    effect: Effect,
    duration: u32,
    clock: u32,
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
            clock: 0,
            phase: 0,
            amplitude: 0,
            noise_reg: INITIAL_NOISE_REG,
            slide: Slide { pitch: 0.0 },
            vibrato: Vibrato { clock: 0, phase: 0 },
            fadeout: FadeOut {
                duration: 0,
                delta: 0.0,
            },
        }
    }

    pub fn play(&mut self, note: f64, tone: ToneIndex, gain: Gain, effect: Effect, duration: u32) {
        let last_pitch = self.pitch;
        self.pitch = Self::note_to_pitch(note);
        self.tone = tone;
        self.gain = gain;
        self.effect = effect;
        self.duration = duration;

        match effect {
            EFFECT_NONE | EFFECT_VIBRATO => {}
            EFFECT_SLIDE => {
                self.slide.pitch = (self.pitch - last_pitch) / self.duration as f64;
                self.pitch = last_pitch;
            }
            EFFECT_FADEOUT | EFFECT_HALF_FADEOUT | EFFECT_QUARTER_FADEOUT => {
                self.fadeout.duration = duration;
                if effect == EFFECT_HALF_FADEOUT {
                    self.fadeout.duration /= 2;
                } else if effect == EFFECT_QUARTER_FADEOUT {
                    self.fadeout.duration /= 4;
                }
                self.fadeout.delta = -self.gain / self.fadeout.duration as f64;
            }
            _ => panic!("Invalid effect '{}'", self.effect),
        }
    }

    pub fn stop(&mut self) {
        self.duration = 0;
    }

    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        // Mute sound
        if self.duration == 0 {
            if self.amplitude != 0 {
                let delta = if self.amplitude > 0 { -1 } else { 1 };
                for i in 0..CLOCKS_PER_TICK {
                    self.amplitude += delta;
                    blip_buf.add_delta(i as u64, delta as i32);
                    if self.amplitude == 0 {
                        break;
                    }
                }
            }

            self.clock = 0;
            self.phase = 0;
            self.vibrato.clock = 0;
            self.vibrato.phase = 0;
            return;
        }

        // Apply effect
        match self.effect {
            EFFECT_SLIDE => {
                self.pitch += self.slide.pitch;
            }
            EFFECT_VIBRATO => {
                self.vibrato.clock += CLOCKS_PER_TICK;
                self.vibrato.phase = (self.vibrato.phase + self.vibrato.clock / VIBRATO_PERIOD)
                    % OSCILLATOR_RESOLUTION;
                self.vibrato.clock %= VIBRATO_PERIOD;
            }
            _ => {}
        }

        // Play sound
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

        let fade_delta = if self.effect >= EFFECT_FADEOUT && self.duration <= self.fadeout.duration
        {
            self.fadeout.delta / ((CLOCKS_PER_TICK - self.clock) / period) as f64
        } else {
            0.0
        };

        while self.clock < CLOCKS_PER_TICK {
            let last_amplitude = self.amplitude;
            self.phase = (self.phase + 1) % OSCILLATOR_RESOLUTION;
            self.amplitude = (tone.amplitude(self.phase, &mut self.noise_reg)
                * self.gain
                * i16::MAX as f64) as i16;
            blip_buf.add_delta(
                self.clock as u64,
                self.amplitude as i32 - last_amplitude as i32,
            );
            self.clock += period;
            self.gain += fade_delta;
        }

        self.duration -= 1;
        if self.duration == 0 && self.effect >= EFFECT_FADEOUT {
            self.clock = 0;
            self.phase = 0;
            self.vibrato.clock = 0;
            self.vibrato.phase = 0;
        } else {
            self.clock -= CLOCKS_PER_TICK;
        }
    }

    fn note_to_pitch(note: f64) -> f64 {
        440.0 * ((note - 33.0) / 12.0).exp2()
    }
}
