use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::canvas::Canvas;
use crate::image::Color;

#[derive(Copy, Clone)]
struct BoundingBox {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

pub struct Glyph {
    dwidth: i32,
    bbx: BoundingBox,
    bitmap: Vec<u32>,
}

pub struct Font {
    font_bounding_box: BoundingBox,
    glyphs: HashMap<i32, Glyph>,
}

pub type SharedFont = shared_type!(Font);

impl Font {
    pub fn new(filename: &str) -> SharedFont {
        let mut font_bounding_box = BoundingBox {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        };
        let mut glyphs = HashMap::new();
        let mut code = None;
        let mut bitmap = None;
        let mut dwidth = 0;
        let mut bbx = BoundingBox {
            width: 0,
            height: 0,
            x: 0,
            y: 0,
        };
        let file = File::open(filename).unwrap();
        for line in BufReader::new(file).lines().map_while(Result::ok) {
            if line.starts_with("FONTBOUNDINGBOX") {
                let values: Vec<i32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|v| v.parse().unwrap())
                    .collect();
                font_bounding_box = BoundingBox {
                    width: values[0],
                    height: values[1],
                    x: values[2],
                    y: values[3],
                };
            } else if line.starts_with("ENCODING") {
                code = Some(
                    line.split_whitespace()
                        .nth(1)
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                );
            } else if line.starts_with("DWIDTH") {
                dwidth = line
                    .split_whitespace()
                    .nth(1)
                    .map(|v| v.parse().unwrap())
                    .unwrap();
            } else if line.starts_with("BBX") {
                let values: Vec<i32> = line
                    .split_whitespace()
                    .skip(1)
                    .map(|v| v.parse().unwrap())
                    .collect();
                bbx = BoundingBox {
                    width: values[0],
                    height: values[1],
                    x: values[2],
                    y: values[3],
                };
            } else if line.starts_with("BITMAP") {
                bitmap = Some(Vec::new());
            } else if line.starts_with("ENDCHAR") {
                if let (Some(code), Some(bitmap)) = (code, bitmap) {
                    glyphs.insert(
                        code,
                        Glyph {
                            dwidth,
                            bbx,
                            bitmap,
                        },
                    );
                }
                bitmap = None;
            } else if let Some(ref mut bitmap) = bitmap {
                let hex_string = line.trim();
                let bin_string = u32::from_str_radix(hex_string, 16).unwrap();
                bitmap.push(bin_string.reverse_bits() >> (32 - hex_string.len() * 4));
            }
        }
        new_shared_type!(Font {
            font_bounding_box,
            glyphs,
        })
    }

    pub fn text_width(&self, s: &str) -> i32 {
        s.chars()
            .map(|c| self.glyphs.get(&(c as i32)).map_or(0, |glyph| glyph.dwidth))
            .sum()
    }

    pub(crate) fn draw(
        &self,
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        text: &str,
        color: Color,
    ) {
        let mut x = x;
        for c in text.chars() {
            if let Some(glyph) = self.glyphs.get(&(c as i32)) {
                self.draw_glyph(canvas, x, y, glyph, color);
                x += glyph.dwidth;
            }
        }
    }

    fn draw_glyph(&self, canvas: &mut Canvas<Color>, x: i32, y: i32, glyph: &Glyph, color: Color) {
        let x = x + self.font_bounding_box.x + glyph.bbx.x;
        let y = y + self.font_bounding_box.y + self.font_bounding_box.height
            - glyph.bbx.y
            - glyph.bbx.height;
        for (i, &row) in glyph.bitmap.iter().enumerate() {
            let value_y = y + i as i32;
            for j in 0..glyph.bbx.width {
                let value_x = x + j;
                if canvas.clip_rect.contains(value_x, value_y) && (row >> j) & 1 == 1 {
                    canvas.write_data(value_x as usize, value_y as usize, color);
                }
            }
        }
    }
}
