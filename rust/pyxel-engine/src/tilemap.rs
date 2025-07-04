use crate::canvas::{Canvas, ToIndex};
use crate::image::SharedImage;
use crate::tmx_parser::parse_tmx;
use crate::utils::{f32_to_u32, parse_hex_string, simplify_string};

pub type ImageTileCoord = u8;
pub type Tile = (ImageTileCoord, ImageTileCoord);

impl ToIndex for Tile {
    fn to_index(&self) -> usize {
        0
    }
}

#[derive(Clone)]
pub enum ImageSource {
    Index(u32),
    Image(SharedImage),
}

pub struct Tilemap {
    pub imgsrc: ImageSource,

    pub(crate) canvas: Canvas<Tile>,
}

pub type SharedTilemap = shared_type!(Tilemap);

impl Tilemap {
    pub fn new(width: u32, height: u32, imgsrc: ImageSource) -> SharedTilemap {
        new_shared_type!(Self {
            imgsrc,

            canvas: Canvas::new(width, height),
        })
    }

    pub fn from_tmx(filename: &str, layer_index: u32) -> SharedTilemap {
        parse_tmx(filename, layer_index)
    }

    pub const fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub const fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn data_ptr(&mut self) -> *mut Tile {
        self.canvas.data_ptr()
    }

    pub fn set(&mut self, x: i32, y: i32, data_str: &[&str]) {
        let width = simplify_string(data_str[0]).len() as u32 / 4;
        let height = data_str.len() as u32;
        let tilemap = Self::new(width, height, self.imgsrc.clone());

        {
            let mut tilemap = tilemap.lock();
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

        self.blt(
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
    }

    pub fn load(&mut self, x: i32, y: i32, filename: &str, layer_index: u32) {
        let tilemap = Self::from_tmx(filename, layer_index);
        let tilemap_width = tilemap.lock().width();
        let tilemap_height = tilemap.lock().height();

        self.blt(
            x as f32,
            y as f32,
            tilemap,
            0.0,
            0.0,
            tilemap_width as f32,
            tilemap_height as f32,
            None,
            None,
            None,
        );
    }

    pub fn clip(&mut self, x: f32, y: f32, width: f32, height: f32) {
        self.canvas.clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        self.canvas.clip0();
    }

    pub fn camera(&mut self, x: f32, y: f32) {
        self.canvas.camera(x, y);
    }

    pub fn camera0(&mut self) {
        self.canvas.camera0();
    }

    pub fn cls(&mut self, tile: Tile) {
        self.canvas.cls(tile);
    }

    pub fn pget(&mut self, x: f32, y: f32) -> Tile {
        self.canvas.pget(x, y)
    }

    pub fn pset(&mut self, x: f32, y: f32, tile: Tile) {
        self.canvas.pset(x, y, tile);
    }

    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, tile: Tile) {
        self.canvas.line(x1, y1, x2, y2, tile);
    }

    pub fn rect(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.rect(x, y, width, height, tile);
    }

    pub fn rectb(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.rectb(x, y, width, height, tile);
    }

    pub fn circ(&mut self, x: f32, y: f32, radius: f32, tile: Tile) {
        self.canvas.circ(x, y, radius, tile);
    }

    pub fn circb(&mut self, x: f32, y: f32, radius: f32, tile: Tile) {
        self.canvas.circb(x, y, radius, tile);
    }

    pub fn elli(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.elli(x, y, width, height, tile);
    }

    pub fn ellib(&mut self, x: f32, y: f32, width: f32, height: f32, tile: Tile) {
        self.canvas.ellib(x, y, width, height, tile);
    }

    pub fn tri(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, tile: Tile) {
        self.canvas.tri(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn trib(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, tile: Tile) {
        self.canvas.trib(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn fill(&mut self, x: f32, y: f32, tile: Tile) {
        self.canvas.fill(x, y, tile);
    }

    pub fn blt(
        &mut self,
        x: f32,
        y: f32,
        tilemap: SharedTilemap,
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
            self.blt_transform(
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

        if let Some(tilemap) = tilemap.try_lock() {
            self.canvas.blt(
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
        } else {
            let copy_width = f32_to_u32(width.abs());
            let copy_height = f32_to_u32(height.abs());
            let mut canvas = Canvas::new(copy_width, copy_height);

            canvas.blt(
                0.0,
                0.0,
                &self.canvas,
                tilemap_x,
                tilemap_y,
                copy_width as f32,
                copy_height as f32,
                None,
                None,
            );

            self.canvas
                .blt(x, y, &canvas, 0.0, 0.0, width, height, transparent, None);
        }
    }

    fn blt_transform(
        &mut self,
        x: f32,
        y: f32,
        tilemap: SharedTilemap,
        tilemap_x: f32,
        tilemap_y: f32,
        width: f32,
        height: f32,
        transparent: Option<Tile>,
        rotate: f32,
        scale: f32,
    ) {
        if let Some(tilemap) = tilemap.try_lock() {
            self.canvas.blt_transform(
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
        } else {
            let copy_width = f32_to_u32(width.abs());
            let copy_height = f32_to_u32(height.abs());
            let mut canvas = Canvas::new(copy_width, copy_height);

            canvas.blt(
                0.0,
                0.0,
                &self.canvas,
                tilemap_x,
                tilemap_y,
                copy_width as f32,
                copy_height as f32,
                None,
                None,
            );

            self.canvas.blt_transform(
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
        }
    }
}
