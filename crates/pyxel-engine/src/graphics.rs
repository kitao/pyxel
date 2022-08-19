use array_macro::array;
use parking_lot::Mutex;

use crate::image::{Image, SharedImage};
use crate::settings::{
    CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, DEFAULT_COLORS, FONT_DATA, FONT_HEIGHT, FONT_WIDTH,
    IMAGE_SIZE, NUM_COLORS, NUM_FONT_ROWS, NUM_IMAGES, NUM_TILEMAPS, TILEMAP_SIZE,
};
use crate::tilemap::{SharedTilemap, Tilemap};
use crate::types::{Color, Rgb8};

static COLORS: Mutex<[Rgb8; NUM_COLORS as usize]> = Mutex::new(DEFAULT_COLORS);

pub struct Graphics {
    screen: SharedImage,
    cursor: SharedImage,
    font: SharedImage,
    images: [SharedImage; NUM_IMAGES as usize],
    tilemaps: [SharedTilemap; NUM_TILEMAPS as usize],
}

unsafe_singleton!(Graphics);

impl Graphics {
    pub fn init() {
        let screen = Image::new(crate::width(), crate::height());
        let cursor = Self::new_cursor_image();
        let font = Self::new_font_image();
        let images = array![_ => Image::new(IMAGE_SIZE, IMAGE_SIZE); NUM_IMAGES as usize];
        let tilemaps = array![_ => Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, images[0].clone()); NUM_TILEMAPS as usize];
        Self::set_instance(Self {
            screen,
            cursor,
            font,
            images,
            tilemaps,
        });
    }

    fn new_cursor_image() -> SharedImage {
        let image = Image::new(CURSOR_WIDTH, CURSOR_HEIGHT);
        image.lock().set(0, 0, &CURSOR_DATA);
        image
    }

    fn new_font_image() -> SharedImage {
        let width = FONT_WIDTH * NUM_FONT_ROWS;
        let height = FONT_HEIGHT * ((FONT_DATA.len() as u32 + NUM_FONT_ROWS - 1) / NUM_FONT_ROWS);
        let image = Image::new(width, height);
        {
            let mut image = image.lock();
            for (fi, data) in FONT_DATA.iter().enumerate() {
                let row = fi as u32 / NUM_FONT_ROWS;
                let col = fi as u32 % NUM_FONT_ROWS;
                let mut data = *data;
                for yi in 0..FONT_HEIGHT {
                    for xi in 0..FONT_WIDTH {
                        let x = FONT_WIDTH * col + xi;
                        let y = FONT_HEIGHT * row + yi;
                        let color = if (data & 0x800000) == 0 { 0 } else { 1 };
                        image.canvas.data[y as usize][x as usize] = color;
                        data <<= 1;
                    }
                }
            }
        }
        image
    }
}

pub fn colors() -> &'static Mutex<[Rgb8; NUM_COLORS as usize]> {
    &COLORS
}

pub fn screen() -> SharedImage {
    Graphics::instance().screen.clone()
}

pub fn cursor() -> SharedImage {
    Graphics::instance().cursor.clone()
}

pub fn font() -> SharedImage {
    Graphics::instance().font.clone()
}

pub fn image(image_no: u32) -> SharedImage {
    Graphics::instance().images[image_no as usize].clone()
}

pub fn image_no(image: SharedImage) -> Option<u32> {
    for (i, builtin_image) in Graphics::instance().images.iter().enumerate() {
        if builtin_image.data_ptr() == image.data_ptr() {
            return Some(i as u32);
        }
    }
    None
}

pub fn tilemap(image_no: u32) -> SharedTilemap {
    Graphics::instance().tilemaps[image_no as usize].clone()
}

pub fn clip(x: f64, y: f64, width: f64, height: f64) {
    crate::screen().lock().clip(x, y, width, height);
}

pub fn clip0() {
    crate::screen().lock().clip0();
}

pub fn camera(x: f64, y: f64) {
    crate::screen().lock().camera(x, y);
}

pub fn camera0() {
    crate::screen().lock().camera0();
}

pub fn pal(src_color: Color, dst_color: Color) {
    crate::screen().lock().pal(src_color, dst_color);
}

pub fn pal0() {
    crate::screen().lock().pal0();
}

pub fn cls(color: Color) {
    crate::screen().lock().cls(color);
}

pub fn pget(x: f64, y: f64) -> Color {
    crate::screen().lock().pget(x, y)
}

pub fn pset(x: f64, y: f64, color: Color) {
    crate::screen().lock().pset(x, y, color);
}

pub fn line(x1: f64, y1: f64, x2: f64, y2: f64, color: Color) {
    crate::screen().lock().line(x1, y1, x2, y2, color);
}

pub fn rect(x: f64, y: f64, width: f64, height: f64, color: Color) {
    crate::screen().lock().rect(x, y, width, height, color);
}

pub fn rectb(x: f64, y: f64, width: f64, height: f64, color: Color) {
    crate::screen().lock().rectb(x, y, width, height, color);
}

pub fn circ(x: f64, y: f64, radius: f64, color: Color) {
    crate::screen().lock().circ(x, y, radius, color);
}

pub fn circb(x: f64, y: f64, radius: f64, color: Color) {
    crate::screen().lock().circb(x, y, radius, color);
}

pub fn elli(x: f64, y: f64, width: f64, height: f64, color: Color) {
    crate::screen().lock().elli(x, y, width, height, color);
}

pub fn ellib(x: f64, y: f64, width: f64, height: f64, color: Color) {
    crate::screen().lock().ellib(x, y, width, height, color);
}

pub fn tri(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
    crate::screen().lock().tri(x1, y1, x2, y2, x3, y3, color);
}

pub fn trib(x1: f64, y1: f64, x2: f64, y2: f64, x3: f64, y3: f64, color: Color) {
    crate::screen().lock().trib(x1, y1, x2, y2, x3, y3, color);
}

pub fn fill(x: f64, y: f64, color: Color) {
    crate::screen().lock().fill(x, y, color);
}

pub fn blt(
    x: f64,
    y: f64,
    image_no: u32,
    image_x: f64,
    image_y: f64,
    width: f64,
    height: f64,
    color_key: Option<Color>,
) {
    crate::screen().lock().blt(
        x,
        y,
        Graphics::instance().images[image_no as usize].clone(),
        image_x,
        image_y,
        width,
        height,
        color_key,
    );
}

pub fn bltm(
    x: f64,
    y: f64,
    tilemap_no: u32,
    tilemap_x: f64,
    tilemap_y: f64,
    width: f64,
    height: f64,
    color_key: Option<Color>,
) {
    crate::screen().lock().bltm(
        x,
        y,
        Graphics::instance().tilemaps[tilemap_no as usize].clone(),
        tilemap_x,
        tilemap_y,
        width,
        height,
        color_key,
    );
}

pub fn text(x: f64, y: f64, string: &str, color: Color) {
    crate::screen().lock().text(x, y, string, color);
}
