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
pub type Note = i8;
pub type Tone = u8;
pub type Volume = u8;
pub type Effect = u8;
pub type Speed = u32;
