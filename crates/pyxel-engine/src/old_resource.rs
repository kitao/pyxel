use std::fmt::{self, Write as _};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip::ZipArchive;

use crate::image::{Color, Image, Rgb24};
use crate::music::Music;
use crate::pyxel::Pyxel;
use crate::resource::ResourceItem;
use crate::settings::TILEMAP_SIZE;
use crate::settings::{
    INITIAL_SPEED, NUM_CHANNELS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS,
    PALETTE_FILE_EXTENSION, RESOURCE_ARCHIVE_DIRNAME, VERSION,
};
use crate::sound::Sound;
use crate::tilemap::{ImageSource, Tilemap};
use crate::utils::{parse_hex_string, parse_version_string};

impl ResourceItem for Image {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "image" + &item_index.to_string()
    }

    fn is_modified(&self) -> bool {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.canvas.read_data(x as usize, y as usize) != 0 {
                    return true;
                }
            }
        }
        false
    }

    fn clear(&mut self) {
        self.cls(0);
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let _guard = write!(
                    output,
                    "{:1x}",
                    self.canvas.read_data(x as usize, y as usize)
                );
            }
            output += "\n";
        }
        output
    }

    fn deserialize(&mut self, _version: u32, input: &str) {
        for (i, line) in input.lines().enumerate() {
            string_loop!(j, color, line, 1, {
                self.canvas
                    .write_data(j, i, parse_hex_string(&color).unwrap() as Color);
            });
        }
    }
}

impl fmt::Display for ImageSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageSource::Index(index) => write!(f, "{index}"),
            ImageSource::Image(_) => write!(f, "0"),
        }
    }
}

impl ResourceItem for Tilemap {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "tilemap" + &item_index.to_string()
    }

    fn is_modified(&self) -> bool {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.canvas.read_data(x as usize, y as usize) != (0, 0) {
                    return true;
                }
            }
        }
        false
    }

    fn clear(&mut self) {
        self.cls((0, 0));
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tile = self.canvas.read_data(x as usize, y as usize);
                let _guard = write!(output, "{:02x}{:02x}", tile.0, tile.1);
            }
            output += "\n";
        }
        let _guard = write!(output, "{}", self.imgsrc);
        output
    }

    fn deserialize(&mut self, version: u32, input: &str) {
        for (y, line) in input.lines().enumerate() {
            if y < TILEMAP_SIZE as usize {
                if version < 10500 {
                    string_loop!(x, tile, line, 3, {
                        let tile = parse_hex_string(&tile).unwrap();
                        self.canvas
                            .write_data(x, y, ((tile % 32) as u8, (tile / 32) as u8));
                    });
                } else {
                    string_loop!(x, tile, line, 4, {
                        let tile_x = parse_hex_string(&tile[0..2]).unwrap();
                        let tile_y = parse_hex_string(&tile[2..4]).unwrap();
                        self.canvas.write_data(x, y, (tile_x as u8, tile_y as u8));
                    });
                }
            } else {
                self.imgsrc = ImageSource::Index(line.parse::<usize>().unwrap() as u32);
            }
        }
    }
}

impl ResourceItem for Sound {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "sound" + &format!("{item_index:02}")
    }

    fn is_modified(&self) -> bool {
        !self.notes.is_empty()
            || !self.tones.is_empty()
            || !self.volumes.is_empty()
            || !self.effects.is_empty()
    }

    fn clear(&mut self) {
        self.notes.clear();
        self.tones.clear();
        self.volumes.clear();
        self.effects.clear();
        self.speed = INITIAL_SPEED;
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        if self.notes.is_empty() {
            output += "none\n";
        } else {
            for note in &self.notes {
                if *note < 0 {
                    output += "ff";
                } else {
                    let _guard = write!(output, "{:02x}", *note);
                }
            }
            output += "\n";
        }

        macro_rules! stringify_data {
            ($name: ident) => {
                if self.$name.is_empty() {
                    output += "none\n";
                } else {
                    for value in &self.$name {
                        let _guard = write!(output, "{:1x}", *value);
                    }
                    output += "\n";
                }
            };
        }

        stringify_data!(tones);
        stringify_data!(volumes);
        stringify_data!(effects);
        let _guard = write!(output, "{}", self.speed);
        output
    }

    fn deserialize(&mut self, _version: u32, input: &str) {
        self.clear();
        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }
            if i == 0 {
                string_loop!(j, value, line, 2, {
                    self.notes.push(parse_hex_string(&value).unwrap() as i8);
                });
                continue;
            } else if i == 4 {
                self.speed = line.parse().unwrap();
                continue;
            }
            let data = match i {
                1 => &mut self.tones,
                2 => &mut self.volumes,
                3 => &mut self.effects,
                _ => panic!(),
            };
            string_loop!(j, value, line, 1, {
                data.push(parse_hex_string(&value).unwrap() as u8);
            });
        }
    }
}

impl ResourceItem for Music {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_index.to_string()
    }

    fn is_modified(&self) -> bool {
        self.seqs.iter().any(|seq| !seq.lock().is_empty())
    }

    fn clear(&mut self) {
        self.seqs = (0..NUM_CHANNELS)
            .map(|_| new_shared_type!(Vec::new()))
            .collect();
    }

    fn serialize(&self) -> String {
        let mut output = String::new();
        for seq in &self.seqs {
            if seq.lock().is_empty() {
                output += "none";
            } else {
                for sound_index in &*seq.lock() {
                    let _guard = write!(output, "{sound_index:02x}");
                }
            }
            output += "\n";
        }
        output
    }

    fn deserialize(&mut self, _version: u32, input: &str) {
        self.clear();
        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }
            string_loop!(j, value, line, 2, {
                self.seqs[i].lock().push(parse_hex_string(&value).unwrap());
            });
        }
    }
}

impl Pyxel {
    pub fn load_old_resource(
        &mut self,
        archive: &mut ZipArchive<File>,
        filename: &str,
        image: bool,
        tilemap: bool,
        sound: bool,
        music: bool,
    ) {
        let version_name = RESOURCE_ARCHIVE_DIRNAME.to_string() + "version";
        let contents = {
            let mut file = archive.by_name(&version_name).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        };
        let version = parse_version_string(&contents).unwrap();
        assert!(
            version <= parse_version_string(VERSION).unwrap(),
            "Unsupported resource file version '{contents}'"
        );

        macro_rules! deserialize {
            ($type: ty, $list: ident, $count: expr) => {
                for i in 0..$count {
                    if let Ok(mut file) = archive.by_name(&<$type>::resource_name(i)) {
                        let mut input = String::new();
                        file.read_to_string(&mut input).unwrap();
                        self.$list.lock()[i as usize]
                            .lock()
                            .deserialize(version, &input);
                    } else {
                        self.$list.lock()[i as usize].lock().clear();
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
            let colors: Vec<Rgb24> = contents
                .replace("\r\n", "\n")
                .replace('\r', "\n")
                .split('\n')
                .filter(|s| !s.is_empty())
                .map(|s| u32::from_str_radix(s.trim(), 16).unwrap() as Rgb24)
                .collect();
            self.colors.lock().clear();
            self.colors.lock().extend(colors.iter());
        }
    }
}
