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
