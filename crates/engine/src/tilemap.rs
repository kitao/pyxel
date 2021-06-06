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
    #[inline]
    fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    fn data<'a>(&'a self) -> &'a Vec<Vec<Tile>> {
        &self.data
    }

    #[inline]
    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Tile>> {
        &mut self.data
    }

    #[inline]
    fn self_rect(&self) -> RectArea {
        self.self_rect
    }

    #[inline]
    fn clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    #[inline]
    fn clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }

    #[inline]
    fn render_color(&self, original_color: Tile) -> Tile {
        original_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //
}
