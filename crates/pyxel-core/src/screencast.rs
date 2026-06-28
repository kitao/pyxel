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
    // Constructors

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

    // Public methods

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

        let screen = self.screen_at(0);
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

        // Reuse scratch buffers across GIF frames.
        let mut base_rgb = vec![0u32; pixel_count];
        let mut curr_rgb = vec![0u32; pixel_count];
        let mut diff_rgb = vec![0u32; pixel_count];
        let mut color_table = IndexMap::<Rgb24, u8>::with_capacity(256);
        let mut index_buf = Vec::<u8>::with_capacity(pixel_count);
        let mut scaled_buf = Vec::<u8>::with_capacity((width * scale * height * scale) as usize);
        let mut palette = Vec::<u8>::with_capacity(256 * 3);

        // Write the first frame as a full image baseline.
        self.screen_at(0).write_rgb(&mut base_rgb);
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

        // Write subsequent frames as diffs when their palette fits.
        for i in 1..self.num_captured_screens {
            self.screen_at(i).write_rgb(&mut curr_rgb);
            let diff_rect =
                Self::compute_diff(&mut base_rgb, &curr_rgb, width, height, &mut diff_rgb);

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
                // Too many colors for a transparent diff; write a full frame.
                Self::encode_region(
                    &curr_rgb,
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
                let scaled_rect = Self::scale_rect(diff_rect, scale);

                encoder
                    .write_frame(&Frame {
                        delay: self.screen_delay(i),
                        dispose: DisposalMethod::Keep,
                        transparent: Some(0),
                        needs_user_input: false,
                        top: scaled_rect.top() as u16,
                        left: scaled_rect.left() as u16,
                        width: scaled_rect.width() as u16,
                        height: scaled_rect.height() as u16,
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

    // Helpers

    fn screen_at(&self, index: u32) -> &Screen {
        &self.screens[((self.capture_start_index + index) % self.max_screens) as usize]
    }

    fn screen_delay(&self, index: u32) -> u16 {
        // Last frame has no next frame to compare against
        if index + 1 >= self.num_captured_screens {
            return (100.0 / self.fps as f32).round() as u16;
        }

        let frame_count = self.screen_at(index).frame_count;
        let next_frame_count = self.screen_at(index + 1).frame_count;

        // The frame counter can restart (pyxel.reset); treat it as one frame
        let num_elapsed_frames = if frame_count > next_frame_count {
            1
        } else {
            next_frame_count - frame_count
        };

        (100.0 / self.fps as f32 * num_elapsed_frames as f32).round() as u16
    }

    fn compute_diff(
        base: &mut [Rgb24],
        next: &[Rgb24],
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
                let rgb = next[i];
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

    // Returns true if color overflow occurred (> 256 entries needed). The many
    // arguments thread caller-owned scratch buffers in to avoid per-frame allocation.
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

        let rect_w = rect.width() as usize;
        let rect_h = rect.height() as usize;

        // Empty diff region still needs a minimal 1x1 GIF frame.
        if rect_w == 0 || rect_h == 0 {
            let scale_usize = scale as usize;
            scaled_buf.resize(scale_usize * scale_usize, 0);
            palette.extend_from_slice(&[0, 0, 0]);
            return false;
        }

        // Build the indexed-color buffer from the requested rectangle.
        let src_stride = src_width as usize;
        let rx = rect.left() as usize;
        let ry = rect.top() as usize;

        for y in 0..rect_h {
            let row = (ry + y) * src_stride + rx;
            for x in 0..rect_w {
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

        // Scale the indexed-color buffer to the requested GIF size.
        if scale == 1 {
            std::mem::swap(index_buf, scaled_buf);
        } else {
            let scale_usize = scale as usize;
            let scaled_w = rect_w * scale_usize;
            let scaled_h = rect_h * scale_usize;
            scaled_buf.reserve(scaled_w * scaled_h);
            for y in 0..scaled_h {
                let src_row = (y / scale_usize) * rect_w;
                for x in 0..scaled_w {
                    scaled_buf.push(index_buf[src_row + x / scale_usize]);
                }
            }
        }

        // Build the per-frame GIF palette.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn capture_solid(screencast: &mut Screencast, rgb: Rgb24, frame_count: u32) {
        screencast.capture(2, 2, &[0, 0, 0, 0], &[rgb], frame_count);
    }

    // Ring buffer

    #[test]
    fn test_capture_ring_buffer_wraparound() {
        // Capacity 2: a third capture drops the oldest screen
        let mut screencast = Screencast::new(2, 1);
        capture_solid(&mut screencast, 0x111111, 10);
        capture_solid(&mut screencast, 0x222222, 20);
        capture_solid(&mut screencast, 0x333333, 30);

        assert_eq!(screencast.num_captured_screens, 2);
        assert_eq!(screencast.capture_start_index, 1);
        assert_eq!(screencast.screen_at(0).frame_count, 20);
        assert_eq!(screencast.screen_at(1).frame_count, 30);
    }

    // Frame delays

    #[test]
    fn test_screen_delay() {
        let mut screencast = Screencast::new(30, 1);
        capture_solid(&mut screencast, 0x111111, 10);
        capture_solid(&mut screencast, 0x222222, 20);

        // 10 elapsed frames at 30 fps: 100 / 30 * 10 = 33 (1/100s units)
        assert_eq!(screencast.screen_delay(0), 33);
        // Last frame falls back to a single frame interval
        assert_eq!(screencast.screen_delay(1), 3);
    }

    #[test]
    fn test_screen_delay_frame_count_wraparound() {
        // The frame counter can restart (pyxel.reset); treat it as one frame
        let mut screencast = Screencast::new(30, 1);
        capture_solid(&mut screencast, 0x111111, 100);
        capture_solid(&mut screencast, 0x222222, 5);

        assert_eq!(screencast.screen_delay(0), 3);
    }

    // Diff computation

    #[test]
    fn test_compute_diff() {
        let mut base = vec![0x111111, 0x111111, 0x111111, 0x111111];
        let next = vec![0x111111, 0x222222, 0x111111, 0x333333];
        let mut diff = vec![0; 4];

        let rect = Screencast::compute_diff(&mut base, &next, 2, 2, &mut diff);
        assert_eq!(
            (rect.left(), rect.top(), rect.width(), rect.height()),
            (1, 0, 1, 2)
        );
        assert_eq!(diff, [TRANSPARENT, 0x222222, TRANSPARENT, 0x333333]);
        assert_eq!(base, next);
    }

    #[test]
    fn test_compute_diff_identical_frames() {
        let mut base = vec![0x111111; 4];
        let next = vec![0x111111; 4];
        let mut diff = vec![0; 4];

        let rect = Screencast::compute_diff(&mut base, &next, 2, 2, &mut diff);
        assert!(rect.is_empty());
        assert_eq!(diff, [TRANSPARENT; 4]);
    }

    // Color overflow

    #[test]
    fn test_encode_region_color_overflow() {
        // 16x16 region with 256 unique colors: exactly fits without the
        // transparent slot, overflows with it (257 entries needed)
        let src: Vec<Rgb24> = (0..256).collect();
        let rect = RectArea::new(0, 0, 16, 16);
        let mut color_table = IndexMap::new();
        let mut index_buf = Vec::new();
        let mut scaled_buf = Vec::new();
        let mut palette = Vec::new();

        let overflow = Screencast::encode_region(
            &src,
            16,
            rect,
            1,
            false,
            &mut color_table,
            &mut index_buf,
            &mut scaled_buf,
            &mut palette,
        );
        assert!(!overflow);
        assert_eq!(palette.len(), 256 * 3);

        let overflow = Screencast::encode_region(
            &src,
            16,
            rect,
            1,
            true,
            &mut color_table,
            &mut index_buf,
            &mut scaled_buf,
            &mut palette,
        );
        assert!(overflow);
    }

    // GIF save

    #[test]
    fn test_save_gif_with_wraparound_and_overflow() {
        // Two 256-color screens with disjoint palettes: the diff frame needs
        // all 256 new colors plus transparent, forcing the full-frame fallback
        let image: Vec<Color> = (0..=255).collect();
        let colors1: Vec<Rgb24> = (0..256).collect();
        let colors2: Vec<Rgb24> = (256..512).collect();

        let mut screencast = Screencast::new(2, 1);
        screencast.capture(16, 16, &image, &colors1, 10);
        screencast.capture(16, 16, &image, &colors2, 11);
        // Capacity wraps here, dropping the first captured screen.
        screencast.capture(16, 16, &image, &colors1, 12);

        let path =
            std::env::temp_dir().join(format!("pyxel_screencast_test_{}.gif", std::process::id()));
        let path_str = path.to_str().unwrap();
        screencast.save(path_str, 1).unwrap();
        assert_eq!(
            screencast.num_captured_screens, 0,
            "save resets capture state"
        );

        let mut options = gif::DecodeOptions::new();
        options.set_color_output(gif::ColorOutput::Indexed);
        let mut decoder = options.read_info(File::open(&path).unwrap()).unwrap();
        let first = decoder.read_next_frame().unwrap().unwrap().clone();
        let second = decoder.read_next_frame().unwrap().unwrap().clone();
        assert!(decoder.read_next_frame().unwrap().is_none());

        // Both frames are full-size: the first by definition, the second via
        // the color-overflow fallback (a diff frame would be transparent-keyed)
        assert_eq!((first.width, first.height), (16, 16));
        assert_eq!((second.width, second.height), (16, 16));
        assert_eq!(second.transparent, None);

        std::fs::remove_file(&path).ok();
    }
}
