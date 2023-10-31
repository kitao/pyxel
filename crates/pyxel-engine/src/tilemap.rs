use std::fmt::{self, Write as _};

use crate::canvas::{Canvas, ToIndex};
use crate::image::SharedImage;
use crate::resource::ResourceItem;
use crate::settings::{RESOURCE_ARCHIVE_DIRNAME, TILEMAP_SIZE};
use crate::utils::{f64_to_u32, parse_hex_string, simplify_string};

pub type Tile = (u8, u8);

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

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageSource::Index(index) => write!(f, "{index}"),
            ImageSource::Image(_) => write!(f, "0"),
        }
    }
}

pub struct Tilemap {
    pub(crate) canvas: Canvas<Tile>,
    pub imgsrc: ImageSource,
}

pub type SharedTilemap = shared_type!(Tilemap);

impl Tilemap {
    pub fn new(width: u32, height: u32, imgsrc: ImageSource) -> SharedTilemap {
        new_shared_type!(Self {
            canvas: Canvas::new(width, height),
            imgsrc,
        })
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
                        (((tile >> 8) & 0xff) as u8, (tile & 0xff) as u8),
                    );
                }
            }
        }
        self.blt(
            x as f64,
            y as f64,
            tilemap,
            0.0,
            0.0,
            width as f64,
            height as f64,
            None,
        );
    }

    pub fn clip(&mut self, x: f64, y: f64, width: f64, height: f64) {
        self.canvas.clip(x, y, width, height);
    }

    pub fn clip0(&mut self) {
        self.canvas.clip0();
    }

    pub fn camera(&mut self, x: f64, y: f64) {
        self.canvas.camera(x, y);
    }

    pub fn camera0(&mut self) {
        self.canvas.camera0();
    }

    pub fn cls(&mut self, tile: Tile) {
        self.canvas.cls(tile);
    }

    pub fn pget(&mut self, x: f64, y: f64) -> Tile {
        self.canvas.pget(x, y)
    }

    pub fn pset(&mut self, x: f64, y: f64, tile: Tile) {
        self.canvas.pset(x, y, tile);
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, tile: Tile) {
        self.canvas.line(x1, y1, x2, y2, tile);
    }

    pub fn rect(&mut self, x: f64, y: f64, width: f64, height: f64, tile: Tile) {
        self.canvas.rect(x, y, width, height, tile);
    }

    pub fn rectb(&mut self, x: f64, y: f64, width: f64, height: f64, tile: Tile) {
        self.canvas.rectb(x, y, width, height, tile);
    }

    pub fn circ(&mut self, x: f64, y: f64, radius: f64, tile: Tile) {
        self.canvas.circ(x, y, radius, tile);
    }

    pub fn circb(&mut self, x: f64, y: f64, radius: f64, tile: Tile) {
        self.canvas.circb(x, y, radius, tile);
    }

    pub fn elli(&mut self, x: f64, y: f64, width: f64, height: f64, tile: Tile) {
        self.canvas.elli(x, y, width, height, tile);
    }

    pub fn ellib(&mut self, x: f64, y: f64, width: f64, height: f64, tile: Tile) {
        self.canvas.ellib(x, y, width, height, tile);
    }

    pub fn tri(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: Tile) {
        self.canvas.tri(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn trib(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, tile: Tile) {
        self.canvas.trib(x1, y1, x2, y2, x3, y3, tile);
    }

    pub fn fill(&mut self, x: f64, y: f64, tile: Tile) {
        self.canvas.fill(x, y, tile);
    }

    pub fn blt(
        &mut self,
        x: f64,
        y: f64,
        tilemap: SharedTilemap,
        tilemap_x: f64,
        tilemap_y: f64,
        width: f64,
        height: f64,
        transparent: Option<Tile>,
    ) {
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
            let copy_width = f64_to_u32(width.abs());
            let copy_height = f64_to_u32(height.abs());
            let mut canvas = Canvas::new(copy_width, copy_height);
            canvas.blt(
                0.0,
                0.0,
                &self.canvas,
                tilemap_x,
                tilemap_y,
                copy_width as f64,
                copy_height as f64,
                None,
                None,
            );
            self.canvas
                .blt(x, y, &canvas, 0.0, 0.0, width, height, transparent, None);
        }
    }
}

impl ResourceItem for Tilemap {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "tilemap" + &item_index.to_string()
    }

    fn is_modified(&self) -> bool {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.canvas.read_data(x as usize, y as usize) != (0, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn clear(&mut self) {
        self.cls((0, 0));
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.canvas.read_data(x as usize, y as usize);
                let _guard = write!(output, "{:02x}{:02x}", tile.0, tile.1);
            }
            output += "\n";
        }
        let _guard = write!(output, "{}", self.imgsrc);
        output
    }

    fn deserialize(&mut self, version: u32, input: &str) {
        for (y, line) in input.lines().enumerate() {
            if y < TILEMAP_SIZE as usize {
                if version < 10500 {
                    string_loop!(x, tile, line, 3, {
                        let tile = parse_hex_string(&tile).unwrap();
                        self.canvas
                            .write_data(x, y, ((tile % 32) as u8, (tile / 32) as u8));
                    });
                } else {
                    string_loop!(x, tile, line, 4, {
                        let tile_x = parse_hex_string(&tile[0..2]).unwrap();
                        let tile_y = parse_hex_string(&tile[2..4]).unwrap();
                        self.canvas.write_data(x, y, (tile_x as u8, tile_y as u8));
                    });
                }
            } else {
                self.imgsrc = ImageSource::Index(line.parse::<usize>().unwrap() as u32);
            }
        }
    }
}
