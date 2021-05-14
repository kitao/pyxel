use crate::canvas::Canvas;
use crate::palette::{Color, Palette};
use crate::rectarea::Rectarea;

#[derive(Debug)]
pub struct Image {
  width: u32,
  height: u32,
  data: Vec<Vec<Color>>,
  palette: Palette,
  self_rect: Rectarea,
  clip_rect: Rectarea,
}

impl Image {
  pub fn new(width: u32, height: u32) -> Image {
    Image {
      width: width,
      height: height,
      data: vec![vec![0; width as usize]; height as usize],
      palette: Palette::new(),
      self_rect: Rectarea::with_size(0, 0, width, height),
      clip_rect: Rectarea::with_size(0, 0, width, height),
    }
  }

  #[inline]
  pub fn palette(&self) -> &Palette {
    &self.palette
  }

  #[inline]
  pub fn palette_mut(&mut self) -> &mut Palette {
    &mut self.palette
  }

  pub fn draw_tilemap(
    &mut self,
    x: i32,
    y: i32,
    src: &dyn Canvas<Color>,
    u: i32,
    v: i32,
    width: i32,
    height: i32,
    color_key: Option<Color>,
  ) {
    //
  }

  pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
    //
  }
}

impl Canvas<Color> for Image {
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
  fn render_color(&self, original_color: Color) -> Color {
    self.palette.render_color(original_color)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  //
}
