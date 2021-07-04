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
    Slide,
    Vibrato,
    FadeOut,
}

pub struct Oscillator {
    frequency: f64,
    period: u32,
    duration: u32,
    tone: Tone,
    volume: f64,
    effect: Effect,
    time: u32,
    phase: u32,
    amplitude: i16,
    noise: u32,
}

impl Oscillator {
    pub fn new() -> Oscillator {
        Oscillator {
            frequency: 0.0,
            period: 0,
            duration: 0,
            tone: Tone::Triangle,
            volume: 0.0,
            effect: Effect::None,
            time: 0,
            phase: 0,
            amplitude: 0,
            noise: 0x8000,
        }
    }

    #[inline]
    pub fn play(&mut self, frequency: f64, duration: u32, tone: Tone, volume: f64, effect: Effect) {
        self.frequency = frequency;
        self.period = (CLOCK_RATE as f64 / frequency / WAVE_RESOLUTION as f64) as u32;
        self.duration = duration;
        self.tone = tone;
        self.volume = volume;
        self.effect = effect;
    }

    #[inline]
    pub fn update(&mut self, blip_buf: &mut BlipBuf) {
        if self.duration == 0 {
            if self.amplitude != 0 {
                blip_buf.add_delta(0, -(self.amplitude as i32));
                self.amplitude = 0;
            }

            return;
        }

        while self.time < TICK_CLOCK_COUNT {
            self.phase = (self.phase + 1) % WAVE_RESOLUTION;

            let last_amplitude = self.amplitude;
            match self.tone {
                Tone::Triangle => self.update_triangle(),
                Tone::Square => self.update_square(),
                Tone::Pulse => self.update_pulse(),
                Tone::Noise => self.update_noise(),
            };
            blip_buf.add_delta(self.time, self.amplitude as i32 - last_amplitude as i32);

            self.time += self.period;
        }

        match self.effect {
            Effect::None => (),
            Effect::Slide => self.update_slide(),
            Effect::Vibrato => self.update_vibrato(),
            Effect::FadeOut => self.update_fadeout(),
        };

        self.duration -= 1;
        self.time -= TICK_CLOCK_COUNT;
    }

    #[inline]
    fn update_triangle(&mut self) {
        let volume = self.volume * MASTER_VOLUME_FACTOR * TRIANGLE_VOLUME_FACTOR * i16::MAX as f64;

        self.amplitude = ((if self.phase < WAVE_RESOLUTION / 2 {
            self.phase as f64 / (WAVE_RESOLUTION / 4) as f64 - 1.0
        } else {
            3.0 - self.phase as f64 / (WAVE_RESOLUTION / 4) as f64
        }) as f64
            * volume) as i16;
    }

    #[inline]
    fn update_square(&mut self) {
        let volume = self.volume * MASTER_VOLUME_FACTOR * SQUARE_VOLUME_FACTOR * i16::MAX as f64;

        self.amplitude = ((if self.phase < WAVE_RESOLUTION / 2 {
            1.0
        } else {
            -1.0
        }) * volume) as i16;
    }

    #[inline]
    fn update_pulse(&mut self) {
        let volume = self.volume * MASTER_VOLUME_FACTOR * PULSE_VOLUME_FACTOR * i16::MAX as f64;

        self.amplitude = ((if self.phase < WAVE_RESOLUTION / 4 {
            1.0
        } else {
            -1.0
        }) * volume) as i16;
    }

    #[inline]
    fn update_noise(&mut self) {
        let volume = self.volume * MASTER_VOLUME_FACTOR * NOISE_VOLUME_FACTOR * i16::MAX as f64;

        self.noise >>= 1;
        self.noise |= ((self.noise ^ (self.noise >> 1)) & 1) << 15;
        self.amplitude = (((self.noise & 1) as f64 * 2.0 - 1.0) * volume) as i16;
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
        /*
        inline float Channel::Lfo(int32_t time) {
          float x = (time * 8.0f / AUDIO_SAMPLE_RATE + 0.25f);
          x -= static_cast<int32_t>(x);

          return Abs(x * 4.0f - 2.0f) - 1.0f;
        }
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
}
