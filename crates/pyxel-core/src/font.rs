use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use fontdue::{Font as FontdueFont, FontSettings, LineMetrics, Metrics};

use crate::canvas::Canvas;
use crate::image::Color;

const DEFAULT_FONT_SIZE: f32 = 10.0;
const FONT_ALPHA_THRESHOLD: u8 = 128;

#[derive(Clone, Copy, Default)]
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

impl Font {
    // Constructors

    pub fn new(filename: &str, font_size: Option<f32>) -> Result<*mut Font, String> {
        let font = if filename.to_lowercase().ends_with(".bdf") {
            Self::parse_bdf(filename)?
        } else {
            Self::parse_fontdue(filename, font_size)?
        };
        Ok(Box::into_raw(Box::new(font)))
    }

    fn parse_bdf(filename: &str) -> Result<Font, String> {
        let parse_err = || format!("Failed to parse file '{filename}'");
        let file = File::open(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;

        let mut bounding_box = BdfBoundingBox::default();
        let mut glyphs = HashMap::new();
        let mut code = None;
        let mut bitmap: Option<Vec<u32>> = None;
        let mut dwidth = 0;
        let mut bbx = BdfBoundingBox::default();

        for line in BufReader::new(file).lines().map_while(Result::ok) {
            if line.starts_with("FONTBOUNDINGBOX") {
                bounding_box = Self::parse_bdf_bbox(&line, &parse_err)?;
            } else if line.starts_with("ENCODING") {
                code = Some(
                    line.split_whitespace()
                        .nth(1)
                        .ok_or_else(&parse_err)?
                        .parse::<i32>()
                        .map_err(|_| parse_err())?,
                );
            } else if line.starts_with("DWIDTH") {
                dwidth = line
                    .split_whitespace()
                    .nth(1)
                    .ok_or_else(&parse_err)?
                    .parse()
                    .map_err(|_| parse_err())?;
            } else if line.starts_with("BBX") {
                bbx = Self::parse_bdf_bbox(&line, &parse_err)?;
            } else if line.starts_with("BITMAP") {
                bitmap = Some(Vec::new());
            } else if line.starts_with("ENDCHAR") {
                if let (Some(code), Some(bitmap)) = (code, bitmap) {
                    glyphs.insert(
                        code,
                        BdfGlyph {
                            dwidth,
                            bbx,
                            bitmap,
                        },
                    );
                }
                bitmap = None;
            } else if let Some(ref mut rows) = bitmap {
                let hex = line.trim();
                let bits = u32::from_str_radix(hex, 16).map_err(|_| parse_err())?;
                rows.push(bits.reverse_bits() >> (32 - hex.len() * 4));
            }
        }

        Ok(Font::Bdf {
            bounding_box,
            glyphs,
        })
    }

    fn parse_bdf_bbox(
        line: &str,
        parse_err: &dyn Fn() -> String,
    ) -> Result<BdfBoundingBox, String> {
        let values: Vec<i32> = line
            .split_whitespace()
            .skip(1)
            .map(|v| v.parse().map_err(|_| parse_err()))
            .collect::<Result<_, _>>()?;
        if values.len() < 4 {
            return Err(parse_err());
        }
        Ok(BdfBoundingBox {
            width: values[0],
            height: values[1],
            x: values[2],
            y: values[3],
        })
    }

    fn parse_fontdue(filename: &str, font_size: Option<f32>) -> Result<Font, String> {
        let mut file =
            File::open(filename).map_err(|_| format!("Failed to open file '{filename}'"))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|_| format!("Failed to read file '{filename}'"))?;
        let font = FontdueFont::from_bytes(buffer, FontSettings::default())
            .map_err(|_| format!("Failed to parse file '{filename}'"))?;
        let size = font_size.unwrap_or(DEFAULT_FONT_SIZE);
        Ok(Font::Fontdue {
            font,
            cache: HashMap::new(),
            size,
        })
    }

    // Public methods

    pub fn text_width(&mut self, s: &str) -> i32 {
        let mut max_width = 0;
        let mut width = 0;
        for c in s.chars() {
            if c == '\n' {
                max_width = max_width.max(width);
                width = 0;
                continue;
            }
            if Self::is_invisible(c) {
                continue;
            }
            width += self.char_advance(c);
        }
        max_width.max(width)
    }

    pub(crate) fn draw(
        &mut self,
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        text: &str,
        color: Color,
    ) {
        let (line_height, ascent) = self.line_params();
        let start_x = x;
        let mut x = x;
        let mut y = y;
        for c in text.chars() {
            if c == '\n' {
                x = start_x;
                y += line_height;
                continue;
            }
            if Self::is_invisible(c) {
                continue;
            }
            x += self.draw_glyph(canvas, c, x, y, ascent, color);
        }
    }

    fn line_params(&self) -> (i32, i32) {
        match self {
            Font::Bdf { bounding_box, .. } => (bounding_box.height, 0),
            Font::Fontdue { font, size, .. } => {
                let metrics = font.horizontal_line_metrics(*size).unwrap_or(LineMetrics {
                    ascent: *size,
                    descent: 0.0,
                    line_gap: 0.0,
                    new_line_size: *size,
                });
                (
                    metrics.new_line_size.ceil() as i32,
                    metrics.ascent.round() as i32,
                )
            }
        }
    }

    fn draw_glyph(
        &mut self,
        canvas: &mut Canvas<Color>,
        c: char,
        x: i32,
        y: i32,
        ascent: i32,
        color: Color,
    ) -> i32 {
        match self {
            Font::Bdf {
                bounding_box,
                glyphs,
            } => {
                if let Some(glyph) = glyphs.get(&(c as i32)) {
                    Self::draw_bdf_glyph(canvas, x, y, bounding_box, glyph, color);
                    glyph.dwidth
                } else {
                    0
                }
            }
            Font::Fontdue { font, cache, size } => {
                let (metrics, bitmap) = cache.entry(c).or_insert_with(|| font.rasterize(c, *size));
                Self::draw_fontdue_glyph(canvas, x, y, ascent, metrics, bitmap, color);
                metrics.advance_width.ceil() as i32
            }
        }
    }

    // Private methods

    fn is_invisible(c: char) -> bool {
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

    fn char_advance(&mut self, c: char) -> i32 {
        match self {
            Font::Bdf { glyphs, .. } => glyphs.get(&(c as i32)).map_or(0, |g| g.dwidth),
            Font::Fontdue { font, cache, size } => {
                let (metrics, _) = cache.entry(c).or_insert_with(|| font.rasterize(c, *size));
                metrics.advance_width.ceil() as i32
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
            let py = y + i as i32;
            for j in 0..glyph.bbx.width {
                let px = x + j;
                if canvas.clip_rect.contains(px, py) && (row >> j) & 1 == 1 {
                    canvas.write_data(px as usize, py as usize, color);
                }
            }
        }
    }

    fn draw_fontdue_glyph(
        canvas: &mut Canvas<Color>,
        x: i32,
        y: i32,
        ascent: i32,
        metrics: &Metrics,
        bitmap: &[u8],
        color: Color,
    ) {
        for (i, &alpha) in bitmap.iter().enumerate() {
            if alpha >= FONT_ALPHA_THRESHOLD {
                let px = x + metrics.xmin + (i % metrics.width) as i32;
                let py = (y + ascent) - (metrics.ymin + metrics.height as i32)
                    + (i / metrics.width) as i32;
                if canvas.clip_rect.contains(px, py) {
                    canvas.write_data(px as usize, py as usize, color);
                }
            }
        }
    }
}
