use crate::rectarea::Rectarea;

pub trait GraphicsBuffer<T: Copy + Default> {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn data(&self) -> &Vec<Vec<T>>;
  fn data_mut(&mut self) -> &mut Vec<Vec<T>>;
  fn self_rect(&self) -> Rectarea;
  fn clip_rect(&self) -> Rectarea;
  fn set_clip_rect(&mut self, rect: Rectarea);
  fn get_render_color(&self, original_color: T) -> T;

  #[inline]
  fn get_clip_area(&mut self) -> (i32, i32, i32, i32) {
    (
      self.clip_rect().left(),
      self.clip_rect().top(),
      self.clip_rect().width() as i32,
      self.clip_rect().height() as i32,
    )
  }

  #[inline]
  fn set_clip_area(&mut self, left: i32, top: i32, width: i32, height: i32) {
    let rect = self.self_rect().intersects(&Rectarea::with_size(
      left,
      top,
      (width as f64) as u32,
      (height as f64) as u32,
    ));

    self.set_clip_rect(rect);
  }

  #[inline]
  fn reset_clip_area(&mut self) {
    self.set_clip_rect(self.self_rect());
  }

  #[inline]
  fn clear_buffer(&mut self, color: T) {
    let color = self.get_render_color(color);

    for i in 0..self.width() {
      for j in 0..self.height() {
        self.data_mut()[i as usize][j as usize] = color;
      }
    }
  }

  #[inline]
  fn get_color(&mut self, x: i32, y: i32) -> T {
    if self.self_rect().contains(x, y) {
      self.data()[y as usize][x as usize]
    } else {
      T::default()
    }
  }

  #[inline]
  fn set_color(&mut self, x: i32, y: i32, color: T) {
    let color = self.get_render_color(color);

    if self.self_rect().contains(x, y) {
      self.data_mut()[y as usize][x as usize] = color;
    }
  }

  #[inline]
  fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: T) {
    //
  }

  #[inline]
  fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32, color: T) {
    //
  }

  #[inline]
  fn draw_rect_border(&mut self, x: i32, y: i32, width: i32, height: i32, color: T) {
    //
  }

  #[inline]
  fn draw_circle(&mut self, x: i32, y: i32, radius: i32, color: T) {
    //
  }

  #[inline]
  fn draw_circle_border(&mut self, x: i32, y: i32, radius: i32, color: T) {
    //
  }

  #[inline]
  fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: T) {
    //
  }

  #[inline]
  fn draw_triangle_border(
    &mut self,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    color: T,
  ) {
    //
  }

  fn copy_buffer(
    &mut self,
    x: i32,
    y: i32,
    gbuf: &dyn GraphicsBuffer<T>,
    u: i32,
    v: i32,
    width: i32,
    height: i32,
    color_key: Option<T>,
  ) {
    //
  }
}
