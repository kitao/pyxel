use chrono::Local;
use gif::{DisposalMethod, Encoder, Frame, Repeat};
use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipArchive, ZipWriter};

use crate::canvas::Canvas;
use crate::image::Image;
use crate::music::Music;
use crate::settings::{
    CAPTURE_SCALE, COLOR_COUNT, IMAGE_COUNT, MUSIC_COUNT, PYXEL_VERSION, RESOURCE_ARCHIVE_DIRNAME,
    SOUND_COUNT, TILEMAP_COUNT,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::utils::{parse_version_string, pyxel_version};
use crate::Pyxel;

struct CaptureFrame {
    frame_image: Image,
    frame_count: u32,
}

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self) -> String;
    fn deserialize(&mut self, input: &str);
}

pub struct Resource {
    fps: u32,
    capture_frame_count: u32,
    capture_frames: Vec<CaptureFrame>,
    start_frame_index: u32,
    cur_frame_index: u32,
    cur_frame_count: u32,
}

impl Resource {
    pub fn new(width: u32, height: u32, fps: u32, capture_sec: u32) -> Resource {
        let capture_frame_count = fps * capture_sec;
        let capture_frames = (0..capture_frame_count)
            .map(|_| CaptureFrame {
                frame_image: Image::without_arc_mutex(width, height),
                frame_count: 0,
            })
            .collect();

        Resource {
            fps,
            capture_frame_count,
            capture_frames,
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

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(File::open(&Path::new(filename)).unwrap()).unwrap();

        {
            let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            if parse_version_string(&contents).unwrap() > pyxel_version() {
                panic!("unsupported resource file version '{}'", contents);
            }
        }

        macro_rules! deserialize {
            ($type: ty, $getter: ident, $count: expr) => {
                for i in 0..$count {
                    if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                        let mut input = String::new();
                        file.read_to_string(&mut input).unwrap();
                        self.$getter(i).lock().deserialize(&input);
                    } else {
                        self.$getter(i).lock().clear();
                    }
                }
            };
        }

        if image {
            deserialize!(Image, image, IMAGE_COUNT);
        }

        if tilemap {
            deserialize!(Tilemap, tilemap, TILEMAP_COUNT);
        }

        if sound {
            deserialize!(Sound, sound, SOUND_COUNT);
        }

        if music {
            deserialize!(Music, music, MUSIC_COUNT);
        }
    }

    pub fn save(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let path = std::path::Path::new(filename);
        let file = std::fs::File::create(&path).unwrap();
        let mut zip = ZipWriter::new(file);

        zip.add_directory(RESOURCE_ARCHIVE_DIRNAME, Default::default())
            .unwrap();

        let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
        zip.start_file(version_name, Default::default()).unwrap();
        zip.write_all(PYXEL_VERSION.as_bytes()).unwrap();

        macro_rules! serialize {
            ($type: ty, $getter: ident, $count: expr) => {
                for i in 0..$count {
                    if self.$getter(i).lock().is_modified() {
                        zip.start_file(<$type>::resource_name(i), Default::default())
                            .unwrap();
                        zip.write_all(self.$getter(i).lock().serialize().as_bytes())
                            .unwrap();
                    }
                }
            };
        }

        if image {
            serialize!(Image, image, IMAGE_COUNT);
        }

        if tilemap {
            serialize!(Tilemap, tilemap, TILEMAP_COUNT);
        }

        if sound {
            serialize!(Sound, sound, SOUND_COUNT);
        }

        if music {
            serialize!(Music, music, MUSIC_COUNT);
        }

        zip.finish().unwrap();
    }

    pub fn screenshot(&mut self) {
        self.resource.capture_frames[self.resource.cur_frame_index as usize]
            .frame_image
            .save(&Resource::export_path(), &self.colors, CAPTURE_SCALE);
    }

    pub fn reset_capture(&mut self) {
        if self.resource.capture_frame_count == 0 {
            return;
        }

        self.resource.start_frame_index =
            (self.resource.cur_frame_index + 1) % self.resource.capture_frame_count;
        self.resource.cur_frame_count = 0;
    }

    pub fn screencast(&mut self) {
        if self.resource.capture_frame_count == 0 || self.resource.cur_frame_count == 0 {
            return;
        }

        let mut last_frame_image =
            &self.resource.capture_frames[self.resource.cur_frame_index as usize].frame_image;
        let mut last_frame_count =
            self.resource.capture_frames[self.resource.start_frame_index as usize].frame_count;
        let width = last_frame_image.width() * CAPTURE_SCALE;
        let height = last_frame_image.height() * CAPTURE_SCALE;

        let mut palette = Vec::new();
        for color in self.colors {
            palette.push(((color >> 16) & 0xff) as u8);
            palette.push(((color >> 8) & 0xff) as u8);
            palette.push((color & 0xff) as u8);
        }
        palette.append(&mut vec![0; 3]);

        let mut image = File::create(Resource::export_path() + ".gif").unwrap();
        let mut encoder = Encoder::new(&mut image, width as u16, height as u16, &palette).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        {
            let mut buffer = Vec::new();

            for i in 0..height {
                for j in 0..width {
                    let x = j / CAPTURE_SCALE;
                    let y = i / CAPTURE_SCALE;

                    buffer.push(last_frame_image._value(x as i32, y as i32));
                }
            }

            encoder
                .write_frame(&Frame {
                    delay: 0,
                    dispose: DisposalMethod::Keep,
                    transparent: None,
                    needs_user_input: false,
                    top: 0,
                    left: 0,
                    width: width as u16,
                    height: height as u16,
                    interlaced: false,
                    palette: None,
                    buffer: Cow::Borrowed(&*buffer),
                })
                .unwrap();
        }

        for i in 0..self.resource.cur_frame_count {
            let index = (self.resource.start_frame_index + i) % self.resource.capture_frame_count;
            let frame_image = &self.resource.capture_frames[index as usize].frame_image;
            let frame_count = self.resource.capture_frames[index as usize].frame_count;
            let mut buffer = Vec::new();

            for j in 0..height {
                for k in 0..width {
                    let x = k / CAPTURE_SCALE;
                    let y = j / CAPTURE_SCALE;
                    let value = frame_image._value(x as i32, y as i32);

                    buffer.push(if value != last_frame_image._value(x as i32, y as i32) {
                        value
                    } else {
                        COLOR_COUNT as u8
                    });
                }
            }

            let delay = ((frame_count - last_frame_count) as f64 * 100.0 / self.resource.fps as f64)
                .round() as u16;

            encoder
                .write_frame(&Frame {
                    delay,
                    dispose: DisposalMethod::Keep,
                    transparent: Some(COLOR_COUNT as u8),
                    needs_user_input: false,
                    top: 0,
                    left: 0,
                    width: width as u16,
                    height: height as u16,
                    interlaced: false,
                    palette: None,
                    buffer: Cow::Borrowed(&*buffer),
                })
                .unwrap();

            last_frame_image = frame_image;
            last_frame_count = frame_count;
        }

        /*
        // try to optimize the generated GIF file with Gifsicle
        int32_t res = system(("gifsicle -b -O3 -Okeep-empty " + filename).c_str());
        */

        self.reset_capture();
    }
}
