use std::cmp::Ordering;
use std::fs::File;
use std::io::Read;

use zip::result::ZipError;
use zip::ZipArchive;

use crate::image::{Color, Image};
use crate::music::Music;
use crate::pyxel::{self, Pyxel};
use crate::settings::{
    DEFAULT_SOUND_SPEED, NUM_CHANNELS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS,
    TILEMAP_SIZE, VERSION,
};
use crate::sound::{Sound, SoundEffect, SoundNote, SoundTone, SoundVolume};
use crate::tilemap::{ImageSource, ImageTileCoord, Tilemap};
use crate::utils::simplify_string;

pub const RESOURCE_ARCHIVE_DIRNAME: &str = "pyxel_resource/";

// Legacy archive item adapters

trait ResourceItem: Clone {
    fn resource_name(item_index: u32) -> String;
    fn clear(&mut self);
    fn deserialize(&mut self, version: u32, input: &str, entry: &str) -> Result<(), String>;
}

// Image data

impl ResourceItem for Image {
    fn resource_name(item_index: u32) -> String {
        format!("{RESOURCE_ARCHIVE_DIRNAME}image{item_index}")
    }

    fn clear(&mut self) {
        self.clear(0);
    }

    fn deserialize(&mut self, _version: u32, input: &str, entry: &str) -> Result<(), String> {
        for (y, line) in input.lines().enumerate() {
            if y >= self.height() as usize {
                return Err(format!(
                    "too many image rows in '{entry}': got {}, maximum {}",
                    input.lines().count(),
                    self.height()
                ));
            }
            let digits: Vec<char> = line.chars().collect();
            if digits.len() > self.width() as usize {
                return Err(format!(
                    "too many image columns in '{entry}' at line {}: got {}, maximum {}",
                    y + 1,
                    digits.len(),
                    self.width()
                ));
            }
            for (x, digit) in digits.into_iter().enumerate() {
                let color = parse_hex_digit(digit, entry, y + 1, x + 1)?;
                self.canvas.write_data(x, y, color as Color);
            }
        }
        Ok(())
    }
}

// Tilemap data

impl ResourceItem for Tilemap {
    fn resource_name(item_index: u32) -> String {
        format!("{RESOURCE_ARCHIVE_DIRNAME}tilemap{item_index}")
    }

    fn clear(&mut self) {
        self.clear((0, 0));
    }

    fn deserialize(&mut self, version: u32, input: &str, entry: &str) -> Result<(), String> {
        for (y, line) in input.lines().enumerate() {
            match y.cmp(&(TILEMAP_SIZE as usize)) {
                Ordering::Less => {
                    let group_width = if version < 10500 { 3 } else { 4 };
                    let digits: Vec<char> = line.chars().collect();
                    if !digits.len().is_multiple_of(group_width) {
                        return Err(format!(
                        "invalid tile width in '{entry}' at line {}: expected groups of {group_width} hexadecimal digits",
                        y + 1
                    ));
                    }
                    let tile_count = digits.len() / group_width;
                    if tile_count > self.width() as usize {
                        return Err(format!(
                            "too many tiles in '{entry}' at line {}: got {tile_count}, maximum {}",
                            y + 1,
                            self.width()
                        ));
                    }
                    for (x, group) in digits.chunks_exact(group_width).enumerate() {
                        let tile = parse_hex_group(group, entry, y + 1, x * group_width + 1)?;
                        let value = if version < 10500 {
                            ((tile % 32) as ImageTileCoord, (tile / 32) as ImageTileCoord)
                        } else {
                            (
                                ((tile >> 8) & 0xff) as ImageTileCoord,
                                (tile & 0xff) as ImageTileCoord,
                            )
                        };
                        self.canvas.write_data(x, y, value);
                    }
                }
                Ordering::Equal => {
                    let index = line.parse::<u32>().map_err(|_| {
                        format!(
                            "invalid decimal value '{line}' in '{entry}' at line {}",
                            y + 1
                        )
                    })?;
                    if index >= NUM_IMAGES {
                        return Err(format!(
                            "image index {index} in '{entry}' at line {} is out of range 0..{NUM_IMAGES}",
                            y + 1
                        ));
                    }
                    self.imgsrc = ImageSource::Index(index);
                }
                Ordering::Greater => {
                    return Err(format!(
                        "too many tilemap lines in '{entry}': got {}, maximum {}",
                        input.lines().count(),
                        TILEMAP_SIZE + 1
                    ));
                }
            }
        }
        Ok(())
    }
}

// Sound data

impl ResourceItem for Sound {
    fn resource_name(item_index: u32) -> String {
        format!("{RESOURCE_ARCHIVE_DIRNAME}sound{item_index:02}")
    }

    fn clear(&mut self) {
        self.notes.clear();
        self.tones.clear();
        self.volumes.clear();
        self.effects.clear();
        self.speed = DEFAULT_SOUND_SPEED;
    }

    fn deserialize(&mut self, _version: u32, input: &str, entry: &str) -> Result<(), String> {
        self.clear();

        for (i, line) in input.lines().enumerate() {
            if line == "none" {
                continue;
            }
            match i {
                0 => parse_hex_values(line, 2, entry, i + 1, |value| {
                    self.notes.push(value as i8 as SoundNote);
                })?,
                1 => parse_hex_values(line, 1, entry, i + 1, |value| {
                    self.tones.push(value as SoundTone);
                })?,
                2 => parse_hex_values(line, 1, entry, i + 1, |value| {
                    self.volumes.push(value as SoundVolume);
                })?,
                3 => parse_hex_values(line, 1, entry, i + 1, |value| {
                    self.effects.push(value as SoundEffect);
                })?,
                4 => {
                    self.speed = line.parse().map_err(|_| {
                        format!(
                            "invalid decimal value '{line}' in '{entry}' at line {}",
                            i + 1
                        )
                    })?;
                }
                _ => {}
            }
        }
        Ok(())
    }
}

// Music data

impl ResourceItem for Music {
    fn resource_name(item_index: u32) -> String {
        format!("{RESOURCE_ARCHIVE_DIRNAME}music{item_index}")
    }

    fn clear(&mut self) {
        self.seqs = vec![Vec::new(); NUM_CHANNELS as usize];
    }

    fn deserialize(&mut self, _version: u32, input: &str, entry: &str) -> Result<(), String> {
        self.clear();

        for (i, line) in input.lines().enumerate() {
            if i >= NUM_CHANNELS as usize {
                return Err(format!(
                    "too many music channels in '{entry}': got {}, maximum {NUM_CHANNELS}",
                    input.lines().count()
                ));
            }
            if line == "none" {
                continue;
            }
            parse_hex_values(line, 2, entry, i + 1, |value| {
                self.seqs[i].push(value);
            })?;
        }
        Ok(())
    }
}

impl Pyxel {
    pub fn load_old_resource(
        &mut self,
        archive: &mut ZipArchive<File>,
        include_images: bool,
        include_tilemaps: bool,
        include_sounds: bool,
        include_musics: bool,
    ) -> Result<(), String> {
        // Read and validate archive version
        let version_name = format!("{RESOURCE_ARCHIVE_DIRNAME}version");
        let contents = {
            let mut file = archive
                .by_name(&version_name)
                .map_err(|_| format!("failed to open '{version_name}'"))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|_| format!("failed to read '{version_name}' as UTF-8"))?;
            contents
        };
        let version =
            parse_version_string(&contents).map_err(|_| format!("invalid version '{contents}'"))?;
        let current_version =
            parse_version_string(VERSION).expect("Pyxel version constant must be valid");
        if version > current_version {
            return Err(format!("unsupported version '{contents}'"));
        }

        // Deserialize selected resource banks
        macro_rules! stage {
            ($read: ident, $type: ty, $accessor: expr, $count: expr, $limit: expr) => {
                if $accessor {
                    let items = $count;
                    let limit = ($limit as usize).min(items.len());
                    let mut staged = Vec::with_capacity(limit);
                    for (i, current) in items.iter().take(limit).enumerate() {
                        let mut item = $read!(current).clone();
                        let entry = <$type>::resource_name(i as u32);
                        match archive.by_name(&entry) {
                            Ok(mut file) => {
                                let mut input = String::new();
                                file.read_to_string(&mut input)
                                    .map_err(|_| format!("failed to read '{entry}' as UTF-8"))?;
                                item.deserialize(version, &input, &entry)?;
                            }
                            Err(ZipError::FileNotFound) => ResourceItem::clear(&mut item),
                            Err(_) => {
                                return Err(format!("failed to open '{entry}'"));
                            }
                        }
                        staged.push(item);
                    }
                    Some(staged)
                } else {
                    None
                }
            };
        }
        macro_rules! commit {
            ($write: ident, $staged: expr, $items: expr) => {
                if let Some(staged) = $staged {
                    for (target, item) in $items.iter().zip(staged) {
                        *$write!(target) = item;
                    }
                }
            };
        }

        let images = stage!(rc_ref, Image, include_images, pyxel::images(), NUM_IMAGES);
        let tilemaps = stage!(
            rc_ref,
            Tilemap,
            include_tilemaps,
            pyxel::tilemaps(),
            NUM_TILEMAPS
        );
        let sounds = stage!(
            audio_ref,
            Sound,
            include_sounds,
            pyxel::sounds(),
            NUM_SOUNDS
        );
        let musics = stage!(
            audio_ref,
            Music,
            include_musics,
            pyxel::musics(),
            NUM_MUSICS
        );

        commit!(rc_mut, images, pyxel::images());
        commit!(rc_mut, tilemaps, pyxel::tilemaps());
        commit!(audio_mut, sounds, pyxel::sounds());
        commit!(audio_mut, musics, pyxel::musics());
        Ok(())
    }
}

// Helpers

fn parse_version_string(string: &str) -> Result<u32, &str> {
    let mut version = 0u32;
    for (i, part) in simplify_string(string).split('.').enumerate() {
        let len = part.len();
        if i > 0 && len != 1 && len != 2 {
            return Err("invalid version string");
        }
        let n: u32 = part.parse().map_err(|_| "invalid version string")?;
        version = version
            .checked_mul(100)
            .and_then(|version| version.checked_add(n))
            .ok_or("invalid version string")?;
    }
    Ok(version)
}

fn parse_hex_digit(digit: char, entry: &str, line: usize, column: usize) -> Result<u32, String> {
    digit.to_digit(16).ok_or_else(|| {
        format!("invalid hexadecimal digit '{digit}' in '{entry}' at line {line}, column {column}")
    })
}

fn parse_hex_group(
    digits: &[char],
    entry: &str,
    line: usize,
    start_column: usize,
) -> Result<u32, String> {
    digits.iter().enumerate().try_fold(0, |value, (i, &digit)| {
        Ok((value << 4) | parse_hex_digit(digit, entry, line, start_column + i)?)
    })
}

fn parse_hex_values(
    line_text: &str,
    group_width: usize,
    entry: &str,
    line: usize,
    mut push: impl FnMut(u32),
) -> Result<(), String> {
    let digits: Vec<char> = line_text.chars().collect();
    if !digits.len().is_multiple_of(group_width) {
        return Err(format!(
            "invalid value width in '{entry}' at line {line}: expected groups of {group_width} hexadecimal digits"
        ));
    }
    for (group_index, group) in digits.chunks_exact(group_width).enumerate() {
        push(parse_hex_group(
            group,
            entry,
            line,
            group_index * group_width + 1,
        )?);
    }
    Ok(())
}
