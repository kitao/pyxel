use std::borrow::Cow;
use std::fs::File;

use gif::{DisposalMethod, Encoder, Frame, Repeat};
use indexmap::IndexMap;

use crate::rectarea::RectArea;
use crate::settings::NUM_COLORS;
use crate::types::{Color, Rgb8};
use crate::utils::add_file_extension;

const TRANSPARENT: Rgb8 = 0xffffffff;

struct Screen {
    image: Vec<Vec<Color>>,
    colors: [Rgb8; NUM_COLORS as usize],
    frame_count: u32,
}

impl Screen {
    fn width(&self) -> u32 {
        self.image[0].len() as u32
    }

    fn height(&self) -> u32 {
        self.image.len() as u32
    }

    fn to_rgb_image(&self) -> Vec<Vec<Rgb8>> {
        let width = self.width();
        let height = self.height();
        let mut rgb_image: Vec<Vec<Rgb8>> = Vec::new();
        for y in 0..height {
            let mut rgb_line: Vec<Rgb8> = Vec::new();
            for x in 0..width {
                let rgb = self.colors[self.image[y as usize][x as usize] as usize];
                rgb_line.push(rgb);
            }
            rgb_image.push(rgb_line);
        }
        rgb_image
    }
}

pub struct Screencast {
    fps: u32,
    max_screens: u32,
    screens: Vec<Screen>,
    capture_start_index: u32,
    num_captured_screens: u32,
}

impl Screencast {
    pub fn new(fps: u32, capture_sec: u32) -> Self {
        let max_screens = fps * capture_sec;
        let screens = (0..max_screens)
            .map(|_| Screen {
                image: Vec::new(),
                colors: [0; NUM_COLORS as usize],
                frame_count: 0,
            })
            .collect();
        Self {
            fps,
            max_screens,
            screens,
            capture_start_index: 0,
            num_captured_screens: 0,
        }
    }

    fn screen(&self, index: u32) -> &Screen {
        &self.screens[((self.capture_start_index + index) % self.max_screens) as usize]
    }

    fn screen_delay(&self, index: u32) -> u16 {
        let frame_count = self.screen(index).frame_count;
        let next_frame_count = self.screen(index + 1).frame_count;
        let num_elapsed_frames = if frame_count > next_frame_count {
            1
        } else {
            next_frame_count - frame_count
        };
        (100.0 / self.fps as f64 * num_elapsed_frames as f64 + 0.5) as u16
    }

    pub fn reset(&mut self) {
        self.capture_start_index = 0;
        self.num_captured_screens = 0;
    }

    pub fn capture(
        &mut self,
        image: &[Vec<Color>],
        colors: &[Rgb8; NUM_COLORS as usize],
        frame_count: u32,
    ) {
        if self.screens.is_empty() {
            return;
        }
        if self.num_captured_screens == self.max_screens {
            self.capture_start_index = (self.capture_start_index + 1) % self.max_screens;
            self.num_captured_screens -= 1;
        }
        let mut screen = &mut self.screens
            [((self.capture_start_index + self.num_captured_screens) % self.max_screens) as usize];
        screen.colors = *colors;
        screen.image = image.to_vec();
        screen.frame_count = frame_count;
        self.num_captured_screens += 1;
    }

    pub fn save(&mut self, filename: &str, scale: u32) {
        if self.num_captured_screens == 0 {
            return;
        }
        let filename = add_file_extension(filename, ".gif");
        let mut file = File::create(&filename)
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
        let screen = self.screen(0);
        let width = screen.width();
        let height = screen.height();
        let mut encoder = Encoder::new(
            &mut file,
            (width * scale) as u16,
            (height * scale) as u16,
            &[],
        )
        .unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        // Write first frame
        let mut base_image = screen.to_rgb_image();
        let (rect, palette, buffer) =
            Self::make_gif_buffer(RectArea::new(0, 0, width, height), &base_image, scale);
        encoder
            .write_frame(&Frame {
                delay: self.screen_delay(0),
                dispose: DisposalMethod::Any,
                transparent: None,
                needs_user_input: false,
                top: rect.top() as u16,
                left: rect.left() as u16,
                width: rect.width() as u16,
                height: rect.height() as u16,
                interlaced: false,
                palette: Some(palette),
                buffer: Cow::Borrowed(&buffer),
            })
            .unwrap();

        // Write subsequent frames
        for i in 1..self.num_captured_screens {
            let screen = &self.screen(i);
            let image = screen.to_rgb_image();
            let (rect, image) = Self::make_diff_image(&mut base_image, &image);
            let (rect, palette, buffer) = Self::make_gif_buffer(rect, &image, scale);
            encoder
                .write_frame(&Frame {
                    delay: self.screen_delay(i),
                    dispose: DisposalMethod::Keep,
                    transparent: Some(0),
                    needs_user_input: false,
                    top: rect.top() as u16,
                    left: rect.left() as u16,
                    width: rect.width() as u16,
                    height: rect.height() as u16,
                    interlaced: false,
                    palette: Some(palette),
                    buffer: Cow::Borrowed(&buffer),
                })
                .unwrap();
        }
        self.reset();
    }

    fn make_diff_image(
        base_image: &mut Vec<Vec<Rgb8>>,
        image: &[Vec<Rgb8>],
    ) -> (RectArea, Vec<Vec<Rgb8>>) {
        let mut min_x = i16::MAX;
        let mut min_y = i16::MAX;
        let mut max_x = i16::MIN;
        let mut max_y = i16::MIN;
        let mut diff_image: Vec<Vec<Rgb8>> = Vec::new();
        for y in 0..base_image.len() {
            let mut diff_line: Vec<Rgb8> = Vec::new();
            for x in 0..base_image[y].len() {
                let rgb = image[y][x];
                if rgb == base_image[y][x] {
                    diff_line.push(TRANSPARENT);
                } else {
                    min_x = min_x.min(x as i16);
                    min_y = min_y.min(y as i16);
                    max_x = max_x.max(x as i16);
                    max_y = max_y.max(y as i16);
                    base_image[y][x] = rgb;
                    diff_line.push(rgb);
                }
            }
            diff_image.push(diff_line);
        }
        if min_x > max_x || min_y > max_y {
            return (RectArea::new(0, 0, 0, 0), Vec::new());
        }
        diff_image = diff_image[min_y as usize..=max_y as usize].to_vec();
        for diff_line in &mut diff_image {
            *diff_line = diff_line[min_x as usize..=max_x as usize].to_vec();
        }
        (
            RectArea::new(
                min_x as i32,
                min_y as i32,
                diff_image[0].len() as u32,
                diff_image.len() as u32,
            ),
            diff_image,
        )
    }

    fn make_gif_buffer(
        rect: RectArea,
        image: &[Vec<Rgb8>],
        scale: u32,
    ) -> (RectArea, Vec<u8>, Vec<u8>) {
        let mut color_table = IndexMap::<Rgb8, u8>::new();
        color_table.insert(TRANSPARENT, 0);
        let mut num_colors = 1;
        let mut buffer = Vec::new();
        for row in image {
            for rgb in row {
                if let Some(index) = color_table.get(rgb) {
                    buffer.push(*index);
                } else {
                    color_table.insert(*rgb, num_colors);
                    buffer.push(num_colors);
                    num_colors += 1;
                }
            }
        }

        let rect = if buffer.is_empty() {
            buffer = vec![0];
            RectArea::new(0, 0, 1, 1)
        } else {
            let width = image[0].len() as u32;
            let height = image.len() as u32;
            let mut scaled_buffer: Vec<u8> = Vec::new();
            for y in 0..height * scale {
                for x in 0..width * scale {
                    let index = (y / scale) * width + x / scale;
                    scaled_buffer.push(buffer[index as usize])
                }
            }
            buffer = scaled_buffer;
            RectArea::new(
                rect.left() * scale as i32,
                rect.top() * scale as i32,
                rect.width() * scale,
                rect.height() * scale,
            )
        };

        let mut palette: Vec<u8> = Vec::new();
        for (rgb, _) in color_table {
            if rgb == TRANSPARENT {
                palette.push(0);
                palette.push(0);
                palette.push(0);
            } else {
                palette.push(((rgb >> 16) & 0xff) as u8);
                palette.push(((rgb >> 8) & 0xff) as u8);
                palette.push((rgb & 0xff) as u8);
            }
        }
        (rect, palette, buffer)
    }
}
