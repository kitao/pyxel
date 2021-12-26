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
use crate::rectarea::RectArea;
use crate::settings::{
    NUM_COLORS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, PYXEL_VERSION,
    RESOURCE_ARCHIVE_DIRNAME,
};
use crate::sound::Sound;
use crate::tilemap::Tilemap;
use crate::types::Rgb8;
use crate::utils::parse_version_string;
use crate::Pyxel;

#[derive(Clone)]
struct ScreenRecord {
    image: SharedImage,
    colors: [Rgb8; NUM_COLORS as usize],
    frame_count: u32,
    delay: u16,
}

impl ScreenRecord {
    fn to_rgb_image(&self) -> Vec<Vec<Rgb8>> {
        let image = &self.image.lock();
        let width = image.width();
        let height = image.height();
        let mut rgb_image: Vec<Vec<Rgb8>> = Vec::new();
        for y in 0..height {
            let mut rgb_line: Vec<Rgb8> = Vec::new();
            for x in 0..width {
                let rgb = self.colors[image.canvas.data[y as usize][x as usize] as usize];
                rgb_line.push(rgb);
            }
            rgb_image.push(rgb_line);
        }
        rgb_image
    }

    fn make_diff_image(
        base_image: &mut Vec<Vec<Rgb8>>,
        target_image: &[Vec<Rgb8>],
    ) -> (RectArea, Vec<Vec<Rgb8>>) {
        let mut min_x = i16::MAX;
        let mut min_y = i16::MAX;
        let mut max_x = i16::MIN;
        let mut max_y = i16::MIN;
        let mut diff_image: Vec<Vec<Rgb8>> = Vec::new();
        for y in 0..base_image.len() {
            let mut diff_line: Vec<Rgb8> = Vec::new();
            for x in 0..base_image[y].len() {
                let rgb = target_image[y][x];
                if rgb == base_image[y][x] {
                    diff_line.push(0xffffffff);
                } else {
                    min_x = min_x.min(x as i16);
                    min_y = min_y.min(y as i16);
                    max_x = max_x.max(x as i16);
                    max_y = max_y.max(y as i16);
                    base_image[y][x] = rgb;
                    diff_line.push(rgb);
                }
            }
            diff_image.push(diff_line);
        }
        if min_x > max_x || min_y > max_y {
            return (RectArea::new(0, 0, 0, 0), Vec::new());
        }
        diff_image = diff_image[min_y as usize..=max_y as usize].to_vec();
        for diff_line in &mut diff_image {
            *diff_line = diff_line[min_x as usize..=max_x as usize].to_vec();
        }
        (
            RectArea::new(
                min_x as i32,
                min_y as i32,
                diff_image[0].len() as u32,
                diff_image.len() as u32,
            ),
            diff_image,
        )
    }

    fn make_gif_buffer(rgb_image: &[Vec<Rgb8>], scale: u32) -> (Vec<u8>, Vec<u8>) {
        let mut color_table = IndexMap::<Rgb8, u8>::new();
        color_table.insert(0xffffffff, 0);
        let mut num_colors = 1;
        let mut buffer = Vec::new();
        for rgb_line in rgb_image {
            for rgb in rgb_line {
                if let Some(index) = color_table.get(rgb) {
                    buffer.push(*index);
                } else {
                    color_table.insert(*rgb, num_colors);
                    buffer.push(num_colors);
                    num_colors += 1;
                }
            }
        }
        if !buffer.is_empty() {
            let width = rgb_image[0].len() as u32;
            let height = rgb_image.len() as u32;
            let mut scaled_buffer: Vec<u8> = Vec::new();
            for y in 0..height * scale {
                for x in 0..width * scale {
                    let index = (y / scale) * width + x / scale;
                    scaled_buffer.push(buffer[index as usize])
                }
            }
            buffer = scaled_buffer;
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
        (palette, buffer)
    }
}

pub struct Resource {
    fps: u32,
    capture_scale: u32,
    max_screen_records: u32,
    num_screen_records: u32,
    screen_records: Vec<ScreenRecord>,
    screen_records_start_index: u32,
    screen_records_next_index: u32,
}

pub trait ResourceItem {
    fn resource_name(item_no: u32) -> String;
    fn is_modified(&self) -> bool;
    fn clear(&mut self);
    fn serialize(&self, pyxel: &Pyxel) -> String;
    fn deserialize(&mut self, pyxel: &Pyxel, version: u32, input: &str);
}

impl Resource {
    pub fn new(width: u32, height: u32, fps: u32, capture_scale: u32, capture_sec: u32) -> Self {
        let capture_scale = u32::max(capture_scale, 1);
        let max_screen_records = fps * capture_sec;
        let screen_records = (0..max_screen_records)
            .map(|_| ScreenRecord {
                image: Image::new(width, height),
                colors: [0; NUM_COLORS as usize],
                frame_count: 0,
                delay: 0,
            })
            .collect();
        Self {
            fps,
            capture_scale,
            max_screen_records,
            num_screen_records: 0,
            screen_records,
            screen_records_start_index: 0,
            screen_records_next_index: 0,
        }
    }

    pub fn capture_screen(
        &mut self,
        screen_image: SharedImage,
        colors: &[Rgb8; NUM_COLORS as usize],
        frame_count: u32,
    ) {
        if self.max_screen_records == 0 {
            return;
        }
        let prev_frame_count =
            self.screen_records[((self.screen_records_next_index + self.max_screen_records - 1)
                % self.max_screen_records) as usize]
                .frame_count;
        let screen_record = &mut self.screen_records[self.screen_records_next_index as usize];
        let width = screen_image.lock().width();
        let height = screen_image.lock().height();
        screen_record.colors = *colors;
        screen_record.image.lock().blt(
            0.0,
            0.0,
            screen_image,
            0.0,
            0.0,
            width as f64,
            height as f64,
            None,
        );
        screen_record.frame_count = frame_count;
        screen_record.delay = ((100.0 / self.fps as f64)
            * if self.num_screen_records == 0 {
                1.0
            } else {
                (screen_record.frame_count - prev_frame_count) as f64
            }
            + 0.5) as u16;
        self.screen_records_next_index =
            (self.screen_records_next_index + 1) % self.max_screen_records;
        self.num_screen_records += 1;
        if self.num_screen_records > self.max_screen_records {
            self.screen_records_start_index =
                (self.screen_records_start_index + 1) % self.max_screen_records;
            self.num_screen_records = self.max_screen_records;
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

    fn screen_record(&self, index: u32) -> &ScreenRecord {
        let index = (self.screen_records_start_index + index) % self.max_screen_records;
        &self.screen_records[index as usize]
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

    pub fn screenshot(&mut self, scale: Option<u32>) {
        let scale = u32::max(scale.unwrap_or(self.resource.capture_scale), 1);
        self.screen
            .lock()
            .save(&Resource::export_path(), &self.colors, scale);
        self.system.disable_next_frame_skip();
    }

    pub fn reset_capture(&mut self) {
        if self.resource.max_screen_records == 0 {
            return;
        }
        self.resource.screen_records_start_index = 0;
        self.resource.screen_records_next_index = 0;
        self.resource.num_screen_records = 0;
    }

    pub fn screencast(&mut self, scale: Option<u32>) {
        if self.resource.num_screen_records == 0 {
            return;
        }
        let width = self.width();
        let height = self.height();
        let scale = u32::max(scale.unwrap_or(self.resource.capture_scale), 1);
        let scaled_width = (width * scale) as u16;
        let scaled_height = (height * scale) as u16;
        let filename = Resource::export_path() + ".gif";
        let mut file = File::create(&filename)
            .unwrap_or_else(|_| panic!("Unable to open file '{}'", filename));
        let mut encoder = Encoder::new(&mut file, scaled_width, scaled_height, &[]).unwrap();
        encoder.set_repeat(Repeat::Infinite).unwrap();

        let screen_record = &self.resource.screen_record(0);
        let mut base_rgb_image = screen_record.to_rgb_image();
        let (palette, buffer) = ScreenRecord::make_gif_buffer(&base_rgb_image, scale);
        encoder
            .write_frame(&Frame {
                delay: screen_record.delay,
                dispose: DisposalMethod::Any,
                transparent: None,
                needs_user_input: false,
                top: 0,
                left: 0,
                width: scaled_width,
                height: scaled_height,
                interlaced: false,
                palette: Some(palette),
                buffer: Cow::Borrowed(&buffer),
            })
            .unwrap();

        for i in 1..self.resource.num_screen_records {
            let screen_record = &self.resource.screen_record(i);
            let rgb_image = screen_record.to_rgb_image();
            let (diff_rect, diff_image) =
                ScreenRecord::make_diff_image(&mut base_rgb_image, &rgb_image);
            let (palette, mut buffer) = ScreenRecord::make_gif_buffer(&diff_image, scale);
            let (top, left, width, height): (u16, u16, u16, u16) = if buffer.is_empty() {
                buffer = vec![0];
                (0, 0, 1, 1)
            } else {
                (
                    diff_rect.top() as u16 * scale as u16,
                    diff_rect.left() as u16 * scale as u16,
                    (diff_rect.width() * scale) as u16,
                    (diff_rect.height() * scale) as u16,
                )
            };
            encoder
                .write_frame(&Frame {
                    delay: screen_record.delay,
                    dispose: DisposalMethod::Keep,
                    transparent: Some(0),
                    needs_user_input: false,
                    top,
                    left,
                    width,
                    height,
                    interlaced: false,
                    palette: Some(palette),
                    buffer: Cow::Borrowed(&buffer),
                })
                .unwrap();
        }
        self.reset_capture();
        self.system.disable_next_frame_skip();
    }
}
