use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use chrono::Local;
use platform_dirs::UserDirs;
use zip::write::FileOptions as ZipFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::image::Image;
use crate::music::Music;
use crate::screencast::Screencast;
use crate::settings::{
    NUM_COLORS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, PYXEL_VERSION,
    RESOURCE_ARCHIVE_DIRNAME,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::types::{Color, Rgb8};
use crate::utils::parse_version_string;

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self) -> String;
    fn deserialize(&mut self, version: u32, input: &str);
}

pub struct Resource {
    capture_scale: u32,
    screencast: Screencast,
}

unsafe_singleton!(Resource);

impl Resource {
    pub fn init(fps: u32, capture_scale: u32, capture_sec: u32) {
        Self::set_instance(Self {
            capture_scale: u32::max(capture_scale, 1),
            screencast: Screencast::new(fps, capture_sec),
        });
    }

    pub fn capture_screen(
        &mut self,
        image: &[Vec<Color>],
        colors: &[Rgb8; NUM_COLORS as usize],
        frame_count: u32,
    ) {
        self.screencast.capture(image, colors, frame_count);
    }

    fn export_path() -> Option<String> {
        UserDirs::new().map(|user_dirs| {
            user_dirs
                .desktop_dir
                .join(Local::now().format("pyxel-%Y%m%d-%H%M%S").to_string())
                .to_str()
                .unwrap()
                .to_string()
        })
    }
}

pub fn load(filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
    let mut archive = ZipArchive::new(
        File::open(&Path::new(&filename))
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename)),
    )
    .unwrap_or_else(|_| panic!("Unable to parse zip archive '{}'", filename));
    let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
    let contents = {
        let mut file = archive.by_name(&version_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        contents
    };
    let version = parse_version_string(&contents).unwrap();
    assert!(
        version <= parse_version_string(PYXEL_VERSION).unwrap(),
        "Unsupported resource file version '{}'",
        contents
    );

    macro_rules! deserialize {
        ($type: ty, $getter: ident, $count: expr) => {
            for i in 0..$count {
                if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                    let mut input = String::new();
                    file.read_to_string(&mut input).unwrap();
                    crate::$getter(i).lock().deserialize(version, &input);
                } else {
                    crate::$getter(i).lock().clear();
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

pub fn save(filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
    let path = std::path::Path::new(&filename);
    let file = std::fs::File::create(&path)
        .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
    let mut zip = ZipWriter::new(file);
    zip.add_directory(RESOURCE_ARCHIVE_DIRNAME, ZipFileOptions::default())
        .unwrap();
    let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
    zip.start_file(version_name, ZipFileOptions::default())
        .unwrap();
    zip.write_all(PYXEL_VERSION.as_bytes()).unwrap();

    macro_rules! serialize {
        ($type: ty, $getter: ident, $count: expr) => {
            for i in 0..$count {
                if crate::$getter(i).lock().is_modified() {
                    zip.start_file(<$type>::resource_name(i), ZipFileOptions::default())
                        .unwrap();
                    zip.write_all(crate::$getter(i).lock().serialize().as_bytes())
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

pub fn screenshot(scale: Option<u32>) {
    Resource::export_path().map_or_else(
        || println!("Failed to save screenshot to desktop"),
        |filename| {
            let scale = u32::max(scale.unwrap_or(Resource::instance().capture_scale), 1);
            crate::screen().lock().save(&filename, scale);
        },
    );
}

pub fn reset_capture() {
    Resource::instance().screencast.reset();
}

pub fn screencast(scale: Option<u32>) {
    Resource::export_path().map_or_else(
        || println!("Failed to save screen capture video to desktop"),
        |filename| {
            let scale = u32::max(scale.unwrap_or(Resource::instance().capture_scale), 1);
            Resource::instance().screencast.save(&filename, scale);
        },
    );
}
