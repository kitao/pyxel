use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use chrono::Local;
use gifski::new as gifski_new;
use gifski::Settings;
use imgref::ImgVec;
use platform_dirs::UserDirs;
use rgb::RGBA8;
use zip::{ZipArchive, ZipWriter};

use crate::image::{Image, SharedImage};
use crate::music::Music;
use crate::settings::{
    CAPTURE_SCALE, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, PYXEL_VERSION,
    RESOURCE_ARCHIVE_DIRNAME,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::utils::parse_version_string;
use crate::Pyxel;

#[derive(Clone)]
struct Screen {
    image: SharedImage,
    frame_count: u32,
}

pub struct Resource {
    fps: u32,
    max_screen_count: u32,
    screens: Vec<Screen>,
    start_screen_index: u32,
    next_screen_index: u32,
    captured_screen_count: u32,
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
        let max_screen_count = fps * capture_sec;
        let screens = (0..max_screen_count)
            .map(|_| Screen {
                image: Image::new(width, height),
                frame_count: 0,
            })
            .collect();
        Self {
            fps,
            max_screen_count,
            screens,
            start_screen_index: 0,
            next_screen_index: 0,
            captured_screen_count: 0,
        }
    }

    pub fn capture_screen(&mut self, screen: SharedImage, frame_count: u32) {
        if self.max_screen_count == 0 {
            return;
        }
        let width = screen.lock().width();
        let height = screen.lock().height();
        self.screens[self.next_screen_index as usize]
            .image
            .lock()
            .blt(
                0.0,
                0.0,
                screen,
                0.0,
                0.0,
                width as f64,
                height as f64,
                None,
            );
        self.screens[self.next_screen_index as usize].frame_count = frame_count;
        self.next_screen_index = (self.next_screen_index + 1) % self.max_screen_count;
        self.captured_screen_count += 1;
        if self.captured_screen_count > self.max_screen_count {
            self.start_screen_index = (self.start_screen_index + 1) % self.max_screen_count;
            self.captured_screen_count = self.max_screen_count;
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
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(File::open(&Path::new(filename)).unwrap()).unwrap();
        let version = {
            let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let version = parse_version_string(&contents).unwrap();
            if version > parse_version_string(PYXEL_VERSION).unwrap() {
                panic!("unsupported resource file version '{}'", contents);
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
        let file = std::fs::File::create(&path).unwrap();
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

    // Advanced API
    pub fn screenshot(&mut self) {
        println!("{}", &Resource::export_path());
        self.screen
            .lock()
            .save(&Resource::export_path(), &self.colors, CAPTURE_SCALE);
        self.system.disable_next_frame_skip();
    }

    // Advanced API
    pub fn reset_capture(&mut self) {
        if self.resource.max_screen_count == 0 {
            return;
        }
        self.resource.start_screen_index = 0;
        self.resource.next_screen_index = 0;
        self.resource.captured_screen_count = 0;
    }

    // Advanced API
    pub fn screencast(&mut self) {
        if self.resource.max_screen_count == 0 || self.resource.captured_screen_count == 0 {
            return;
        }
        let (mut collector, writer) = gifski_new(Settings::default()).unwrap();
        let colors = self.colors;
        let fps = self.resource.fps;
        let max_screen_count = self.resource.max_screen_count;
        let screens = self.resource.screens.clone();
        let start_screen_index = self.resource.start_screen_index;
        let captured_screen_count = self.resource.captured_screen_count;
        let start_frame_count = screens[start_screen_index as usize].frame_count;
        let handle = std::thread::spawn(move || {
            for i in 0..captured_screen_count {
                let index = (start_screen_index + i) % max_screen_count;
                let image = &screens[index as usize].image.lock();
                let width = image.width();
                let height = image.height();
                let imgvec = ImgVec::new(
                    (0..width * CAPTURE_SCALE * height * CAPTURE_SCALE)
                        .map(|i| {
                            let i = i / CAPTURE_SCALE;
                            let x = i % width;
                            let y = i / (width * CAPTURE_SCALE);
                            let rgb = colors[image.canvas.data[y as usize][x as usize] as usize];
                            RGBA8::new(
                                ((rgb >> 16) & 0xff) as u8,
                                ((rgb >> 8) & 0xff) as u8,
                                (rgb & 0xff) as u8,
                                0xff,
                            )
                        })
                        .collect(),
                    (width * CAPTURE_SCALE) as usize,
                    (height * CAPTURE_SCALE) as usize,
                );
                let timestamp = (screens[index as usize].frame_count - start_frame_count + 1)
                    as f64
                    / fps as f64;
                collector
                    .add_frame_rgba(i as usize, imgvec, timestamp)
                    .unwrap();
            }
        });
        let mut file = File::create(&(Resource::export_path() + ".gif")).unwrap();
        writer
            .write(&mut file, &mut gifski::progress::NoProgress {})
            .unwrap();
        handle.join().unwrap();
        self.reset_capture();
        self.system.disable_next_frame_skip();
    }
}
