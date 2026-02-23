use std::cmp::max;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf, MAIN_SEPARATOR};

use directories::UserDirs;
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

use crate::image::{Color, Image, Rgb24};
use crate::pyxel::{self, Pyxel};
use crate::resource_data::ResourceData;
use crate::screencast::Screencast;
use crate::settings::{
    BASE_DIR, DEFAULT_CAPTURE_SCALE, DEFAULT_CAPTURE_SEC, PALETTE_FILE_EXTENSION,
    RESOURCE_ARCHIVE_NAME, RESOURCE_FILE_EXTENSION, RESOURCE_FORMAT_VERSION,
};

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
    pub fn load_resource(
        &mut self,
        filename: &str,
        exclude_images: Option<bool>,
        exclude_tilemaps: Option<bool>,
        exclude_sounds: Option<bool>,
        exclude_musics: Option<bool>,
    ) -> Result<(), String> {
        let file = File::open(Path::new(&filename))
            .map_err(|_e| format!("Failed to open file '{filename}'"))?;
        let mut archive = ZipArchive::new(file).unwrap();

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
            self.load_palette(filename);
            return Ok(());
        }

        // New resource file
        let mut file = archive.by_name(RESOURCE_ARCHIVE_NAME).unwrap();
        let mut toml_text = String::new();
        file.read_to_string(&mut toml_text).unwrap();

        let format_version = Self::parse_format_version(&toml_text);
        if format_version > RESOURCE_FORMAT_VERSION {
            Err(format!("Unknown resource file version '{format_version}'"))
        } else {
            let resource_data = ResourceData::from_toml(&toml_text);
            resource_data.to_runtime(
                self,
                exclude_images.unwrap_or(false),
                exclude_tilemaps.unwrap_or(false),
                exclude_sounds.unwrap_or(false),
                exclude_musics.unwrap_or(false),
            );
            self.load_palette(filename);
            Ok(())
        }
    }

    pub fn save_resource(
        &mut self,
        filename: &str,
        exclude_images: Option<bool>,
        exclude_tilemaps: Option<bool>,
        exclude_sounds: Option<bool>,
        exclude_musics: Option<bool>,
    ) -> Result<(), String> {
        let toml_text = ResourceData::from_runtime(self).to_toml(
            exclude_images.unwrap_or(false),
            exclude_tilemaps.unwrap_or(false),
            exclude_sounds.unwrap_or(false),
            exclude_musics.unwrap_or(false),
        );

        let path = Path::new(&filename);
        let file = File::create(path).map_err(|_e| format!("Failed to open file '{filename}'"))?;

        let mut zip = ZipWriter::new(file);
        zip.start_file(RESOURCE_ARCHIVE_NAME, SimpleFileOptions::default())
            .unwrap();
        zip.write_all(toml_text.as_bytes()).unwrap();
        zip.finish().unwrap();

        crate::platform::export_browser_file(filename);
        Ok(())
    }

    pub fn load_palette(&mut self, filename: &str) {
        let filename = Self::palette_filename(filename);

        if let Ok(mut file) = File::open(Path::new(&filename)) {
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let colors: Vec<Rgb24> = contents
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| u32::from_str_radix(s.trim(), 16).unwrap() as Rgb24)
                .collect();
            *pyxel::colors() = if colors.is_empty() {
                vec![0xffffff]
            } else {
                colors
            };
        }
    }

    pub fn save_palette(&self, filename: &str) -> Result<(), String> {
        let filename = Self::palette_filename(filename);
        let mut file = File::create(Path::new(&filename))
            .map_err(|_e| format!("Failed to open file '{filename}'"))?;

        let colors = pyxel::colors();
        for &color in colors.iter() {
            writeln!(file, "{color:06x}").unwrap();
        }

        crate::platform::export_browser_file(&filename);
        Ok(())
    }

    pub fn take_screenshot(&mut self, scale: Option<u32>) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-{}", Self::datetime_string()));
        let scale = max(scale.unwrap_or(self.resource.capture_scale), 1);
        pyxel::screen().save(&filename, scale).unwrap();

        crate::platform::export_browser_file(&(filename + ".png"));
    }

    pub fn save_screencast(&mut self, scale: Option<u32>) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-{}", Self::datetime_string()));
        let scale = max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.resource.screencast.save(&filename, scale);

        crate::platform::export_browser_file(&(filename + ".gif"));
    }

    pub fn reset_screencast(&mut self) {
        self.resource.screencast.reset();
    }

    pub fn user_data_dir(&self, vendor_name: &str, app_name: &str) -> String {
        let home_dir = UserDirs::new()
            .map_or_else(PathBuf::new, |user_dirs| user_dirs.home_dir().to_path_buf());
        let app_data_dir = home_dir
            .join(BASE_DIR)
            .join(Self::make_dir_name(vendor_name))
            .join(Self::make_dir_name(app_name));

        if !app_data_dir.exists() {
            fs::create_dir_all(&app_data_dir).unwrap();
            println!("created '{}'", app_data_dir.to_string_lossy());
        }

        let mut app_data_dir = app_data_dir.to_string_lossy().to_string();
        if !app_data_dir.ends_with(MAIN_SEPARATOR) {
            app_data_dir.push(MAIN_SEPARATOR);
        }

        app_data_dir
    }

    pub(crate) fn capture_screen(&mut self) {
        self.resource.screencast.capture(
            *pyxel::width(),
            *pyxel::height(),
            &pyxel::screen().canvas.data,
            pyxel::colors(),
            *pyxel::frame_count(),
        );
    }

    pub(crate) fn dump_image_bank(&self, image_index: u32) {
        let filename = Self::prepend_desktop_path(&format!("pyxel-image{image_index}"));

        if let Some(&image) = pyxel::images().get(image_index as usize) {
            unsafe { &*image }.save(&filename, 1).unwrap();

            crate::platform::export_browser_file(&(filename + ".png"));
        }
    }

    pub(crate) fn dump_palette(&self) {
        let filename = Self::prepend_desktop_path("pyxel-palette");
        let num_colors = pyxel::colors().len();
        let image = Image::new(num_colors as u32, 1);

        {
            let image = unsafe { &mut *image };
            for i in 0..num_colors {
                image.set_pixel(i as f32, 0.0, i as Color);
            }

            image.save(&filename, 16).unwrap();

            crate::platform::export_browser_file(&(filename + ".png"));
        }
    }

    fn palette_filename(filename: &str) -> String {
        if filename.to_lowercase().ends_with(RESOURCE_FILE_EXTENSION) {
            filename[..filename.len() - RESOURCE_FILE_EXTENSION.len()].to_string()
                + PALETTE_FILE_EXTENSION
        } else {
            filename.to_string()
        }
    }

    fn datetime_string() -> String {
        chrono::Local::now().format("%Y%m%d-%H%M%S").to_string()
    }

    fn prepend_desktop_path(basename: &str) -> String {
        let desktop_dir = UserDirs::new()
            .and_then(|user_dirs| user_dirs.desktop_dir().map(Path::to_path_buf))
            .unwrap_or_default();

        desktop_dir.join(basename).to_string_lossy().to_string()
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

    fn make_dir_name(name: &str) -> String {
        name.to_lowercase()
            .replace(' ', "_")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
            .collect()
    }
}
