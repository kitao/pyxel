use std::mem::size_of_val;

//
// Input
//
pub type Key = u32;
pub type KeyValue = i32;

//
// Graphics
//
pub type Rgb8 = u32;
pub type Color = u8;
pub type Tile = (u8, u8);

pub trait ToIndex {
    fn to_index(&self) -> usize;
}

impl ToIndex for Color {
    fn to_index(&self) -> usize {
        *self as usize
    }
}

impl ToIndex for Tile {
    fn to_index(&self) -> usize {
        (self.1 as usize) << (size_of_val(&self.1) * 8) + self.0 as usize
    }
}

//
// Audio
//
pub type Note = i32;
pub type Speed = u32;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Volume {
    Level0 = 0,
    Level1 = 1,
    Level2 = 2,
    Level3 = 3,
    Level4 = 4,
    Level5 = 5,
    Level6 = 6,
    Level7 = 7,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Effect {
    None,
    Slide,
    Vibrato,
    FadeOut,
}
