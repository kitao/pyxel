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

    pub fn amplitude(&self, phase: u32, noise_reg: &mut u16) -> f64 {
        (match self.noise {
            Noise::Off => self.wavetable[phase as usize] as f64 / 7.5 - 1.0,
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
