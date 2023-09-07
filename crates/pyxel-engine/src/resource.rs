use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use chrono::Local;
use platform_dirs::UserDirs;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::prelude::*;
use crate::screencast::Screencast;
use crate::utils;

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self, pyxel: &Pyxel) -> String;
    fn deserialize(&mut self, pyxel: &Pyxel, version: u32, input: &str);
}

pub(crate) struct Resource {
    capture_scale: u32,
    screencast: Screencast,
}

impl Resource {
    pub(crate) fn new(capture_scale: Option<u32>, capture_sec: Option<u32>, fps: u32) -> Self {
        let capture_scale = capture_scale.unwrap_or(DEFAULT_CAPTURE_SCALE);
        let capture_sec = capture_sec.unwrap_or(DEFAULT_CAPTURE_SEC);
        Self {
            capture_scale: u32::max(capture_scale, 1),
            screencast: Screencast::new(fps, capture_sec),
        }
    }
}

impl Pyxel {
    pub(crate) fn capture_screen(&mut self) {
        self.resource.screencast.capture(
            self.width,
            self.height,
            &self.screen.lock().canvas.data,
            &self.colors,
            self.frame_count,
        );
    }

    fn export_path() -> String {
        let desktop_dir = if let Some(user_dirs) = UserDirs::new() {
            user_dirs.desktop_dir
        } else {
            PathBuf::new()
        };
        let basename = "pyxel-".to_string() + &Self::local_time_string();
        desktop_dir.join(basename).to_str().unwrap().to_string()
    }

    fn local_time_string() -> String {
        #[cfg(not(target_os = "emscripten"))]
        return Local::now().format("%Y%m%d-%H%M%S").to_string();

        #[cfg(target_os = "emscripten")]
        {
            let script = "
            let now = new Date();
            let year = now.getFullYear();
            let month = now.getMonth() + 1;
            let date = now.getDate();
            let hour = now.getHours();
            let min = now.getMinutes();
            let sec = now.getSeconds();
            `${year}${month}${date}-${hour}${min}${sec}`
        ";
            emscripten::run_script_string(script)
        }
    }
}

impl Pyxel {
    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let mut archive = ZipArchive::new(
            File::open(Path::new(&filename))
                .unwrap_or_else(|_| panic!("Unable to open file '{filename}'")),
        )
        .unwrap_or_else(|_| panic!("Unable to parse zip archive '{filename}'"));
        let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
        let contents = {
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        };
        let version = utils::parse_version_string(&contents).unwrap();
        assert!(
            version <= utils::parse_version_string(PYXEL_VERSION).unwrap(),
            "Unsupported resource file version '{contents}'"
        );

        macro_rules! deserialize {
            ($type: ty, $list: ident, $count: expr) => {
                for i in 0..$count {
                    if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                        let mut input = String::new();
                        file.read_to_string(&mut input).unwrap();
                        self.$list[i as usize]
                            .lock()
                            .deserialize(self, version, &input);
                    } else {
                        self.$list[i as usize].lock().clear();
                    }
                }
            };
        }

        if image {
            deserialize!(Image, images, NUM_IMAGES);
        }
        if tilemap {
            deserialize!(Tilemap, tilemaps, NUM_TILEMAPS);
        }
        if sound {
            deserialize!(Sound, sounds, NUM_SOUNDS);
        }
        if music {
            deserialize!(Music, musics, NUM_MUSICS);
        }

        // Try to load Pyxel palette file
        let filename = filename
            .rfind('.')
            .map_or(filename, |i| &filename[..i])
            .to_string()
            + PALETTE_FILE_EXTENSION;
        if let Ok(mut file) = File::open(Path::new(&filename)) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let colors: Vec<Rgb8> = contents
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| u32::from_str_radix(s.trim(), 16).unwrap() as Rgb8)
                .take(NUM_COLORS as usize)
                .collect();
            self.colors.clear();
            self.colors.extend_from_slice(&colors);
        }
    }

    pub fn save(&self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        let path = std::path::Path::new(&filename);
        let file = std::fs::File::create(path)
            .unwrap_or_else(|_| panic!("Unable to open file '{filename}'"));
        let mut zip = ZipWriter::new(file);
        zip.add_directory(RESOURCE_ARCHIVE_DIRNAME, FileOptions::default())
            .unwrap();
        let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
        zip.start_file(version_name, FileOptions::default())
            .unwrap();
        zip.write_all(PYXEL_VERSION.as_bytes()).unwrap();

        macro_rules! serialize {
            ($type: ty, $list: ident, $count: expr) => {
                for i in 0..$count {
                    if self.$list[i as usize].lock().is_modified() {
                        zip.start_file(<$type>::resource_name(i), FileOptions::default())
                            .unwrap();
                        zip.write_all(self.$list[i as usize].lock().serialize(self).as_bytes())
                            .unwrap();
                    }
                }
            };
        }

        if image {
            serialize!(Image, images, NUM_IMAGES);
        }
        if tilemap {
            serialize!(Tilemap, tilemaps, NUM_TILEMAPS);
        }
        if sound {
            serialize!(Sound, sounds, NUM_SOUNDS);
        }
        if music {
            serialize!(Music, musics, NUM_MUSICS);
        }
        zip.finish().unwrap();
        #[cfg(target_os = "emscripten")]
        Platform::save_file_on_web_browser(filename);
    }
    pub fn screenshot(&self, scale: Option<u32>) {
        let filename = Self::export_path();
        let scale = u32::max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.screen.lock().save(&filename, scale, &self.colors);
        #[cfg(target_os = "emscripten")]
        Platform::save_file_on_web_browser(&(filename + ".png"));
    }

    pub fn screencast(&mut self, scale: Option<u32>) {
        let filename = Self::export_path();
        let scale = u32::max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.resource.screencast.save(&filename, scale);
        #[cfg(target_os = "emscripten")]
        Platform::save_file_on_web_browser(&(filename + ".gif"));
    }

    pub fn reset_screencast(&mut self) {
        self.resource.screencast.reset();
    }
}
