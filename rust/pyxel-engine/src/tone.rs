use crate::oscillator::Gain;
use crate::settings::NUM_WAVEFORM_STEPS;

pub type Amp4 = u8;
pub type Waveform = [Amp4; NUM_WAVEFORM_STEPS as usize];

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
    pub waveform: Waveform,
}

pub type SharedTone = shared_type!(Tone);

impl Tone {
    pub fn new() -> SharedTone {
        new_shared_type!(Self {
            gain: 1.0,
            noise: Noise::Off,
            waveform: [0; NUM_WAVEFORM_STEPS as usize],
        })
    }

    pub fn amplitude(&self, phase: u32, noise_reg: &mut u16) -> f64 {
        (match self.noise {
            Noise::Off => self.waveform[phase as usize] as f64 / 7.5 - 1.0,
            Noise::ShortPeriod | Noise::LongPeriod => {
                if phase % 8 == 0 {
                    let bit = if self.noise == Noise::LongPeriod {
                        1
                    } else {
                        6
                    };
                    let feedback = (*noise_reg ^ (*noise_reg >> bit)) & 1;
                    *noise_reg >>= 1;
                    *noise_reg |= feedback << 14;
                }
                (*noise_reg & 1) as f64 * 2.0 - 1.0
            }
        }) * self.gain
    }
}
