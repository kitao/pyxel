//
// Input
//
pub type Key = u32;
pub type KeyValue = i32;

//
// Graphics
//
pub type Color = u8;
pub type Rgb8 = u32;
pub type Tile = (u8, u8);

//
// Audio
//
pub type Note = i32;
pub type Volume = u32;
pub type Speed = u32;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Tone {
    Triangle,
    Square,
    Pulse,
    Noise,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Effect {
    None,
    Slide,
    Vibrato,
    FadeOut,
}
