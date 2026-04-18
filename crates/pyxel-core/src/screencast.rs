use std::borrow::Cow;
use std::fs::File;

use gif::{DisposalMethod, Encoder, Frame, Repeat};
use indexmap::IndexMap;

use crate::image::{Color, Rgb24};
use crate::rect_area::RectArea;
use crate::utils::add_file_extension;

const TRANSPARENT: Rgb24 = 0xffff_ffff;

struct Screen {
    width: u32,
    height: u32,
    image: Vec<Color>,
    colors: Vec<Rgb24>,
    frame_count: u32,
}

impl Screen {
    fn write_rgb(&self, out: &mut [Rgb24]) {
        for (i, &color) in self.image.iter().enumerate() {
            out[i] = self.colors[color as usize];
        }
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
                width: 0,
                height: 0,
                image: Vec::new(),
                colors: Vec::new(),
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

    pub fn reset(&mut self) {
        self.capture_start_index = 0;
        self.num_captured_screens = 0;
    }

    pub fn capture(
        &mut self,
        width: u32,
        height: u32,
        image: &[Color],
        colors: &[Rgb24],
        frame_count: u32,
    ) {
        if self.screens.is_empty() {
            return;
        }

        if self.num_captured_screens == self.max_screens {
            self.capture_start_index = (self.capture_start_index + 1) % self.max_screens;
            self.num_captured_screens -= 1;
        }

        let screen = &mut self.screens
            [((self.capture_start_index + self.num_captured_screens) % self.max_screens) as usize];

        screen.width = width;
        screen.height = height;

        if screen.image.len() != image.len() {
            screen.image.resize(image.len(), 0);
        }
        screen.image.copy_from_slice(image);
        if screen.colors.len() != colors.len() {
            screen.colors.resize(colors.len(), 0);
        }
        screen.colors.copy_from_slice(colors);

        screen.frame_count = frame_count;

        self.num_captured_screens += 1;
    }

    pub fn save(&mut self, filename: &str, scale: u32) -> Result<(), String> {
        if self.num_captured_screens == 0 {
            return Ok(());
        }

        let filename = add_file_extension(filename, ".gif");
        let save_err = || format!("Failed to save file '{filename}'");
        let mut file =
            File::create(&filename).map_err(|_| format!("Failed to create file '{filename}'"))?;

        let screen = self.screen(0);
        let width = screen.width;
        let height = screen.height;
        let pixel_count = (width * height) as usize;

        let mut encoder = Encoder::new(
            &mut file,
            (width * scale) as u16,
            (height * scale) as u16,
            &[],
        )
        .map_err(|_| save_err())?;

        encoder
            .set_repeat(Repeat::Infinite)
            .map_err(|_| save_err())?;

        // Preallocate buffers
        let mut base_rgb = vec![0u32; pixel_count];
        let mut new_rgb = vec![0u32; pixel_count];
        let mut diff_rgb = vec![0u32; pixel_count];
        let mut color_table = IndexMap::<Rgb24, u8>::with_capacity(256);
        let mut index_buf = Vec::<u8>::with_capacity(pixel_count);
        let mut scaled_buf = Vec::<u8>::with_capacity((width * scale * height * scale) as usize);
        let mut palette = Vec::<u8>::with_capacity(256 * 3);

        // Write first frame
        self.screen(0).write_rgb(&mut base_rgb);
        let full_rect = RectArea::new(0, 0, width, height);
        Self::encode_region(
            &base_rgb,
            width,
            full_rect,
            scale,
            false,
            &mut color_table,
            &mut index_buf,
            &mut scaled_buf,
            &mut palette,
        );

        encoder
            .write_frame(&Frame {
                delay: self.screen_delay(0),
                dispose: DisposalMethod::Any,
                transparent: None,
                needs_user_input: false,
                top: 0,
                left: 0,
                width: (width * scale) as u16,
                height: (height * scale) as u16,
                interlaced: false,
                palette: Some(palette.clone()),
                buffer: Cow::Borrowed(&scaled_buf),
            })
            .map_err(|_| save_err())?;

        // Write subsequent frames
        for i in 1..self.num_captured_screens {
            self.screen(i).write_rgb(&mut new_rgb);
            let diff_rect =
                Self::compute_diff(&mut base_rgb, &new_rgb, width, height, &mut diff_rgb);

            let overflow = Self::encode_region(
                &diff_rgb,
                width,
                diff_rect,
                scale,
                true,
                &mut color_table,
                &mut index_buf,
                &mut scaled_buf,
                &mut palette,
            );

            if overflow {
                // Too many colors for diff; write as full frame
                Self::encode_region(
                    &new_rgb,
                    width,
                    full_rect,
                    scale,
                    false,
                    &mut color_table,
                    &mut index_buf,
                    &mut scaled_buf,
                    &mut palette,
                );

                encoder
                    .write_frame(&Frame {
                        delay: self.screen_delay(i),
                        dispose: DisposalMethod::Any,
                        transparent: None,
                        needs_user_input: false,
                        top: 0,
                        left: 0,
                        width: (width * scale) as u16,
                        height: (height * scale) as u16,
                        interlaced: false,
                        palette: Some(palette.clone()),
                        buffer: Cow::Borrowed(&scaled_buf),
                    })
                    .map_err(|_| save_err())?;
            } else {
                let sr = Self::scale_rect(diff_rect, scale);

                encoder
                    .write_frame(&Frame {
                        delay: self.screen_delay(i),
                        dispose: DisposalMethod::Keep,
                        transparent: Some(0),
                        needs_user_input: false,
                        top: sr.top() as u16,
                        left: sr.left() as u16,
                        width: sr.width() as u16,
                        height: sr.height() as u16,
                        interlaced: false,
                        palette: Some(palette.clone()),
                        buffer: Cow::Borrowed(&scaled_buf),
                    })
                    .map_err(|_| save_err())?;
            }
        }

        self.reset();
        Ok(())
    }

    fn screen(&self, index: u32) -> &Screen {
        &self.screens[((self.capture_start_index + index) % self.max_screens) as usize]
    }

    fn screen_delay(&self, index: u32) -> u16 {
        // Last frame has no next frame to compare against
        if index + 1 >= self.num_captured_screens {
            return (100.0 / self.fps as f32).round() as u16;
        }

        let frame_count = self.screen(index).frame_count;
        let next_frame_count = self.screen(index + 1).frame_count;

        // Ring buffer wraparound: next screen was captured before current
        let num_elapsed_frames = if frame_count > next_frame_count {
            1
        } else {
            next_frame_count - frame_count
        };

        (100.0 / self.fps as f32 * num_elapsed_frames as f32).round() as u16
    }

    fn compute_diff(
        base: &mut [Rgb24],
        new: &[Rgb24],
        width: u32,
        height: u32,
        diff: &mut [Rgb24],
    ) -> RectArea {
        let w = width as usize;
        let h = height as usize;
        let mut min_x = w;
        let mut min_y = h;
        let mut max_x = 0;
        let mut max_y = 0;

        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                let rgb = new[i];
                if rgb == base[i] {
                    diff[i] = TRANSPARENT;
                } else {
                    min_x = min_x.min(x);
                    min_y = min_y.min(y);
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    base[i] = rgb;
                    diff[i] = rgb;
                }
            }
        }

        if min_x > max_x {
            RectArea::new(0, 0, 0, 0)
        } else {
            RectArea::new(
                min_x as i32,
                min_y as i32,
                (max_x - min_x + 1) as u32,
                (max_y - min_y + 1) as u32,
            )
        }
    }

    /// Encodes a region of a flat RGB image into a GIF index buffer and palette.
    /// Returns true if color overflow occurred (> 256 entries needed).
    #[allow(clippy::too_many_arguments)]
    fn encode_region(
        src: &[Rgb24],
        src_width: u32,
        rect: RectArea,
        scale: u32,
        use_transparent: bool,
        color_table: &mut IndexMap<Rgb24, u8>,
        index_buf: &mut Vec<u8>,
        scaled_buf: &mut Vec<u8>,
        palette: &mut Vec<u8>,
    ) -> bool {
        color_table.clear();
        index_buf.clear();
        scaled_buf.clear();
        palette.clear();

        let mut next_index: u16 = 0;
        if use_transparent {
            color_table.insert(TRANSPARENT, 0);
            next_index = 1;
        }

        let rw = rect.width() as usize;
        let rh = rect.height() as usize;

        // Empty region: emit a minimal 1x1 frame
        if rw == 0 || rh == 0 {
            let s = scale as usize;
            scaled_buf.resize(s * s, 0);
            palette.extend_from_slice(&[0, 0, 0]);
            return false;
        }

        // Build index buffer from the rect region
        let src_w = src_width as usize;
        let rx = rect.left() as usize;
        let ry = rect.top() as usize;

        for y in 0..rh {
            let row = (ry + y) * src_w + rx;
            for x in 0..rw {
                let rgb = src[row + x];
                if let Some(&index) = color_table.get(&rgb) {
                    index_buf.push(index);
                } else {
                    if next_index >= 256 {
                        return true;
                    }
                    let index = next_index as u8;
                    color_table.insert(rgb, index);
                    index_buf.push(index);
                    next_index += 1;
                }
            }
        }

        // Scale
        if scale == 1 {
            std::mem::swap(index_buf, scaled_buf);
        } else {
            let sw = rw * scale as usize;
            let sh = rh * scale as usize;
            let s = scale as usize;
            scaled_buf.reserve(sw * sh);
            for y in 0..sh {
                let src_row = (y / s) * rw;
                for x in 0..sw {
                    scaled_buf.push(index_buf[src_row + x / s]);
                }
            }
        }

        // Build palette
        for &rgb in color_table.keys() {
            if rgb == TRANSPARENT {
                palette.extend_from_slice(&[0, 0, 0]);
            } else {
                palette.push((rgb >> 16) as u8);
                palette.push((rgb >> 8) as u8);
                palette.push(rgb as u8);
            }
        }

        false
    }

    fn scale_rect(rect: RectArea, scale: u32) -> RectArea {
        if rect.is_empty() {
            RectArea::new(0, 0, scale, scale)
        } else {
            RectArea::new(
                rect.left() * scale as i32,
                rect.top() * scale as i32,
                rect.width() * scale,
                rect.height() * scale,
            )
        }
    }
}
