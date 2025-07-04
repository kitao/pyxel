use crate::settings::{WAVETABLE_LENGTH, WAVETABLE_LEVELS};

pub type Gain = f64;
pub type WavetableValue = u8;
pub type Wavetable = [WavetableValue; WAVETABLE_LENGTH as usize];

#[derive(PartialEq, Copy, Clone)]
pub enum Noise {
    Off,
    ShortPeriod,
    LongPeriod,
}

impl Noise {
    pub fn from_index(index: u32) -> Self {
        match index {
            1 => Self::ShortPeriod,
            2 => Self::LongPeriod,
            _ => Self::Off,
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Self::Off => 0,
            Self::ShortPeriod => 1,
            Self::LongPeriod => 2,
        }
    }
}

pub struct Tone {
    pub gain: Gain,
    pub noise: Noise,
    pub wavetable: Wavetable,

    last_wavetable: Wavetable,
    waveform: Vec<f64>,
}

pub type SharedTone = shared_type!(Tone);

impl Tone {
    pub fn new() -> SharedTone {
        new_shared_type!(Self {
            gain: 1.0,
            noise: Noise::Off,
            wavetable: [0; WAVETABLE_LENGTH as usize],
            last_wavetable: [0; WAVETABLE_LENGTH as usize],
            waveform: Vec::new(),
        })
    }

    pub(crate) fn waveform(&mut self) -> &Vec<f64> {
        if self.wavetable != self.last_wavetable {
            self.last_wavetable = self.wavetable;

            self.waveform.clear();
            self.waveform.reserve(WAVETABLE_LENGTH as usize);
            for &value in &self.wavetable {
                assert!(value < WAVETABLE_LEVELS as u8);
                self.waveform
                    .push((value as f64 / (WAVETABLE_LEVELS - 1) as f64) * 2.0 - 1.0);
            }
        }

        &self.waveform
    }
}
