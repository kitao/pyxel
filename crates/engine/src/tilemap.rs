use crate::canvas::Canvas;
use crate::rectarea::RectArea;

pub type Tile = u16;

pub struct Tilemap {
    width: u32,
    height: u32,
    data: Vec<Vec<Tile>>,
    self_rect: RectArea,
    clip_rect: RectArea,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Tilemap {
        Tilemap {
            width: width,
            height: height,
            data: vec![vec![0; width as usize]; height as usize],
            self_rect: RectArea::with_size(0, 0, width, height),
            clip_rect: RectArea::with_size(0, 0, width, height),
        }
    }
}

impl Canvas<Tile> for Tilemap {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data<'a>(&'a self) -> &'a Vec<Vec<Tile>> {
        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Tile>> {
        &mut self.data
    }

    fn self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }

    fn render_value(&self, original_value: Tile) -> Tile {
        original_value
    }
}
