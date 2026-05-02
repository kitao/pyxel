use crate::settings::DEFAULT_TONE_SAMPLE_BITS;

pub type ToneSample = u32;
pub type ToneGain = f32;

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

define_rc_type!(RcTone, Tone);

impl Tone {
    pub fn new() -> RcTone {
        new_rc_type!(Self {
            mode: ToneMode::Wavetable,
            sample_bits: DEFAULT_TONE_SAMPLE_BITS,
            wavetable: Vec::new(),
            gain: 1.0,
        })
    }
}
