use crate::canvas::Canvas;
use crate::palette::{Color, Palette};
use crate::rectarea::RectArea;
use crate::tilemap::Tilemap;
use crate::utility::{parse_hex_string, simplify_string};

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<Vec<Color>>,
    palette: Palette,
    self_rect: RectArea,
    clip_rect: RectArea,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Image {
        Image {
            width: width,
            height: height,
            data: vec![vec![0; width as usize]; height as usize],
            palette: Palette::new(),
            self_rect: RectArea::with_size(0, 0, width, height),
            clip_rect: RectArea::with_size(0, 0, width, height),
        }
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    pub fn palette_mut(&mut self) -> &mut Palette {
        &mut self.palette
    }

    pub fn set(&mut self, x: i32, y: i32, data: &[&str]) {
        let width = data[0].len() as u32;
        let height = data.len() as u32;

        if width <= 0 || height <= 0 {
            return;
        }

        let mut image = Image::new(width, height);

        for i in 0..height as usize {
            let data = simplify_string(data[i]);

            for j in 0..width as usize {
                if let Some(value) = parse_hex_string(&data[j..j + 1]) {
                    image.data[i as usize][j as usize] = value as u8;
                } else {
                    panic!("invalid image data");
                }
            }
        }

        self.copy(x, y, &image, 0, 0, width as i32, height as i32, None);
    }

    pub fn draw_tilemap(
        &mut self,
        x: i32,
        y: i32,
        src: &Tilemap,
        u: i32,
        v: i32,
        width: i32,
        height: i32,
        color_key: Option<Color>,
    ) {
        //
    }

    pub fn draw_text(&mut self, x: i32, y: i32, text: &str, color: Color) {
        //
    }
}

impl Canvas<Color> for Image {
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn data<'a>(&'a self) -> &'a Vec<Vec<Color>> {
        &self.data
    }

    fn data_mut<'a>(&'a mut self) -> &'a mut Vec<Vec<Color>> {
        &mut self.data
    }

    fn self_rect(&self) -> RectArea {
        self.self_rect
    }

    fn clip_rect(&self) -> RectArea {
        self.clip_rect
    }

    fn clip_rect_mut(&mut self) -> &mut RectArea {
        &mut self.clip_rect
    }

    fn render_color(&self, original_color: Color) -> Color {
        self.palette.render_color(original_color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //
}
