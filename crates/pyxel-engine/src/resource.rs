use std::cmp::max;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use cfg_if::cfg_if;
use platform_dirs::UserDirs;
use zip::write::FileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::image::Image;
use crate::music::Music;
use crate::pyxel::Pyxel;
use crate::screencast::Screencast;
use crate::settings::{
    DEFAULT_CAPTURE_SCALE, DEFAULT_CAPTURE_SEC, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS,
    RESOURCE_ARCHIVE_DIRNAME, VERSION,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;

pub trait ResourceItem {
    fn resource_name(item_index: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self) -> String;
    fn deserialize(&mut self, version: u32, input: &str);
}

pub struct Resource {
    capture_scale: u32,
    screencast: Screencast,
}

impl Resource {
    pub fn new(capture_scale: Option<u32>, capture_sec: Option<u32>, fps: u32) -> Self {
        let capture_scale = capture_scale.unwrap_or(DEFAULT_CAPTURE_SCALE);
        let capture_sec = capture_sec.unwrap_or(DEFAULT_CAPTURE_SEC);
        Self {
            capture_scale: max(capture_scale, 1),
            screencast: Screencast::new(fps, capture_sec),
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
        self.load_old_resource(&mut archive, &filename, image, tilemap, sound, music);
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
        zip.write_all(VERSION.as_bytes()).unwrap();

        macro_rules! serialize {
            ($type: ty, $list: ident, $count: expr) => {
                for i in 0..$count {
                    if self.$list.lock()[i as usize].lock().is_modified() {
                        zip.start_file(<$type>::resource_name(i), FileOptions::default())
                            .unwrap();
                        zip.write_all(self.$list.lock()[i as usize].lock().serialize().as_bytes())
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
        pyxel_platform::emscripten::save_file(filename);
    }

    pub fn screenshot(&self, scale: Option<u32>) {
        let filename = Self::export_path();
        let scale = max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.screen.lock().save(&filename, scale);
        #[cfg(target_os = "emscripten")]
        pyxel_platform::emscripten::save_file(&(filename + ".png"));
    }

    pub fn screencast(&mut self, scale: Option<u32>) {
        let filename = Self::export_path();
        let scale = max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.resource.screencast.save(&filename, scale);
        #[cfg(target_os = "emscripten")]
        pyxel_platform::emscripten::save_file(&(filename + ".gif"));
    }

    pub fn reset_screencast(&mut self) {
        self.resource.screencast.reset();
    }

    pub(crate) fn capture_screen(&mut self) {
        self.resource.screencast.capture(
            self.width,
            self.height,
            &self.screen.lock().canvas.data,
            &self.colors.lock(),
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
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                pyxel_platform::emscripten::timestamp_string()
            } else {
                chrono::Local::now().format("%Y%m%d-%H%M%S").to_string()
            }
        }
    }
}
