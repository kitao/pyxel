use crate::canvas::Canvas;
use crate::rectarea::Rectarea;

pub type Tile = u16;

#[derive(Debug)]
pub struct Tilemap {
    width: u32,
    height: u32,
    data: Vec<Vec<Tile>>,
    self_rect: Rectarea,
    clip_rect: Rectarea,
}

impl Tilemap {
    pub fn new(width: u32, height: u32) -> Tilemap {
        Tilemap {
            width: width,
            height: height,
            data: vec![vec![0; width as usize]; height as usize],
            self_rect: Rectarea::with_size(0, 0, width, height),
            clip_rect: Rectarea::with_size(0, 0, width, height),
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
    fn data<'a>(&'a mut self) -> &'a mut Vec<Vec<Tile>> {
        &mut self.data
    }

    #[inline]
    fn self_rect(&self) -> Rectarea {
        self.self_rect
    }

    #[inline]
    fn clip_rect(&self) -> Rectarea {
        self.clip_rect
    }

    #[inline]
    fn clip_rect_mut(&mut self) -> &mut Rectarea {
        &mut self.clip_rect
    }

    #[inline]
    fn get_render_color(&self, original_color: Tile) -> Tile {
        original_color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //
}
