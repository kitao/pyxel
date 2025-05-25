use crate::settings::WAVETABLE_LENGTH;

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
}

pub type SharedTone = shared_type!(Tone);

impl Tone {
    pub fn new() -> SharedTone {
        new_shared_type!(Self {
            gain: 1.0,
            noise: Noise::Off,
            wavetable: [0; WAVETABLE_LENGTH as usize],
        })
    }
}
