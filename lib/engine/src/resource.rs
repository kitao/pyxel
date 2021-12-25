use std::borrow::Cow;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use chrono::Local;
use gif::{DisposalMethod, Encoder, Frame, Repeat};
use indexmap::IndexMap;
use platform_dirs::UserDirs;
use zip::{ZipArchive, ZipWriter};

use crate::image::{Image, SharedImage};
use crate::music::Music;
use crate::settings::{
    CAPTURE_SCALE, NUM_COLORS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, PYXEL_VERSION,
    RESOURCE_ARCHIVE_DIRNAME,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::types::Rgb8;
use crate::utils::parse_version_string;
use crate::Pyxel;

#[derive(Clone)]
struct Screen {
    image: SharedImage,
    colors: [Rgb8; NUM_COLORS as usize],
    frame_count: u32,
    delay: u16,
}

pub struct Resource {
    fps: u32,
    max_screens: u32,
    screens: Vec<Screen>,
    start_screen_index: u32,
    next_screen_index: u32,
    num_screens: u32,
}

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self, pyxel: &Pyxel) -> String;
    fn deserialize(&mut self, pyxel: &Pyxel, version: u32, input: &str);
}

impl Resource {
    pub fn new(width: u32, height: u32, fps: u32, capture_sec: u32) -> Self {
        let max_screens = fps * capture_sec;
        let screens = (0..max_screens)
            .map(|_| Screen {
                image: Image::new(width, height),
                colors: [0; NUM_COLORS as usize],
                frame_count: 0,
                delay: 0,
            })
            .collect();
        Self {
            fps,
            max_screens,
            screens,
            start_screen_index: 0,
            next_screen_index: 0,
            num_screens: 0,
        }
    }

    pub fn capture_screen(
        &mut self,
        screen_image: SharedImage,
        colors: &[Rgb8; NUM_COLORS as usize],
        frame_count: u32,
    ) {
        if self.max_screens == 0 {
            return;
        }
        let prev_frame_count = self.screens
            [((self.next_screen_index + self.max_screens - 1) % self.max_screens) as usize]
            .frame_count;
        let screen = &mut self.screens[self.next_screen_index as usize];
        let width = screen_image.lock().width();
        let height = screen_image.lock().height();
        screen.colors = *colors;
        screen.image.lock().blt(
            0.0,
            0.0,
            screen_image,
            0.0,
            0.0,
            width as f64,
            height as f64,
            None,
        );
        screen.frame_count = frame_count;
        screen.delay = ((100.0 / self.fps as f64)
            * if self.num_screens == 0 {
                1.0
            } else {
                (screen.frame_count - prev_frame_count) as f64
            }
            + 0.5) as u16;
        self.next_screen_index = (self.next_screen_index + 1) % self.max_screens;
        self.num_screens += 1;
        if self.num_screens > self.max_screens {
            self.start_screen_index = (self.start_screen_index + 1) % self.max_screens;
            self.num_screens = self.max_screens;
        }
    }

    fn export_path() -> String {
        UserDirs::new()
            .unwrap()
            .desktop_dir
            .join(Local::now().format("pyxel-%Y%m%d-%H%M%S").to_string())
            .to_str()
            .unwrap()
            .to_string()
    }

    fn scale_buffer(buffer: &[u8], width: u32, height: u32, scale: u32) -> Vec<u8> {
        let mut scaled_buffer: Vec<u8> = Vec::new();
        for y in 0..height * scale {
            for x in 0..width * scale {
                let index = (y / scale) * width + x / scale;
                scaled_buffer.push(buffer[index as usize])
            }
        }
        scaled_buffer
    }
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(
            File::open(&Path::new(filename))
                .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename)),
        )
        .unwrap_or_else(|_| panic!("Unable to parse zip archive '{}'", filename));
        let version = {
            let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let version = parse_version_string(&contents).unwrap();
            if version > parse_version_string(PYXEL_VERSION).unwrap() {
                panic!("Unsupported resource file version '{}'", contents);
            }
            version
        };

        macro_rules! deserialize {
            ($type: ty, $getter: ident, $count: expr) => {
                for i in 0..$count {
                    if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                        let mut input = String::new();
                        file.read_to_string(&mut input).unwrap();
                        self.$getter(i).lock().deserialize(self, version, &input);
                    } else {
                        self.$getter(i).lock().clear();
                    }
                }
            };
        }

        if image {
            deserialize!(Image, image, NUM_IMAGES);
        }
        if tilemap {
            deserialize!(Tilemap, tilemap, NUM_TILEMAPS);
        }
        if sound {
            deserialize!(Sound, sound, NUM_SOUNDS);
        }
        if music {
            deserialize!(Music, music, NUM_MUSICS);
        }
    }

    pub fn save(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let path = std::path::Path::new(filename);
        let file = std::fs::File::create(&path)
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
        let mut zip = ZipWriter::new(file);
        zip.add_directory(RESOURCE_ARCHIVE_DIRNAME, Default::default())
            .unwrap();
        {
            let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
            zip.start_file(version_name, Default::default()).unwrap();
            zip.write_all(PYXEL_VERSION.as_bytes()).unwrap();
        }

        macro_rules! serialize {
            ($type: ty, $getter: ident, $count: expr) => {
                for i in 0..$count {
                    if self.$getter(i).lock().is_modified() {
                        zip.start_file(<$type>::resource_name(i), Default::default())
                            .unwrap();
                        zip.write_all(self.$getter(i).lock().serialize(self).as_bytes())
                            .unwrap();
                    }
                }
            };
        }

        if image {
            serialize!(Image, image, NUM_IMAGES);
        }
        if tilemap {
            serialize!(Tilemap, tilemap, NUM_TILEMAPS);
        }
        if sound {
            serialize!(Sound, sound, NUM_SOUNDS);
        }
        if music {
            serialize!(Music, music, NUM_MUSICS);
        }
        zip.finish().unwrap();
    }

    pub fn screenshot(&mut self) {
        self.screen
            .lock()
            .save(&Resource::export_path(), &self.colors, CAPTURE_SCALE);
        self.system.disable_next_frame_skip();
    }

    pub fn reset_capture(&mut self) {
        if self.resource.max_screens == 0 {
            return;
        }
        self.resource.start_screen_index = 0;
        self.resource.next_screen_index = 0;
        self.resource.num_screens = 0;
    }

    pub fn screencast(&mut self) {
        if self.resource.max_screens == 0 || self.resource.num_screens == 0 {
            return;
        }
        let width = self.width();
        let height = self.height();
        let filename = Resource::export_path() + ".gif";
        let mut file = File::create(&filename)
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
        let mut encoder = Encoder::new(
            &mut file,
            (width * CAPTURE_SCALE) as u16,
            (height * CAPTURE_SCALE) as u16,
            &[],
        )
        .unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();
        let mut prev_rgb_image: Vec<Vec<Rgb8>> = Vec::new();
        for i in 0..self.resource.num_screens {
            let index = (self.resource.start_screen_index + i) % self.resource.max_screens;
            let screen = &self.resource.screens[index as usize];
            let colors = &screen.colors;
            let image = &screen.image.lock();

            let mut rgb_image: Vec<Vec<Rgb8>> = Vec::new();
            for y in 0..height {
                let mut rgb_line: Vec<Rgb8> = Vec::new();
                for x in 0..width {
                    rgb_line.push(colors[image.canvas.data[y as usize][x as usize] as usize]);
                }
                rgb_image.push(rgb_line);
            }

            if i == 0 {
                let mut buffer: Vec<u8> = Vec::new();
                let mut color_table = IndexMap::<Rgb8, u8>::new();
                let mut num_colors = 0;
                for y in 0..height {
                    for x in 0..width {
                        let rgb = rgb_image[y as usize][x as usize];
                        if let Some(index) = color_table.get(&rgb) {
                            buffer.push(*index);
                        } else {
                            color_table.insert(rgb, num_colors);
                            buffer.push(num_colors);
                            num_colors += 1;
                        }
                    }
                }

                let mut palette: Vec<u8> = Vec::new();
                for (rgb, _) in color_table {
                    palette.push(((rgb >> 16) & 0xff) as u8);
                    palette.push(((rgb >> 8) & 0xff) as u8);
                    palette.push((rgb & 0xff) as u8);
                }

                buffer = Resource::scale_buffer(&buffer, width, height, CAPTURE_SCALE);
                encoder
                    .write_frame(&Frame {
                        delay: screen.delay,
                        dispose: DisposalMethod::Any,
                        transparent: None,
                        needs_user_input: false,
                        top: 0,
                        left: 0,
                        width: (width * CAPTURE_SCALE) as u16,
                        height: (height * CAPTURE_SCALE) as u16,
                        interlaced: false,
                        palette: Some(palette),
                        buffer: Cow::Borrowed(&buffer),
                    })
                    .unwrap();
            } else {
                let mut min_x = width as i16;
                let mut min_y = height as i16;
                let mut max_x = -1_i16;
                let mut max_y = -1_i16;

                let mut diff_buffer: Vec<Vec<u8>> = Vec::new();
                let mut color_table = IndexMap::<Rgb8, u8>::new();
                color_table.insert(0xffffffff, 0);
                let mut num_colors = 1;
                for y in 0..height {
                    let mut diff_line: Vec<u8> = Vec::new();
                    for x in 0..width {
                        let cur_rgb = rgb_image[y as usize][x as usize];
                        let prev_rgb = prev_rgb_image[y as usize][x as usize];
                        if cur_rgb == prev_rgb {
                            diff_line.push(0);
                            continue;
                        }

                        min_x = min_x.min(x as i16);
                        min_y = min_y.min(y as i16);
                        max_x = max_x.max(x as i16);
                        max_y = max_y.max(y as i16);

                        if let Some(index) = color_table.get(&cur_rgb) {
                            diff_line.push(*index);
                        } else {
                            color_table.insert(cur_rgb, num_colors);
                            diff_line.push(num_colors);
                            num_colors += 1;
                        }
                    }
                    diff_buffer.push(diff_line);
                }

                let mut buffer: Vec<u8> = Vec::new();
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        buffer.push(diff_buffer[y as usize][x as usize]);
                    }
                }

                let mut palette: Vec<u8> = Vec::new();
                for (rgb, _) in color_table {
                    if rgb > 0xffffff {
                        palette.push(0xff);
                        palette.push(0x00);
                        palette.push(0xff);
                    } else {
                        palette.push(((rgb >> 16) & 0xff) as u8);
                        palette.push(((rgb >> 8) & 0xff) as u8);
                        palette.push((rgb & 0xff) as u8);
                    }
                }

                let diff_width = if min_x < max_x {
                    (max_x - min_x + 1) as u32
                } else {
                    0
                };
                let diff_height = if min_y < max_y {
                    (max_y - min_y + 1) as u32
                } else {
                    0
                };
                buffer = Resource::scale_buffer(&buffer, diff_width, diff_height, CAPTURE_SCALE);
                let frame_top: u16;
                let frame_left: u16;
                let frame_width: u16;
                let frame_height: u16;
                if buffer.is_empty() {
                    frame_top = 0;
                    frame_left = 0;
                    frame_width = 1;
                    frame_height = 1;
                    buffer.push(0);
                } else {
                    frame_top = (min_y as u16) * CAPTURE_SCALE as u16;
                    frame_left = (min_x as u16) * CAPTURE_SCALE as u16;
                    frame_width = (diff_width * CAPTURE_SCALE) as u16;
                    frame_height = (diff_height * CAPTURE_SCALE) as u16;
                }
                encoder
                    .write_frame(&Frame {
                        delay: screen.delay,
                        dispose: DisposalMethod::Keep,
                        transparent: Some(0),
                        needs_user_input: false,
                        top: frame_top,
                        left: frame_left,
                        width: frame_width,
                        height: frame_height,
                        interlaced: false,
                        palette: Some(palette),
                        buffer: Cow::Borrowed(&buffer),
                    })
                    .unwrap();
            }
            prev_rgb_image = rgb_image;
        }
        self.reset_capture();
        self.system.disable_next_frame_skip();
    }
}
