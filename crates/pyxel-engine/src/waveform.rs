use crate::channel::Gain;
use crate::settings::NUM_WAVEFORM_STEPS;

pub type Amp4 = u8;
pub type WaveformTable = [Amp4; NUM_WAVEFORM_STEPS as usize];

#[derive(Copy, Clone)]
pub enum Noise {
    None,
    Periodic,
    White,
}

impl Noise {
    pub fn from_index(index: u32) -> Self {
        match index {
            1 => Self::Periodic,
            2 => Self::White,
            _ => Self::None,
        }
    }

    pub fn to_index(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::Periodic => 1,
            Self::White => 2,
        }
    }
}

pub struct Waveform {
    pub gain: Gain,
    pub noise: Noise,
    pub table: WaveformTable,
}

pub type SharedWaveform = shared_type!(Waveform);

impl Waveform {
    pub fn new() -> SharedWaveform {
        new_shared_type!(Self {
            gain: 1.0,
            noise: Noise::None,
            table: [0; NUM_WAVEFORM_STEPS as usize],
        })
    }
}
