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
            cached_wavetable: Vec::new(),
            waveform: Vec::new(),
            gain: 1.0,
        }))
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

#[cfg(test)]
mod tests {
    use super::*;

    // ToneMode conversions

    #[test]
    fn test_tone_mode_from_u32() {
        assert_eq!(ToneMode::from(0), ToneMode::Wavetable);
        assert_eq!(ToneMode::from(1), ToneMode::ShortPeriodNoise);
        assert_eq!(ToneMode::from(2), ToneMode::LongPeriodNoise);
        assert_eq!(ToneMode::from(3), ToneMode::Wavetable); // fallback
        assert_eq!(ToneMode::from(99), ToneMode::Wavetable);
    }

    #[test]
    fn test_tone_mode_to_u32() {
        assert_eq!(u32::from(ToneMode::Wavetable), 0);
        assert_eq!(u32::from(ToneMode::ShortPeriodNoise), 1);
        assert_eq!(u32::from(ToneMode::LongPeriodNoise), 2);
    }

    #[test]
    fn test_tone_mode_roundtrip() {
        for i in 0..3 {
            assert_eq!(u32::from(ToneMode::from(i)), i);
        }
    }

    // Tone

    #[test]
    fn test_tone_new() {
        let tone = unsafe { &mut *Tone::new() };
        assert_eq!(tone.mode, ToneMode::Wavetable);
        assert_eq!(tone.sample_bits, DEFAULT_TONE_SAMPLE_BITS);
        assert!(tone.wavetable.is_empty());
        assert_eq!(tone.gain, 1.0);
    }

    #[test]
    fn test_waveform_generation() {
        let tone = unsafe { &mut *Tone::new() };
        tone.sample_bits = 1; // max_sample = 1
        tone.wavetable = vec![0, 1, 0, 1];
        let waveform = tone.waveform();
        // 0 -> (0/1)*2-1 = -1.0, 1 -> (1/1)*2-1 = 1.0
        assert_eq!(waveform, &[-1.0, 1.0, -1.0, 1.0]);
    }

    #[test]
    fn test_waveform_caching() {
        let tone = unsafe { &mut *Tone::new() };
        tone.sample_bits = 1;
        tone.wavetable = vec![0, 1];
        let w1 = tone.waveform().clone();
        let w2 = tone.waveform().clone();
        assert_eq!(w1, w2);
    }

    #[test]
    fn test_waveform_invalidation() {
        let tone = unsafe { &mut *Tone::new() };
        tone.sample_bits = 1;
        tone.wavetable = vec![0, 1];
        let w1 = tone.waveform().clone();
        tone.wavetable = vec![1, 0];
        let w2 = tone.waveform().clone();
        assert_ne!(w1, w2);
    }
}
