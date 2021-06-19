use blip_buf::BlipBuf;

use crate::settings::{
    MASTER_VOLUME_FACTOR, NOISE_VOLUME_FACTOR, PULSE_VOLUME_FACTOR, SQUARE_VOLUME_FACTOR,
    TRIANGLE_VOLUME_FACTOR,
};

pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

pub enum Effect {
    None,
    Slide,
    Vibrato,
    FadeOut,
}

pub struct Oscillator {
    elapsed: u32,
    noise_seed: u32,

    period: u32,
    duration: u32,
    tone: Tone,
    volume: f64,
    effect: Effect,
    phase: u32,
    amplitude: u16,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            elapsed: 0,
            noise_seed: 0x8000,

            period: 0,
            duration: 0,
            tone: Tone::Triangle,
            volume: 0.0,
            effect: Effect::None,
            phase: 0,
            amplitude: 0,
        }
    }

    #[inline]
    pub fn play(&mut self, period: u32, duration: u32, tone: Tone, volume: u8, effect: Effect) {
        let last_period = self.period;

        self.period = period / 32;
        self.duration = duration;
        self.tone = tone;
        self.volume = volume as f64;
        self.effect = effect;

        if self.period != last_period {
            self.phase = 0;
            self.elapsed = 0;
        }
    }

    #[inline]
    pub fn update(&mut self, blip_buf: &mut BlipBuf, start_time: u32, clocks: u32) {
        assert!(clocks > 0);

        let mut cur_time = start_time;
        let end_time = start_time + clocks;

        while cur_time < end_time {
            if self.duration == 0 {
                if self.amplitude > 0 {
                    blip_buf.add_delta(cur_time, -(self.amplitude as i32));
                }

                break;
            }

            let remaining = end_time - cur_time;

            if remaining < self.period && remaining < self.duration {
                if self.duration < self.period {
                    cur_time += self.duration;
                    self.duration = 0;
                } else {
                    cur_time += self.period;
                    self.duration -= self.period;
                }

                continue;
            }

            if self.duration < self.period && self.duration < remaining {
                cur_time += self.duration;
                self.duration = 0;

                continue;
            }

            let last_amplitude = self.amplitude;

            match self.tone {
                Tone::Triangle => self.update_triangle(),
                Tone::Square => self.update_square(),
                Tone::Pulse => self.update_pulse(),
                Tone::Noise => self.update_noise(),
            };

            blip_buf.add_delta(cur_time, self.amplitude as i32 - last_amplitude as i32);

            self.phase = (self.phase + 1) % 32;
            self.duration -= self.period;
            cur_time += self.period;

            match self.effect {
                Effect::None => break,
                Effect::Slide => break,
                Effect::Vibrato => break,
                Effect::FadeOut => break,
            }
        }

        self.elapsed = cur_time - end_time;
    }

    #[inline]
    fn update_triangle(&mut self) {
        self.amplitude = ((if self.phase < 16 {
            self.phase
        } else {
            31 - self.phase
        }) as f64
            / 15.0
            * u16::MAX as f64
            * self.volume as f64
            / u8::MAX as f64
            * MASTER_VOLUME_FACTOR
            * TRIANGLE_VOLUME_FACTOR) as u16;
    }

    #[inline]
    fn update_square(&mut self) {
        self.amplitude = ((if self.phase < 16 { u16::MAX } else { 0 }) as f64 * self.volume as f64
            / u8::MAX as f64
            * MASTER_VOLUME_FACTOR
            * SQUARE_VOLUME_FACTOR) as u16;
    }

    #[inline]
    fn update_pulse(&mut self) {
        self.amplitude = ((if self.phase < 8 { u16::MAX } else { 0 }) as f64 * self.volume as f64
            / u8::MAX as f64
            * MASTER_VOLUME_FACTOR
            * PULSE_VOLUME_FACTOR) as u16;
    }

    #[inline]
    fn update_noise(&mut self) {
        self.noise_seed >>= 1;
        self.noise_seed |= ((self.noise_seed ^ (self.noise_seed >> 1)) & 1) << 15;

        self.amplitude = ((self.noise_seed & 1) as f64 * self.volume as f64 / u8::MAX as f64
            * MASTER_VOLUME_FACTOR
            * NOISE_VOLUME_FACTOR) as u16;
    }

    #[inline]
    fn update_slide(&mut self) {
        /*
        effect_time_ = time_;
        effect_pitch_ = last_pitch > 0.0f ? last_pitch : pitch_;
        */
        /*
        a = static_cast<float>(time_ - effect_time_) / one_note_time_;
        pitch = pitch_ * a + effect_pitch_ * (1.0f - a);
        oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
        */
    }

    #[inline]
    fn update_vibrato(&mut self) {
        /*
        effect_time_ = time_;
        effect_pitch_ = NoteToPitch(note_ + 0.5f) - pitch_;
        */
        /*
        pitch = pitch_ + Lfo(time_) * effect_pitch_;
        oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
        */
    }

    #[inline]
    fn update_fadeout(&mut self) {
        /*
        effect_time_ = time_;
        effect_volume_ = volume_;
        */
        /*
        oscillator_.SetVolume(
            static_cast<float>(effect_volume_) *
            (1.0f - static_cast<float>(time_ - effect_time_) / one_note_time_));
        */
    }

    /*
    float Channel::Lfo(int32_t time) {
        float x = (time * 8.0f / AUDIO_SAMPLE_RATE + 0.25f);
        x -= static_cast<int32_t>(x);

        return Abs(x * 4.0f - 2.0f) - 1.0f;
    }
    */
}
