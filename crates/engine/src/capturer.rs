use chrono::Local;
use image::imageops::{self, FilterType};
use image::{Rgb, RgbImage};
use std::env;
use std::path::Path;

use crate::canvas::Canvas;
use crate::image::Image;
use crate::settings::CAPTURE_SCALE;
use crate::types::Rgb8;

struct CaptureFrame {
    frame_image: Image,
    frame_count: u32,
}

pub struct Capturer {
    capture_frame_count: u32,
    capture_frames: Vec<CaptureFrame>,
    start_frame_index: u32,
    cur_frame_index: u32,
    cur_frame_count: u32,
}

impl Capturer {
    pub fn new(width: u32, height: u32, capture_frame_count: u32) -> Capturer {
        let capture_frames = (0..capture_frame_count)
            .map(|_| CaptureFrame {
                frame_image: Image::without_arc_mutex(width, height),
                frame_count: 0,
            })
            .collect();

        Capturer {
            capture_frame_count: capture_frame_count,
            capture_frames: capture_frames,
            start_frame_index: 0,
            cur_frame_index: 0,
            cur_frame_count: 0,
        }
    }

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        if self.capture_frame_count == 0 {
            return;
        }

        self.cur_frame_index = (self.cur_frame_index + 1) % self.capture_frame_count;
        self.cur_frame_count += 1;

        self.capture_frames[self.cur_frame_index as usize]
            .frame_image
            .blt(
                0,
                0,
                screen,
                0,
                0,
                screen.width() as i32,
                screen.height() as i32,
                None,
            );
        self.capture_frames[self.cur_frame_index as usize].frame_count = frame_count;

        if self.cur_frame_count > self.capture_frame_count {
            self.start_frame_index = (self.start_frame_index + 1) % self.capture_frame_count;
            self.cur_frame_count = self.capture_frame_count;
        }
    }

    pub fn screenshot(&mut self, colors: &[Rgb8]) {
        let screen = &self.capture_frames[self.cur_frame_index as usize].frame_image;
        let width = screen.width();
        let height = screen.height();
        let mut image = RgbImage::new(width, height);

        for i in 0..height {
            for j in 0..width {
                let rgb = colors[screen._value(j as i32, i as i32) as usize];
                let r = ((rgb >> 16) & 0xff) as u8;
                let g = ((rgb >> 8) & 0xff) as u8;
                let b = (rgb & 0xff) as u8;

                image.put_pixel(j, i, Rgb([r, g, b]));
            }
        }

        let image = imageops::resize(
            &image,
            width * CAPTURE_SCALE,
            height * CAPTURE_SCALE,
            FilterType::Nearest,
        );

        image.save(Capturer::export_path() + ".png").unwrap();
    }

    pub fn reset_capture(&mut self) {
        if self.capture_frame_count == 0 {
            return;
        }

        self.start_frame_index = (self.cur_frame_index + 1) % self.capture_frame_count;
        self.cur_frame_count = 0;
    }

    pub fn screencast(&mut self, colors: &[Rgb8]) {
        if self.capture_frame_count == 0 || self.cur_frame_count == 0 {
            return;
        }

        // TODO
        /*
        std::string filename = GetBaseName() + ".gif";
        GifWriter* gif_writer =
            new GifWriter(filename, width_, height_, palette_color_);

        for (int32_t i = 0; i < frame_count_; i++) {
          int32_t index = (start_frame_ + i) % SCREEN_CAPTURE_COUNT;

          gif_writer->AddFrame(captured_images_[index],
                               captured_frames_[index] * 100.0f / fps_ + 0.5f);
        }

        gif_writer->EndFrame();
        delete gif_writer;

        // try to optimize the generated GIF file with Gifsicle
        int32_t res = system(("gifsicle -b -O3 -Okeep-empty " + filename).c_str());

        ResetScreenCapture();
        */
    }

    #[cfg(not(target_os = "windows"))]
    fn export_path() -> String {
        Path::new(&env::var("HOME").unwrap())
            .join("Desktop")
            .join(Local::now().format("pyxel-%Y%m%d-%H%M%S").to_string())
            .to_str()
            .unwrap()
            .to_string()
    }

    #[cfg(target_os = "windows")]
    fn export_path() -> String {
        Path::new(&env::var("USERPROFILE").unwrap())
            .join(RegKey::predef(HKEY_LOCAL_MACHINE)
                .open_subkey("HKEY_CURRENT_USER\\Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Shell Folders")
                .unwrap()
                .get_value("Desktop")
                .unwrap())
    }
}
