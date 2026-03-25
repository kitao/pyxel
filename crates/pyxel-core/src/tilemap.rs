use std::ptr;

use crate::canvas::{Canvas, ToIndex};
use crate::image::Image;
use crate::settings::TILE_SIZE;
use crate::tmx_parser::parse_tmx;
use crate::utils::{f32_to_u32, parse_hex_string, simplify_string};

pub type ImageTileCoord = u16;
pub type Tile = (ImageTileCoord, ImageTileCoord);

impl ToIndex for Tile {
    fn to_index(&self) -> usize {
        0
    }
}

#[derive(Clone)]
pub enum ImageSource {
    Index(u32),
    Image(*mut Image),
}

impl ImageSource {
    /// # Safety
    /// The Image pointer (for `ImageSource::Image`) must be valid.
    pub(crate) unsafe fn resolve(&self) -> &Image {
        match self {
            ImageSource::Index(index) => &*crate::pyxel::images()[*index as usize],
            ImageSource::Image(image) => &**image,
        }
    }
}

pub struct Tilemap {
    pub imgsrc: ImageSource,
    pub(crate) canvas: Canvas<Tile>,
}

impl Tilemap {
    // Constructors

    pub fn new(width: u32, height: u32, imgsrc: ImageSource) -> *mut Tilemap {
        Box::into_raw(Box::new(Self {
            imgsrc,
            canvas: Canvas::new(width, height),
        }))
    }

    pub fn from_tmx(filename: &str, layer_index: u32) -> Result<*mut Tilemap, String> {
        parse_tmx(filename, layer_index)
    }

    // Public accessors

    pub const fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub const fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn data_ptr(&mut self) -> *mut Tile {
        self.canvas.data_ptr()
    }

    // Public data operations

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32 / 4;
        let height = data_str.len() as u32;
        let tilemap = Self::new(width, height, self.imgsrc.clone());
        {
            let tilemap = unsafe { &mut *tilemap };
            for y in 0..height {
                let src_data = simplify_string(data_str[y as usize]);
                for x in 0..width {
                    let index = x as usize * 4;
                    let tile = parse_hex_string(&src_data[index..index + 4]).unwrap();
                    tilemap.canvas.write_data(
                        x as usize,
                        y as usize,
                        (
                            ((tile >> 8) & 0xff) as ImageTileCoord,
                            (tile & 0xff) as ImageTileCoord,
                        ),
                    );
                }
            }
        }
        unsafe {
            self.draw_tilemap(
                x as f32,
                y as f32,
                tilemap,
                0.0,
                0.0,
                width as f32,
                height as f32,
                None,
                None,
                None,
            );
            drop(Box::from_raw(tilemap));
        }
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, layer_index: u32) -> Result<(), String> {
        let tilemap = Self::from_tmx(filename, layer_index)?;
        let w = unsafe { &*tilemap }.width();
        let h = unsafe { &*tilemap }.height();
        unsafe {
            self.draw_tilemap(
                x as f32, y as f32, tilemap, 0.0, 0.0, w as f32, h as f32, None, None, None,
            );
            drop(Box::from_raw(tilemap));
        }
        Ok(())
    }

    // Clip and offset

    pub fn set_clip_rect(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.canvas.set_clip_rect(x, y, width, height);
    }

    pub fn reset_clip_rect(&mut self) {
        self.canvas.reset_clip_rect();
    }

    pub fn set_draw_offset(&mut self, x: f32, y: f32) {
        self.canvas.set_draw_offset(x, y);
    }

    pub fn reset_draw_offset(&mut self) {
        self.canvas.reset_draw_offset();
    }

    // Drawing primitives

    pub fn clear(&mut self, tile: Tile) {
        self.canvas.clear(tile);
    }

    pub fn get_tile(&self, x: f32, y: f32) -> Tile {
        self.canvas.get_value(x, y)
    }

    pub fn set_tile(&mut self, x: f32, y: f32, tile: Tile) {
        self.canvas.set_value(x, y, tile);
    }

    pub fn draw_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, tile: Tile) {
        self.canvas.draw_line(x1, y1, x2, y2, tile);
    }

    pub fn draw_rect(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.draw_rect(x, y, width, height, tile);
    }

    pub fn draw_rect_border(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.draw_rect_border(x, y, width, height, tile);
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, tile: Tile) {
        self.canvas.draw_circle(x, y, radius, tile);
    }

    pub fn draw_circle_border(&mut self, x: f32, y: f32, radius: f32, tile: Tile) {
        self.canvas.draw_circle_border(x, y, radius, tile);
    }

    pub fn draw_ellipse(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.draw_ellipse(x, y, width, height, tile);
    }

    pub fn draw_ellipse_border(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.draw_ellipse_border(x, y, width, height, tile);
    }

    pub fn draw_triangle(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        tile: Tile,
    ) {
        self.canvas.draw_triangle(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn draw_triangle_border(
        &mut self,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        x3: f32,
        y3: f32,
        tile: Tile,
    ) {
        self.canvas
            .draw_triangle_border(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn flood_fill(&mut self, x: f32, y: f32, tile: Tile) {
        self.canvas.flood_fill(x, y, tile);
    }

    // Tilemap blit

    /// # Safety
    /// `tilemap` must be a valid, non-null pointer to a `Tilemap`.
    pub unsafe fn draw_tilemap(
        &mut self,
        x: f32,
        y: f32,
        tilemap: *mut Tilemap,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Tile>,
        rotate: Option<f32>,
        scale: Option<f32>,
    ) {
        let rotate = rotate.unwrap_or(0.0);
        let scale = scale.unwrap_or(1.0);
        if rotate != 0.0 || scale != 1.0 {
            self.draw_tilemap_with_transform(
                x,
                y,
                tilemap,
                tilemap_x,
                tilemap_y,
                width,
                height,
                transparent,
                rotate,
                scale,
            );
            return;
        }

        if self.is_self_blit(tilemap) {
            let canvas = self.copy_region(tilemap_x, tilemap_y, width, height);
            self.canvas
                .blit(x, y, &canvas, 0.0, 0.0, width, height, transparent, None);
        } else {
            let tilemap = unsafe { &*tilemap };
            self.canvas.blit(
                x,
                y,
                &tilemap.canvas,
                tilemap_x,
                tilemap_y,
                width,
                height,
                transparent,
                None,
            );
        }
    }

    // Collision detection

    pub fn collide(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        dx: f32,
        dy: f32,
        walls: &[Tile],
    ) -> (f32, f32) {
        // Resolve the larger-magnitude axis first for better corner handling
        let mut cur_x = x;
        let mut cur_y = y;
        let mut ndx = dx;
        let mut ndy = dy;
        if dx.abs() >= dy.abs() {
            ndx = self.collide_resolve_x(cur_x, cur_y, width, height, ndx, walls);
            cur_x += ndx;
            ndy = self.collide_resolve_y(cur_x, cur_y, width, height, ndy, walls);
        } else {
            ndy = self.collide_resolve_y(cur_x, cur_y, width, height, ndy, walls);
            cur_y += ndy;
            ndx = self.collide_resolve_x(cur_x, cur_y, width, height, ndx, walls);
        }
        (ndx, ndy)
    }

    // Private methods

    fn is_self_blit(&self, tilemap: *mut Tilemap) -> bool {
        ptr::eq(tilemap, ptr::from_ref(self).cast_mut())
    }

    /// Copies a region of this tilemap's canvas for safe self-blit.
    fn copy_region(&self, x: f32, y: f32, width: f32, height: f32) -> Canvas<Tile> {
        let w = f32_to_u32(width.abs());
        let h = f32_to_u32(height.abs());
        let mut canvas = Canvas::new(w, h);
        canvas.blit(0.0, 0.0, &self.canvas, x, y, w as f32, h as f32, None, None);
        canvas
    }

    fn draw_tilemap_with_transform(
        &mut self,
        x: f32,
        y: f32,
        tilemap: *mut Tilemap,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Tile>,
        rotate: f32,
        scale: f32,
    ) {
        if self.is_self_blit(tilemap) {
            let canvas = self.copy_region(tilemap_x, tilemap_y, width, height);
            self.canvas.blit_with_transform(
                x,
                y,
                &canvas,
                0.0,
                0.0,
                width,
                height,
                transparent,
                None,
                rotate,
                scale,
                false,
            );
        } else {
            let tilemap = unsafe { &*tilemap };
            self.canvas.blit_with_transform(
                x,
                y,
                &tilemap.canvas,
                tilemap_x,
                tilemap_y,
                width,
                height,
                transparent,
                None,
                rotate,
                scale,
                false,
            );
        }
    }

    fn collide_resolve_x(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        dx: f32,
        walls: &[Tile],
    ) -> f32 {
        if dx == 0.0 {
            return dx;
        }

        let tile_size = TILE_SIZE as f32;
        let ty0 = (y / tile_size).floor() as i32;
        let ty1 = ((y + height - 1.0) / tile_size).floor() as i32;

        if dx > 0.0 {
            let cur_edge = x + width - 1.0;
            let new_edge = cur_edge + dx;
            let start = (cur_edge / tile_size).floor() as i32 + 1;
            let end = (new_edge / tile_size).floor() as i32;
            for tx in start..=end {
                for ty in ty0..=ty1 {
                    if self.is_wall(tx, ty, walls) {
                        return tx as f32 * tile_size - width - x;
                    }
                }
            }
        } else {
            let new_edge = x + dx;
            let start = (x / tile_size).floor() as i32 - 1;
            let end = (new_edge / tile_size).floor() as i32;
            for tx in (end..=start).rev() {
                for ty in ty0..=ty1 {
                    if self.is_wall(tx, ty, walls) {
                        return (tx + 1) as f32 * tile_size - x;
                    }
                }
            }
        }

        dx
    }

    fn collide_resolve_y(
        &self,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        dy: f32,
        walls: &[Tile],
    ) -> f32 {
        if dy == 0.0 {
            return dy;
        }

        let tile_size = TILE_SIZE as f32;
        let tx0 = (x / tile_size).floor() as i32;
        let tx1 = ((x + width - 1.0) / tile_size).floor() as i32;

        if dy > 0.0 {
            let cur_edge = y + height - 1.0;
            let new_edge = cur_edge + dy;
            let start = (cur_edge / tile_size).floor() as i32 + 1;
            let end = (new_edge / tile_size).floor() as i32;
            for ty in start..=end {
                for tx in tx0..=tx1 {
                    if self.is_wall(tx, ty, walls) {
                        return ty as f32 * tile_size - height - y;
                    }
                }
            }
        } else {
            let new_edge = y + dy;
            let start = (y / tile_size).floor() as i32 - 1;
            let end = (new_edge / tile_size).floor() as i32;
            for ty in (end..=start).rev() {
                for tx in tx0..=tx1 {
                    if self.is_wall(tx, ty, walls) {
                        return (ty + 1) as f32 * tile_size - y;
                    }
                }
            }
        }

        dy
    }

    fn is_wall(&self, tx: i32, ty: i32, walls: &[Tile]) -> bool {
        let width = self.canvas.width() as i32;
        let height = self.canvas.height() as i32;
        if tx < 0 || ty < 0 || tx >= width || ty >= height {
            return false;
        }
        walls.contains(&self.canvas.read_data(tx as usize, ty as usize))
    }
}
