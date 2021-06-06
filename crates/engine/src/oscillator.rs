use crate::settings::{
    NOISE_VOLUME_FACTOR, PULSE_VOLUME_FACTOR, SQUARE_VOLUME_FACTOR, TRIANGLE_VOLUME_FACTOR,
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
    sample_rate: u32,

    period: u32,
    tone: Tone,
    volume: f64,
    effect: Effect,

    length: u32,
    phase: u32,
    output: i16,

    noise_reg: i32,
    noise_out: i32,
}

impl Oscillator {
    pub fn new(sample_rate: u32) -> Oscillator {
        assert!(sample_rate > 0);

        Oscillator {
            sample_rate: sample_rate,

            period: 0,
            tone: Tone::Triangle,
            volume: 0.0,
            effect: Effect::None,

            length: 0,
            phase: 0,
            output: 0,

            noise_reg: 0x8000,
            noise_out: 0,
        }
    }

    #[inline]
    pub fn output(&self) -> i16 {
        self.output
    }

    #[inline]
    pub fn play(&mut self, note: u8, tone: Tone, volume: f64, effect: Effect, length: u32) {
        self.period = self.note_to_period(note as f64);
        self.tone = tone;
        self.volume = volume;
        self.effect = effect;

        self.length = length;
        self.phase = self.period - 1;
    }

    #[inline]
    pub fn update(&mut self) {
        if self.period > 0 && self.volume > 0.0 {
            self.phase = (self.phase + 1) % self.period;

            let x = self.phase as f64 / self.period as f64;
            let amp = match self.tone {
                Tone::Triangle => {
                    (((x + 0.75).fract() * 4.0 - 2.0).abs() - 1.0) * TRIANGLE_VOLUME_FACTOR
                }
                Tone::Square => (if x < 0.5 { 1.0 } else { -1.0 }) * SQUARE_VOLUME_FACTOR,
                Tone::Pulse => (if x < 0.25 { 1.0 } else { -1.0 }) * PULSE_VOLUME_FACTOR,
                Tone::Noise => {
                    //if x >= 0.0 && x <= 1.0 {
                    self.noise_reg >>= 1;
                    self.noise_reg |= ((self.noise_reg ^ (self.noise_reg >> 1)) & 1) << 15;
                    self.noise_out = self.noise_reg & 1;
                    //}

                    self.noise_out as f64 * NOISE_VOLUME_FACTOR
                }
            };

            self.output = (amp * self.volume * i16::MAX as f64) as i16;
        } else {
            self.output = 0;
        }

        /*
        switch (effect_) {
        case EFFECT_SLIDE:
            effect_time_ = time_;
            effect_pitch_ = last_pitch > 0.0f ? last_pitch : pitch_;
            break;

        case EFFECT_VIBRATO:
            effect_time_ = time_;
            effect_pitch_ = NoteToPitch(note_ + 0.5f) - pitch_;
            break;

        case EFFECT_FADEOUT:
            effect_time_ = time_;
            effect_volume_ = volume_;
            break;
        }
        */

        /*
        switch (effect_) {
            case EFFECT_SLIDE:
            a = static_cast<float>(time_ - effect_time_) / one_note_time_;
            pitch = pitch_ * a + effect_pitch_ * (1.0f - a);
            oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
            break;

            case EFFECT_VIBRATO:
            pitch = pitch_ + Lfo(time_) * effect_pitch_;
            oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
            break;

            case EFFECT_FADEOUT:
            oscillator_.SetVolume(
                static_cast<float>(effect_volume_) *
                (1.0f - static_cast<float>(time_ - effect_time_) / one_note_time_));
            break;
        }
        */

        /*
        float Channel::Lfo(int32_t time) {
            float x = (time * 8.0f / AUDIO_SAMPLE_RATE + 0.25f);
            x -= static_cast<int32_t>(x);

            return Abs(x * 4.0f - 2.0f) - 1.0f;
        }
        */
    }

    #[inline]
    fn note_to_period(&self, note: f64) -> u32 {
        let freq = 440.0 * 2.0_f64.powf((note - 33.0) as f64 / 12.0);
        (self.sample_rate as f64 / freq).round() as u32
    }
}
