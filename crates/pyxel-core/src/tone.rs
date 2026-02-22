use crate::settings::DEFAULT_TONE_SAMPLE_BITS;

pub type ToneSample = u32;
pub type ToneGain = f32;

#[derive(PartialEq, Copy, Clone)]
pub enum ToneMode {
    Wavetable,
    ShortPeriodNoise,
    LongPeriodNoise,
}

impl From<u32> for ToneMode {
    fn from(index: u32) -> Self {
        match index {
            1 => Self::ShortPeriodNoise,
            2 => Self::LongPeriodNoise,
            _ => Self::Wavetable,
        }
    }
}

impl From<ToneMode> for u32 {
    fn from(mode: ToneMode) -> Self {
        match mode {
            ToneMode::Wavetable => 0,
            ToneMode::ShortPeriodNoise => 1,
            ToneMode::LongPeriodNoise => 2,
        }
    }
}

pub struct Tone {
    pub mode: ToneMode,
    pub sample_bits: u32,
    pub wavetable: Vec<ToneSample>,
    pub gain: ToneGain,

    cached_wavetable: Vec<ToneSample>,
    waveform: Vec<f32>,
}

pub type SharedTone = shared_type!(Tone);

impl Tone {
    pub fn new() -> SharedTone {
        new_shared_type!(Self {
            mode: ToneMode::Wavetable,
            sample_bits: DEFAULT_TONE_SAMPLE_BITS,
            wavetable: Vec::new(),
            cached_wavetable: Vec::new(),
            waveform: Vec::new(),
            gain: 1.0,
        })
    }

    pub(crate) fn waveform(&mut self) -> &Vec<f32> {
        if self.wavetable != self.cached_wavetable {
            assert!(self.sample_bits <= 32);

            self.cached_wavetable = self.wavetable.clone();
            self.waveform.clear();
            self.waveform.reserve(self.wavetable.len());
            let max_sample = (1 << self.sample_bits) - 1;

            for &sample in &self.wavetable {
                assert!(sample <= max_sample);
                self.waveform
                    .push((sample as f32 / max_sample as f32) * 2.0 - 1.0);
            }
        }

        &self.waveform
    }
}
