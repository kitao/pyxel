use crate::settings::{AUDIO_SAMPLE_BITS, DEFAULT_TONE_SAMPLE_BITS};

// Tone data types
pub type ToneSample = u32;
pub type ToneGain = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToneMode {
    Wavetable,
    ShortPeriodNoise,
    LongPeriodNoise,
}

// Tone mode conversions
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

// Tone state
pub struct Tone {
    pub mode: ToneMode,
    pub sample_bits: u32,
    pub wavetable: Vec<ToneSample>,
    pub gain: ToneGain,
    cached_wavetable: Vec<ToneSample>,
    cached_sample_bits: u32,
    waveform: Vec<f32>,
    waveform_revision: u64,
}

define_audio_type!(RcTone, Tone);

// Tone lifecycle and cached waveform
impl Tone {
    pub fn new() -> RcTone {
        new_audio_type!(Self {
            mode: ToneMode::Wavetable,
            sample_bits: DEFAULT_TONE_SAMPLE_BITS,
            wavetable: Vec::new(),
            gain: 1.0,
            cached_wavetable: Vec::new(),
            cached_sample_bits: 0,
            waveform: Vec::new(),
            waveform_revision: 0,
        })
    }

    pub(crate) fn waveform(&mut self) -> (&[f32], u64) {
        if self.wavetable != self.cached_wavetable || self.sample_bits != self.cached_sample_bits {
            self.cached_wavetable.clone_from(&self.wavetable);
            self.cached_sample_bits = self.sample_bits;
            self.waveform.clear();
            if (1..=AUDIO_SAMPLE_BITS).contains(&self.sample_bits) && !self.wavetable.is_empty() {
                let max_sample = (1u32 << self.sample_bits) - 1;
                self.waveform.reserve(self.wavetable.len());
                for &sample in &self.wavetable {
                    let raw = sample.min(max_sample);
                    self.waveform
                        .push((raw as f32 / max_sample as f32) * 2.0 - 1.0);
                }
            }
            self.waveform_revision = self.waveform_revision.wrapping_add(1);
        }
        (&self.waveform, self.waveform_revision)
    }
}
