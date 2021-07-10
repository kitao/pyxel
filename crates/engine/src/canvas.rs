use crate::rectarea::RectArea;

pub trait Canvas<T: Copy + Default> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data(&self) -> &Vec<Vec<T>>;
    fn data_mut(&mut self) -> &mut Vec<Vec<T>>;
    fn self_rect(&self) -> RectArea;
    fn clip_rect(&self) -> RectArea;
    fn clip_rect_mut(&mut self) -> &mut RectArea;
    fn render_color(&self, original_color: T) -> T;

    fn clip_area(&mut self) -> (i32, i32, u32, u32) {
        (
            self.clip_rect().left(),
            self.clip_rect().top(),
            self.clip_rect().width(),
            self.clip_rect().height(),
        )
    }

    fn set_clip_area(&mut self, left: i32, top: i32, width: u32, height: u32) {
        let rect = RectArea::with_size(left, top, width, height).intersects(self.self_rect());

        *self.clip_rect_mut() = rect;
    }

    fn reset_clip_area(&mut self) {
        *self.clip_rect_mut() = self.self_rect();
    }

    fn clear(&mut self, color: T) {
        let color = self.render_color(color);

        for i in 0..self.height() {
            for j in 0..self.width() {
                self.data_mut()[i as usize][j as usize] = color;
            }
        }
    }

    fn point(&mut self, x: i32, y: i32) -> T {
        if self.self_rect().contains(x, y) {
            self.data()[y as usize][x as usize]
        } else {
            T::default()
        }
    }

    fn draw_point(&mut self, x: i32, y: i32, color: T) {
        let color = self.render_color(color);

        if self.self_rect().contains(x, y) {
            self.data_mut()[y as usize][x as usize] = color;
        }
    }

    fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: T) {
        //
        /*
        int32_t draw_color = GET_DRAW_COLOR(color);

        if (x1 == x2 && y1 == y2) {
          SetPixel(x1, y1, draw_color);
          return;
        }

        if (Abs(x1 - x2) > Abs(y1 - y2)) {
          int32_t start_x, start_y;
          int32_t end_x, end_y;

          if (x1 < x2) {
            start_x = x1;
            start_y = y1;
            end_x = x2;
            end_y = y2;
          } else {
            start_x = x2;
            start_y = y2;
            end_x = x1;
            end_y = y1;
          }

          int32_t length = end_x - start_x + 1;
          float alpha = static_cast<float>(end_y - start_y) / (end_x - start_x);

          for (int32_t i = 0; i < length; i++) {
            SetPixel(start_x + i, start_y + alpha * i + 0.5f, draw_color);
          }
        } else {
          int32_t start_x, start_y;
          int32_t end_x, end_y;

          if (y1 < y2) {
            start_x = x1;
            start_y = y1;
            end_x = x2;
            end_y = y2;
          } else {
            start_x = x2;
            start_y = y2;
            end_x = x1;
            end_y = y1;
          }

          int32_t length = end_y - start_y + 1;
          float alpha = static_cast<float>(end_x - start_x) / (end_y - start_y);

          for (int32_t i = 0; i < length; i++) {
            SetPixel(start_x + alpha * i + 0.5f, start_y + i, draw_color);
          }
        }
              */
    }

    fn draw_rect(&mut self, x: i32, y: i32, width: u32, height: u32, color: T) {
        let color = self.render_color(color);
        let rect = RectArea::with_size(x, y, width, height).intersects(self.clip_rect());

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

    fn draw_rect_border(&mut self, x: i32, y: i32, width: u32, height: u32, color: T) {
        let color = self.render_color(color);
        let rect = RectArea::with_size(x, y, width, height).intersects(self.clip_rect());

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

    fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: T) {
        let color = self.render_color(color);
        /*
        int32_t draw_color = GET_DRAW_COLOR(color);

        if (radius == 0) {
          SetPixel(x, y, draw_color);
          return;
        }

        int32_t sq_radius = radius * radius;

        for (int32_t dx = 0; dx <= radius; dx++) {
          int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

          if (dx > dy) {
            continue;
          }

          for (int32_t i = -dy; i <= dy; i++) {
            SetPixel(x - dx, y + i, draw_color);
            SetPixel(x + dx, y + i, draw_color);
            SetPixel(x + i, y - dx, draw_color);
            SetPixel(x + i, y + dx, draw_color);
          }
        }

        */

        //
    }

    fn draw_circle_border(&mut self, x: i32, y: i32, radius: u32, color: T) {
        let color = self.render_color(color);

        /*
        if (radius == 0) {
          SetPixel(x, y, draw_color);
          return;
        }

        int32_t sq_radius = radius * radius;

        for (int32_t dx = 0; dx <= radius; dx++) {
          int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

          if (dx > dy) {
            continue;
          }

          SetPixel(x - dx, y - dy, draw_color);
          SetPixel(x + dx, y - dy, draw_color);
          SetPixel(x - dx, y + dy, draw_color);
          SetPixel(x + dx, y + dy, draw_color);

          SetPixel(x - dy, y - dx, draw_color);
          SetPixel(x + dy, y - dx, draw_color);
          SetPixel(x - dy, y + dx, draw_color);
          SetPixel(x + dy, y + dx, draw_color);
        }
        */
    }

    fn draw_triangle(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: T) {
        let color = self.render_color(color);

        /*
        // rank as y3 > y2 > y1
        if (y1 > y2) {
          std::swap(y1, y2);
          std::swap(x1, x2);
        }
        if (y1 > y3) {
          std::swap(y1, y3);
          std::swap(x1, x3);
        }
        if (y2 > y3) {
          std::swap(y2, y3);
          std::swap(x2, x3);
        }
        // slide bottom-up from y1 to y3
        float alpha12 = (y2 == y1) ? 0 : static_cast<float>(x2 - x1) / (y2 - y1);
        float alpha13 = (y3 == y1) ? 0 : static_cast<float>(x3 - x1) / (y3 - y1);
        float alpha23 = (y3 == y2) ? 0 : static_cast<float>(x3 - x2) / (y3 - y2);
        int32_t x_intersection = x1 + alpha13 * (y2 - y1) + 0.5f;
        int32_t y_slider = y1;
        for (; y_slider <= y2; y_slider++) {
          int32_t x_slider, x_end;

          if (x_intersection < x2) {
            x_slider = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
            x_end = x2 + alpha12 * (y_slider - y2) + 0.5f;
          } else {
            x_slider = x2 + alpha12 * (y_slider - y2) + 0.5f;
            x_end = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
          }

          for (; x_slider <= x_end; x_slider++) {
            SetPixel(x_slider, y_slider, draw_color);
          }
        }
        for (; y_slider <= y3; y_slider++) {
          int32_t x_slider, x_end;

          if (x_intersection < x2) {
            x_slider = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
            x_end = x2 + alpha23 * (y_slider - y2) + 0.5f;
          } else {
            x_slider = x2 + alpha23 * (y_slider - y2) + 0.5f;
            x_end = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
          }

          for (; x_slider <= x_end; x_slider++) {
            SetPixel(x_slider, y_slider, draw_color);
          }
        }
        */
    }

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
        let color = self.render_color(color);

        /*
        DrawLine(x1, y1, x2, y2, color);
        DrawLine(x1, y1, x3, y3, color);
        DrawLine(x2, y2, x3, y3, color);
        */
    }

    fn paint(&mut self, x: i32, y: i32, color: T) {
        let color = self.render_color(color);

        /*
        _col = col if type(col) is int else OverlayCanvas.COLOR_MARK

        self._fill_recursively(x, y, _col, dst)

        if type(col) is not int:
            self._replace_with_tiles(dst, x, y, col)
        */
    }

    fn paint_rec(&mut self) {
        /*
        dst_col = dst[y][x]

        if dst_col == col:
            return

        for i in range(x, -1, -1):
            if dst[y][i] != dst_col:
                break

            dst[y][i] = col

            if y > 0 and dst[y - 1][i] == dst_col:
                self._fill_recursively(i, y - 1, col, dst)

            if y < 15 and dst[y + 1][i] == dst_col:
                self._fill_recursively(i, y + 1, col, dst)

        for i in range(x + 1, 16):
            if dst[y][i] != dst_col:
                return

            dst[y][i] = col

            if y > 0 and dst[y - 1][i] == dst_col:
                self._fill_recursively(i, y - 1, col, dst)

            if y < 15 and dst[y + 1][i] == dst_col:
                self._fill_recursively(i, y + 1, col, dst)
        */
    }

    fn copy(
        &mut self,
        x: i32,
        y: i32,
        src: &Self,
        u: i32,
        v: i32,
        width: u32,
        height: u32,
        color_key: Option<T>,
    ) {
        /*
        Image* image = GetImageBank(image_index, true);

        if (color_key != -1 && (color_key < 0 || color_key >= COLOR_COUNT)) {
          PYXEL_ERROR("invalid color");
        }

        Rectangle::CopyArea copy_area =
            clip_area_.GetCopyArea(x, y, image->Rectangle(), u, v, Abs(width),
                                   Abs(height), width < 0, height < 0);

        if (copy_area.IsEmpty()) {
          return;
        }

        int32_t** src_data = image->Data();
        int32_t** dst_data = screen_data_;

        int32_t sign_x, sign_y;
        int32_t offset_x, offset_y;

        if (width < 0) {
          sign_x = -1;
          offset_x = copy_area.width - 1;
        } else {
          sign_x = 1;
          offset_x = 0;
        }

        if (height < 0) {
          sign_y = -1;
          offset_y = copy_area.height - 1;
        } else {
          sign_y = 1;
          offset_y = 0;
        }

        for (int32_t i = 0; i < copy_area.height; i++) {
          int32_t* src_line = src_data[copy_area.v + sign_y * i + offset_y];
          int32_t* dst_line = dst_data[copy_area.y + i];

          for (int32_t j = 0; j < copy_area.width; j++) {
            int32_t src_color = src_line[copy_area.u + sign_x * j + offset_x];

            if (src_color != color_key) {
              dst_line[copy_area.x + j] = palette_table_[src_color];
            }
          }
        }
        */
    }
}
