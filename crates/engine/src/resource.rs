use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use zip::{ZipArchive, ZipWriter};

use crate::capturer::Capturer;
use crate::image::Image;
use crate::music::Music;
use crate::settings::{
    IMAGE_COUNT, MUSIC_COUNT, PYXEL_VERSION, RESOURCE_ARCHIVE_DIRNAME, SOUND_COUNT, TILEMAP_COUNT,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::utils::{parse_version_string, pyxel_version};
use crate::Pyxel;

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self) -> String;
    fn deserialize(&mut self, input: &str);
}

pub struct Resource {
    capturer: Capturer,
}

impl Resource {
    pub fn new(width: u32, height: u32, capture_frame_count: u32) -> Resource {
        Resource {
            capturer: Capturer::new(width, height, capture_frame_count),
        }
    }

    pub fn capture_screen(&mut self, screen: &Image, frame_count: u32) {
        self.capturer.capture_screen(screen, frame_count);
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
        self.resource.capturer.screenshot(&self.colors);
    }

    pub fn reset_capture(&mut self) {
        self.resource.capturer.reset_capture();
    }

    pub fn screencast(&mut self) {
        self.resource.capturer.screencast(&self.colors);
    }
}
