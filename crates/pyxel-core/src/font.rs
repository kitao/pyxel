use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use fontdue::{Font as FontdueFont, FontSettings, LineMetrics, Metrics};

use crate::canvas::Canvas;
use crate::image::Color;

const DEFAULT_FONT_SIZE: f32 = 10.0;
const FONT_ALPHA_THRESHOLD: u8 = 128;

#[derive(Copy, Clone)]
pub struct BdfBoundingBox {
    width: i32,
    height: i32,
    x: i32,
    y: i32,
}

pub struct BdfGlyph {
    dwidth: i32,
    bbx: BdfBoundingBox,
    bitmap: Vec<u32>,
}

pub enum Font {
    Bdf {
        bounding_box: BdfBoundingBox,
        glyphs: HashMap<i32, BdfGlyph>,
    },
    Fontdue {
        font: FontdueFont,
        cache: HashMap<char, (Metrics, Vec<u8>)>,
        size: f32,
    },
}

pub type SharedFont = shared_type!(Font);

impl Font {
    pub fn new(filename: &str, font_size: Option<f32>) -> Result<SharedFont, String> {
        if filename.to_lowercase().ends_with(".bdf") {
            let mut bdf_font_bounding_box = BdfBoundingBox {
                width: 0,
                height: 0,
                x: 0,
                y: 0,
            };
            let mut bdf_glyphs = HashMap::new();
            let mut code = None;
            let mut bitmap = None;
            let mut dwidth = 0;
            let mut bbx = BdfBoundingBox {
                width: 0,
                height: 0,
                x: 0,
                y: 0,
            };

            let file =
                File::open(filename).map_err(|_e| format!("Failed to open file '{filename}'"))?;
            for line in BufReader::new(file).lines().map_while(Result::ok) {
                if line.starts_with("FONTBOUNDINGBOX") {
                    let values: Vec<i32> = line
                        .split_whitespace()
                        .skip(1)
                        .map(|v| v.parse().unwrap())
                        .collect();
                    bdf_font_bounding_box = BdfBoundingBox {
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
                    bbx = BdfBoundingBox {
                        width: values[0],
                        height: values[1],
                        x: values[2],
                        y: values[3],
                    };
                } else if line.starts_with("BITMAP") {
                    bitmap = Some(Vec::new());
                } else if line.starts_with("ENDCHAR") {
                    if let (Some(code), Some(bitmap)) = (code, bitmap) {
                        bdf_glyphs.insert(
                            code,
                            BdfGlyph {
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

            Ok(new_shared_type!(Font::Bdf {
                bounding_box: bdf_font_bounding_box,
                glyphs: bdf_glyphs,
            }))
        } else {
            let mut file =
                File::open(filename).map_err(|_e| format!("Failed to open file '{filename}'"))?;
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            let font = FontdueFont::from_bytes(buffer, FontSettings::default()).unwrap();
            let size = font_size.unwrap_or(DEFAULT_FONT_SIZE);
            Ok(new_shared_type!(Font::Fontdue {
                font,
                cache: HashMap::new(),
                size,
            }))
        }
    }

    fn should_skip_char(c: char) -> bool {
        let cp = c as u32;

        // Control characters
        if c.is_control() {
            return true;
        }

        // Variation Selectors (VS + IVS)
        if (0xFE00..=0xFE0F).contains(&cp) || (0xE0100..=0xE01EF).contains(&cp) {
            return true;
        }

        // Bidi controls
        if (0x200E..=0x200F).contains(&cp)
            || (0x202A..=0x202E).contains(&cp)
            || (0x2066..=0x2069).contains(&cp)
        {
            return true;
        }

        // Zero-width specials
        matches!(cp, 0x200B | 0x200C | 0x200D | 0x2060)
    }

    pub fn text_width(&mut self, s: &str) -> i32 {
        let mut max_width = 0;
        let mut current_width = 0;

        for c in s.chars() {
            if c == '\n' {
                max_width = max_width.max(current_width);
                current_width = 0;
                continue;
            }
            if Self::should_skip_char(c) {
                continue;
            }

            let advance = match self {
                Font::Bdf { glyphs, .. } => glyphs.get(&(c as i32)).map_or(0, |g| g.dwidth),
                Font::Fontdue { font, cache, size } => {
                    let metrics = if let Some((m, _)) = cache.get(&c) {
                        *m
                    } else {
                        let (m, bitmap) = font.rasterize(c, *size);
                        cache.insert(c, (m, bitmap));
                        m
                    };
                    metrics.advance_width.ceil() as i32
                }
            };

            current_width += advance;
        }

        max_width.max(current_width)
    }

    pub(crate) fn draw(
        &mut self,
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        text: &str,
        color: Color,
    ) {
        match self {
            Font::Bdf {
                bounding_box,
                glyphs,
            } => {
                let start_x = x;
                let mut x = x;
                let mut y = y;
                for c in text.chars() {
                    if c == '\n' {
                        x = start_x;
                        y += bounding_box.height;
                        continue;
                    }
                    if Self::should_skip_char(c) {
                        continue;
                    }

                    if let Some(glyph) = glyphs.get(&(c as i32)) {
                        Self::draw_bdf_glyph(canvas, x, y, bounding_box, glyph, color);
                        x += glyph.dwidth;
                    }
                }
            }
            Font::Fontdue { font, cache, size } => {
                let start_x = x;
                let mut x = x;
                let mut y = y;
                let line_metrics = font.horizontal_line_metrics(*size).unwrap_or(LineMetrics {
                    ascent: *size,
                    descent: 0.0,
                    line_gap: 0.0,
                    new_line_size: *size,
                });
                let ascent = line_metrics.ascent.round() as i32;
                let line_height = line_metrics.new_line_size.ceil() as i32;

                for c in text.chars() {
                    if c == '\n' {
                        x = start_x;
                        y += line_height;
                        continue;
                    }
                    if Self::should_skip_char(c) {
                        continue;
                    }

                    let (metrics, bitmap) = if let Some(entry) = cache.get(&c) {
                        entry
                    } else {
                        let (metrics, bitmap) = font.rasterize(c, *size);
                        cache.insert(c, (metrics, bitmap));
                        cache.get(&c).unwrap()
                    };

                    Self::draw_fontdue_glyph(canvas, x, y, ascent, metrics, bitmap, color);
                    x += metrics.advance_width.ceil() as i32;
                }
            }
        }
    }

    fn draw_bdf_glyph(
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        bounding_box: &BdfBoundingBox,
        glyph: &BdfGlyph,
        color: Color,
    ) {
        let x = x + bounding_box.x + glyph.bbx.x;
        let y = y + bounding_box.y + bounding_box.height - glyph.bbx.y - glyph.bbx.height;

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

    fn draw_fontdue_glyph(
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        ascent: i32,
        metrics: &fontdue::Metrics,
        bitmap: &[u8],
        color: Color,
    ) {
        for (i, &alpha) in bitmap.iter().enumerate() {
            if alpha >= FONT_ALPHA_THRESHOLD {
                let gx = (i % metrics.width) as i32;
                let gy = (i / metrics.width) as i32;
                let value_x = x + metrics.xmin + gx;
                let value_y = (y + ascent) - (metrics.ymin + metrics.height as i32) + gy;

                if canvas.clip_rect.contains(value_x, value_y) {
                    canvas.write_data(value_x as usize, value_y as usize, color);
                }
            }
        }
    }
}
