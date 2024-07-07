use std::cmp::max;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use cfg_if::cfg_if;
use platform_dirs::UserDirs;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::image::{Color, Image, Rgb24};
use crate::pyxel::Pyxel;
use crate::resource_data::{ResourceData1, ResourceData2};
use crate::screencast::Screencast;
use crate::settings::{DEFAULT_CAPTURE_SCALE, DEFAULT_CAPTURE_SEC};
use crate::{PALETTE_FILE_EXTENSION, RESOURCE_ARCHIVE_NAME, RESOURCE_FORMAT_VERSION};

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
    pub fn load(
        &mut self,
        filename: &str,
        exclude_images: Option<bool>,
        exclude_tilemaps: Option<bool>,
        exclude_sounds: Option<bool>,
        exclude_musics: Option<bool>,
        include_colors: Option<bool>,
        include_channels: Option<bool>,
        include_tones: Option<bool>,
    ) {
        let mut archive = ZipArchive::new(
            File::open(Path::new(&filename))
                .unwrap_or_else(|_| panic!("Failed to open file '{filename}'")),
        )
        .unwrap();

        // Old resource file
        if archive.by_name("pyxel_resource/version").is_ok() {
            println!("An old Pyxel resource file '{filename}' is loaded. Please re-save it with the latest Pyxel.");
            self.load_old_resource(
                &mut archive,
                filename,
                !exclude_images.unwrap_or(false),
                !exclude_tilemaps.unwrap_or(false),
                !exclude_sounds.unwrap_or(false),
                !exclude_musics.unwrap_or(false),
            );
            self.load_pyxel_palette_file(filename);
            return;
        }

        // New resource file
        let mut file = archive.by_name(RESOURCE_ARCHIVE_NAME).unwrap();
        let mut toml_text = String::new();
        file.read_to_string(&mut toml_text).unwrap();
        let format_version = Self::parse_format_version(&toml_text);
        assert!(
            format_version <= RESOURCE_FORMAT_VERSION,
            "Unknown resource file version"
        );
        if format_version >= 2 {
            let resource_data = ResourceData2::from_toml(&toml_text);
            resource_data.to_runtime(
                self,
                exclude_images.unwrap_or(false),
                exclude_tilemaps.unwrap_or(false),
                exclude_sounds.unwrap_or(false),
                exclude_musics.unwrap_or(false),
                include_colors.unwrap_or(false),
                include_channels.unwrap_or(false),
                include_tones.unwrap_or(false),
            );
            self.load_pyxel_palette_file(filename);
        } else {
            let resource_data = ResourceData1::from_toml(&toml_text);
            resource_data.to_runtime(
                self,
                exclude_images.unwrap_or(false),
                exclude_tilemaps.unwrap_or(false),
                exclude_sounds.unwrap_or(false),
                exclude_musics.unwrap_or(false),
                include_colors.unwrap_or(false),
                include_channels.unwrap_or(false),
                include_tones.unwrap_or(false),
            );
            self.load_pyxel_palette_file(filename);
        }
    }

    pub fn save(
        &mut self,
        filename: &str,
        exclude_images: Option<bool>,
        exclude_tilemaps: Option<bool>,
        exclude_sounds: Option<bool>,
        exclude_musics: Option<bool>,
        include_colors: Option<bool>,
        include_channels: Option<bool>,
        include_tones: Option<bool>,
    ) {
        let toml_text = ResourceData2::from_runtime(self).to_toml(
            exclude_images.unwrap_or(false),
            exclude_tilemaps.unwrap_or(false),
            exclude_sounds.unwrap_or(false),
            exclude_musics.unwrap_or(false),
            include_colors.unwrap_or(false),
            include_channels.unwrap_or(false),
            include_tones.unwrap_or(false),
        );
        let path = std::path::Path::new(&filename);
        let file = std::fs::File::create(path)
            .unwrap_or_else(|_| panic!("Failed to open file '{filename}'"));
        let mut zip = ZipWriter::new(file);
        zip.start_file(RESOURCE_ARCHIVE_NAME, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(toml_text.as_bytes()).unwrap();
        zip.finish().unwrap();
        #[cfg(target_os = "emscripten")]
        pyxel_platform::emscripten::save_file(filename);
    }

    pub fn screenshot(&mut self, scale: Option<u32>) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-{}", Self::datetime_string()));
        let scale = max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.screen.lock().save(&filename, scale);
        #[cfg(target_os = "emscripten")]
        pyxel_platform::emscripten::save_file(&(filename + ".png"));
    }

    pub fn screencast(&mut self, scale: Option<u32>) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-{}", Self::datetime_string()));
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

    pub(crate) fn dump_image_bank(&self, image_index: u32) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-image{image_index}"));
        if let Some(image) = self.images.lock().get(image_index as usize) {
            image.lock().save(&filename, 1);
            #[cfg(target_os = "emscripten")]
            pyxel_platform::emscripten::save_file(&(filename + ".png"));
        }
    }

    pub(crate) fn dump_palette(&self) {
        let filename = Self::prepend_desktop_path("pyxel-palette");
        let num_colors = self.colors.lock().len();
        let image = Image::new(num_colors as u32, 1);
        {
            let mut image = image.lock();
            for i in 0..num_colors {
                image.pset(i as f64, 0.0, i as Color);
            }
            image.save(&filename, 16);
            #[cfg(target_os = "emscripten")]
            pyxel_platform::emscripten::save_file(&(filename + ".png"));
        }
    }

    fn datetime_string() -> String {
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                pyxel_platform::emscripten::datetime_string()
            } else {
                chrono::Local::now().format("%Y%m%d-%H%M%S").to_string()
            }
        }
    }

    fn prepend_desktop_path(basename: &str) -> String {
        let desktop_dir = if let Some(user_dirs) = UserDirs::new() {
            user_dirs.desktop_dir
        } else {
            PathBuf::new()
        };
        desktop_dir.join(basename).to_str().unwrap().to_string()
    }

    fn parse_format_version(toml_text: &str) -> u32 {
        toml_text
            .lines()
            .find(|line| line.trim().starts_with("format_version"))
            .and_then(|line| line.split_once('='))
            .map(|(_, value)| value.trim().parse::<u32>())
            .unwrap()
            .unwrap()
    }

    fn load_pyxel_palette_file(&mut self, filename: &str) {
        let filename = filename
            .rfind('.')
            .map_or(filename, |i| &filename[..i])
            .to_string()
            + PALETTE_FILE_EXTENSION;
        if let Ok(mut file) = File::open(Path::new(&filename)) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            *self.colors.lock() = contents
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| u32::from_str_radix(s.trim(), 16).unwrap() as Rgb24)
                .collect();
        }
    }
}
