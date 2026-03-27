use crate::settings::DEFAULT_TONE_SAMPLE_BITS;

pub type ToneSample = u32;
pub type ToneGain = f32;

#[derive(Debug, PartialEq, Copy, Clone)]
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

impl Tone {
    pub fn new() -> *mut Tone {
        Box::into_raw(Box::new(Self {
            mode: ToneMode::Wavetable,
            sample_bits: DEFAULT_TONE_SAMPLE_BITS,
            wavetable: Vec::new(),
            gain: 1.0,
            cached_wavetable: Vec::new(),
            waveform: Vec::new(),
        }))
    }

    pub(crate) fn waveform(&mut self) -> &[f32] {
        if self.wavetable != self.cached_wavetable {
            assert!(self.sample_bits <= 32);

            let max_sample = (1 << self.sample_bits) - 1;
            self.waveform = self
                .wavetable
                .iter()
                .map(|&sample| {
                    assert!(sample <= max_sample);
                    (sample as f32 / max_sample as f32) * 2.0 - 1.0
                })
                .collect();
            self.cached_wavetable.clone_from(&self.wavetable);
        }

        &self.waveform
    }
}
