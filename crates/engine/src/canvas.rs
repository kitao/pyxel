use crate::rectarea::RectArea;

pub trait Canvas<T: Copy> {
  fn width(&self) -> usize;
  fn height(&self) -> usize;
  fn data<'a>(&'a self) -> &'a Vec<Vec<T>>;
  fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<T>>;
  fn self_rect(&self) -> RectArea;
  fn clip_rect(&self) -> RectArea;
  fn set_clip_rect(&mut self, rect: &RectArea);
  fn get_render_color(&self, original_color: T) -> T;

  #[inline]
  fn get_clipping_area(&mut self) -> (i32, i32, i32, i32) {
    (
      self.clip_rect().left(),
      self.clip_rect().top(),
      self.clip_rect().width(),
      self.clip_rect().height(),
    )
  }

  #[inline]
  fn set_clipping_area(&mut self, left: i32, top: i32, width: i32, height: i32) {
    let rect = self
      .self_rect()
      .intersects(&RectArea::with_size(left, top, width, height));

    self.set_clip_rect(&rect);
  }

  #[inline]
  fn reset_clipping_area(&mut self) {
    self.set_clip_rect(&self.self_rect());
  }

  #[inline]
  fn clear(&mut self, color: T) {
    let color = self.get_render_color(color);

    for i in 0..self.width() {
      for j in 0..self.height() {
        self.data_mut()[i][j] = color;
      }
    }
  }

  #[inline]
  fn get_pixel(&mut self, x: i32, y: i32) -> T {
    self.data()[y as usize][x as usize]
  }

  #[inline]
  fn set_pixel(&mut self, x: i32, y: i32, color: T) {
    let color = self.get_render_color(color);

    self.data_mut()[y as usize][x as usize] = color;
  }
}

/*
proc drawLine(self: Canvas, x1, y1, x2, y2, color: int) =
proc drawRectangle(self: Canvas, x, y, width, height, color: int) =
proc drawRectangleBorder(self: Canvas, x, y, width, height, color: int) =
proc drawCircle(self: Canvas, x, y, radius, color: int) =
proc drawCircleBorder(self: Canvas, x, y, r, color: int) =
proc drawTriangle(self: Canvas, x1, y1, x2, y2, x3, y3, color: int) =
proc drawTriangleBorder(self: Canvas, x1, y1, x2, y2, x3, y3, color: int) =
proc drawCanvas(self: Canvas, x, y: int, gbuf: Canvas,
                        u, v, width, height, colorKey: int = -1) =
#proc bltm(self: Canvas, x, y: int, img: Canvas, u, v, w, h, colkey: int) =
#proc text(self: Canvas, x, y: int, s: string, col: int) =
*/
