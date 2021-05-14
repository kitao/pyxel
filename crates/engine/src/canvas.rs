use crate::rectarea::Rectarea;

pub trait Canvas<T: Copy + Default> {
  fn width(&self) -> u32;
  fn height(&self) -> u32;
  fn data(&self) -> &Vec<Vec<T>>;
  fn data_mut(&mut self) -> &mut Vec<Vec<T>>;
  fn self_rect(&self) -> Rectarea;
  fn clip_rect(&self) -> Rectarea;
  fn clip_rect_mut(&mut self) -> &mut Rectarea;
  fn render_color(&self, original_color: T) -> T;

  #[inline]
  fn clip_area(&mut self) -> (i32, i32, u32, u32) {
    (
      self.clip_rect().left(),
      self.clip_rect().top(),
      self.clip_rect().width(),
      self.clip_rect().height(),
    )
  }

  #[inline]
  fn set_clip_area(&mut self, left: i32, top: i32, width: u32, height: u32) {
    let rect = self
      .self_rect()
      .intersects(Rectarea::with_size(left, top, width, height));

    *self.clip_rect_mut() = rect;
  }

  #[inline]
  fn reset_clip_area(&mut self) {
    *self.clip_rect_mut() = self.self_rect();
  }

  #[inline]
  fn clear(&mut self, color: T) {
    let color = self.render_color(color);

    for i in 0..self.height() {
      for j in 0..self.width() {
        self.data_mut()[i as usize][j as usize] = color;
      }
    }
  }

  #[inline]
  fn point(&mut self, x: i32, y: i32) -> T {
    if self.self_rect().contains(x, y) {
      self.data()[y as usize][x as usize]
    } else {
      T::default()
    }
  }

  #[inline]
  fn draw_point(&mut self, x: i32, y: i32, color: T) {
    let color = self.render_color(color);

    if self.self_rect().contains(x, y) {
      self.data_mut()[y as usize][x as usize] = color;
    }
  }

  #[inline]
  fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: T) {
    //
  }

  #[inline]
  fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: T) {
    let color = self.render_color(color);
    let rect = Rectarea::with_size(x, y, width, height).intersects(self.clip_rect());

    if rect.is_empty() {
      return;
    }

    let left = rect.left();
    let top = rect.top();
    let right = rect.right();
    let bottom = rect.bottom();

    for i in top..=bottom {
      for j in left..=right {
        self.data_mut()[i as usize][j as usize] = color;
      }
    }
  }

  #[inline]
  fn draw_rect_border(&mut self, x: i32, y: i32, width: u32, height: u32, color: T) {
    let color = self.render_color(color);
    let rect = Rectarea::with_size(x, y, width, height).intersects(self.clip_rect());

    if rect.is_empty() {
      return;
    }

    let left = rect.left();
    let top = rect.top();
    let right = rect.right();
    let bottom = rect.bottom();

    for i in left..=right {
      self.data_mut()[top as usize][i as usize] = color;
      self.data_mut()[bottom as usize][i as usize] = color;
    }

    for i in top..=bottom {
      self.data_mut()[i as usize][left as usize] = color;
      self.data_mut()[i as usize][right as usize] = color;
    }
  }

  #[inline]
  fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: T) {
    //
  }

  #[inline]
  fn draw_circle_border(&mut self, x: i32, y: i32, radius: u32, color: T) {
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

  #[inline]
  fn paint(&mut self, x: i32, y: i32, color: T) {
    //
  }

  #[inline]
  fn copy(
    &mut self,
    x: i32,
    y: i32,
    src: &dyn Canvas<T>,
    u: i32,
    v: i32,
    width: u32,
    height: u32,
    color_key: Option<T>,
  ) {
    //
  }
}
