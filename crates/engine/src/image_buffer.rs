use std::cell::RefCell;
use std::rc::Rc;

use crate::color_palette::{Color, ColorPalette};
use crate::graphics_buffer::GraphicsBuffer;
use crate::rectarea::Rectarea;

#[derive(Debug)]
pub struct ImageBuffer {
  width: u32,
  height: u32,
  data: Vec<Vec<Color>>,
  palette: Rc<RefCell<ColorPalette>>,
  self_rect: Rectarea,
  clip_rect: Rectarea,
}

impl ImageBuffer {
  pub fn new(width: u32, height: u32, palette: Rc<RefCell<ColorPalette>>) -> ImageBuffer {
    ImageBuffer {
      width: width,
      height: height,
      data: vec![vec![0; width as usize]; height as usize],
      palette: palette,
      self_rect: Rectarea::with_size(0, 0, width, height),
      clip_rect: Rectarea::with_size(0, 0, width, height),
    }
  }

  /*
  bltm(self: Canvas, x, y: int, img: Canvas, u, v, w, h, colkey: int) =
  text(self: Canvas, x, y: int, s: string, col: int) =
  */
}

impl GraphicsBuffer<Color> for ImageBuffer {
  #[inline]
  fn width(&self) -> u32 {
    self.width
  }

  #[inline]
  fn height(&self) -> u32 {
    self.height
  }

  #[inline]
  fn data<'a>(&'a self) -> &'a Vec<Vec<Color>> {
    &self.data
  }

  #[inline]
  fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Color>> {
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
  fn get_render_color(&self, original_color: Color) -> Color {
    self.palette.borrow().get_render_color(original_color)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  //
}
