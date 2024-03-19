use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use zip::ZipArchive;

use crate::image::{Color, Image, Rgb24};
use crate::music::Music;
use crate::pyxel::Pyxel;
use crate::settings::{
    INITIAL_SOUND_SPEED, NUM_CHANNELS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS,
    PALETTE_FILE_EXTENSION, TILEMAP_SIZE, VERSION,
};
use crate::sound::Sound;
use crate::tilemap::{ImageSource, TileCoord, Tilemap};
use crate::utils::{parse_hex_string, simplify_string};

pub const RESOURCE_ARCHIVE_DIRNAME: &str = "pyxel_resource/";

trait ResourceItem {
    fn resource_name(item_index: u32) -> String;
    fn clear(&mut self);
    fn deserialize(&mut self, version: u32, input: &str);
}

impl ResourceItem for Image {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "image" + &item_index.to_string()
    }

    fn clear(&mut self) {
        self.cls(0);
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

    fn clear(&mut self) {
        self.cls((0, 0));
    }

    fn deserialize(&mut self, version: u32, input: &str) {
        for (y, line) in input.lines().enumerate() {
            if y < TILEMAP_SIZE as usize {
                if version < 10500 {
                    string_loop!(x, tile, line, 3, {
                        let tile = parse_hex_string(&tile).unwrap();
                        self.canvas.write_data(
                            x,
                            y,
                            ((tile % 32) as TileCoord, (tile / 32) as TileCoord),
                        );
                    });
                } else {
                    string_loop!(x, tile, line, 4, {
                        let tile_x = parse_hex_string(&tile[0..2]).unwrap();
                        let tile_y = parse_hex_string(&tile[2..4]).unwrap();
                        self.canvas
                            .write_data(x, y, (tile_x as TileCoord, tile_y as TileCoord));
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

    fn clear(&mut self) {
        self.notes.clear();
        self.tones.clear();
        self.volumes.clear();
        self.effects.clear();
        self.speed = INITIAL_SOUND_SPEED;
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
            } else if i == 1 {
                string_loop!(j, value, line, 1, {
                    self.tones.push(parse_hex_string(&value).unwrap() as u32);
                });
            } else if i == 2 {
                string_loop!(j, value, line, 1, {
                    self.volumes.push(parse_hex_string(&value).unwrap() as u8);
                });
            } else if i == 3 {
                string_loop!(j, value, line, 1, {
                    self.effects.push(parse_hex_string(&value).unwrap() as u8);
                });
            } else if i == 4 {
                self.speed = line.parse().unwrap();
                continue;
            }
        }
    }
}

impl ResourceItem for Music {
    fn resource_name(item_index: u32) -> String {
        RESOURCE_ARCHIVE_DIRNAME.to_string() + "music" + &item_index.to_string()
    }

    fn clear(&mut self) {
        self.seqs = (0..NUM_CHANNELS)
            .map(|_| new_shared_type!(Vec::new()))
            .collect();
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
        include_images: bool,
        include_tilemaps: bool,
        include_sounds: bool,
        include_musics: bool,
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

        if include_images {
            deserialize!(Image, images, NUM_IMAGES);
        }
        if include_tilemaps {
            deserialize!(Tilemap, tilemaps, NUM_TILEMAPS);
        }
        if include_sounds {
            deserialize!(Sound, sounds, NUM_SOUNDS);
        }
        if include_musics {
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

fn parse_version_string(string: &str) -> Result<u32, &str> {
    let mut version = 0;
    for (i, number) in simplify_string(string).split('.').enumerate() {
        let digit = number.len();
        let number = if i > 0 && digit == 1 {
            "0".to_string() + number
        } else if i == 0 || digit == 2 {
            number.to_string()
        } else {
            return Err("invalid version string");
        };
        if let Ok(number) = number.parse::<u32>() {
            version = version * 100 + number;
        } else {
            return Err("invalid version string");
        }
    }
    Ok(version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_version_string() {
        assert_eq!(parse_version_string("1.2.3"), Ok(10203));
        assert_eq!(parse_version_string("12.34.5"), Ok(123405));
        assert_eq!(parse_version_string("12.3.04"), Ok(120304));
        assert_eq!(
            parse_version_string("12.345.0"),
            Err("invalid version string")
        );
        assert_eq!(
            parse_version_string("12.0.345"),
            Err("invalid version string")
        );
        assert_eq!(parse_version_string(" "), Err("invalid version string"));
    }
}
