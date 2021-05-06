use std::cell::RefCell;
use std::rc::Rc;

use crate::color_palette::ColorPalette;
use crate::image_buffer::ImageBuffer;
use crate::tilemap_buffer::TilemapBuffer;

const IMAGE_BUFFER_COUNT: usize = 3 + 1;
const IMAGE_BUFFER_SIZE: u32 = 256;

const TILEMAP_BUFFER_COUNT: usize = 8;
const TILEMAP_BUFFER_SIZE: u32 = 256;

pub struct Graphics {
    screen_width: u32,
    screen_height: u32,
    palette: Rc<RefCell<ColorPalette>>,
    screen: ImageBuffer,
    image_buf: Vec<ImageBuffer>,
    tilemap_buf: Vec<TilemapBuffer>,
}

impl Graphics {
    pub fn new(screen_width: u32, screen_height: u32) -> Graphics {
        let palette = Rc::new(RefCell::new(ColorPalette::new()));
        let screen = ImageBuffer::new(screen_width, screen_height, Rc::clone(&palette));
        let mut image_buf = Vec::new();
        let mut tilemap_buf = Vec::new();

        for i in 0..IMAGE_BUFFER_COUNT {
            image_buf.push(ImageBuffer::new(
                IMAGE_BUFFER_SIZE,
                IMAGE_BUFFER_SIZE,
                Rc::clone(&palette),
            ));
        }

        for i in 0..TILEMAP_BUFFER_COUNT {
            tilemap_buf.push(TilemapBuffer::new(TILEMAP_BUFFER_SIZE, TILEMAP_BUFFER_SIZE));
        }

        let graphics = Graphics {
            screen_width: screen_width,
            screen_height: screen_height,
            palette: palette,
            screen: screen,
            image_buf: image_buf,
            tilemap_buf: tilemap_buf,
        };

        graphics
    }
}

/*
Graphics(int32_t width, int32_t height);

const Rectangle& ClipArea() const { return clip_area_; }
const pyxelcore::PaletteTable& PaletteTable() const { return palette_table_; }
Image* ScreenImage() const { return image_bank_[IMAGE_BANK_FOR_SCREEN]; }

Image* GetImageBank(int32_t image_index, bool system = false) const;
Tilemap* GetTilemapBank(int32_t tilemap_index) const;

void ResetClipArea();
void SetClipArea(int32_t x, int32_t y, int32_t width, int32_t height);
void ResetPalette();
void SetPalette(int32_t src_color, int32_t dst_color);
void ClearScreen(int32_t color);
int32_t GetPoint(int32_t x, int32_t y);
void SetPoint(int32_t x, int32_t y, int32_t color);
void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t color);
void DrawRectangle(int32_t x,
                    int32_t y,
                    int32_t width,
                    int32_t height,
                    int32_t color);
void DrawRectangleBorder(int32_t x,
                        int32_t y,
                        int32_t width,
                        int32_t height,
                        int32_t color);
void DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color);
void DrawCircleBorder(int32_t x, int32_t y, int32_t radius, int32_t color);
void DrawTriangle(int32_t x1,
                int32_t y1,
                int32_t x2,
                int32_t y2,
                int32_t x3,
                int32_t y3,
                int32_t color);
void DrawTriangleBorder(int32_t x1,
                        int32_t y1,
                        int32_t x2,
                        int32_t y2,
                        int32_t x3,
                        int32_t y3,
                        int32_t color);
void DrawImage(int32_t x,
                int32_t y,
                int32_t image_index,
                int32_t u,
                int32_t v,
                int32_t width,
                int32_t height,
                int32_t color_key = -1);
void DrawTilemap(int32_t x,
                int32_t y,
                int32_t tilemap_index,
                int32_t u,
                int32_t v,
                int32_t width,
                int32_t height,
                int32_t color_key = -1);
void DrawText(int32_t x, int32_t y, const char* text, int32_t color);
*/
