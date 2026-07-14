// ResourceData owns only plain resource values, so Serde derivation is intentional here.
#![allow(clippy::unsafe_derive_deserialize)]

use serde::{Deserialize, Serialize};

use crate::image::{Color, Image, RcImage};
use crate::music::{Music, RcMusic};
use crate::pyxel::{self, Pyxel};
use crate::sound::{RcSound, Sound, SoundEffect, SoundNote, SoundSpeed, SoundTone, SoundVolume};
use crate::tilemap::{ImageSource, ImageTileCoord, RcTilemap, Tilemap};
use crate::utils::{compress_vec2, expand_vec2, trim_empty_vec};

#[derive(Clone, Serialize, Deserialize)]
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<Vec<Color>>,
}

impl ImageData {
    fn from_image(image: &RcImage) -> Self {
        let image = rc_ref!(image);
        let width = image.width();
        let height = image.height();

        let data: Vec<Vec<_>> = image
            .canvas
            .data
            .chunks(width as usize)
            .map(<[Color]>::to_vec)
            .collect();
        let data = compress_vec2(&data);

        Self {
            width,
            height,
            data,
        }
    }

    fn to_image(&self) -> RcImage {
        let data = expand_vec2(&self.data, self.height as usize, self.width as usize);
        let image = Image::new(self.width, self.height);
        rc_mut!(image).canvas.data = data.into_iter().flatten().collect();
        image
    }

    fn validate(&self, index: usize) -> Result<(), String> {
        validate_grid("images", index, self.width, self.height, &self.data, 1)
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TilemapData {
    width: u32,
    height: u32,
    imgsrc: u32,
    data: Vec<Vec<ImageTileCoord>>,
}

impl TilemapData {
    fn from_tilemap(tilemap: &RcTilemap) -> Self {
        let tilemap = rc_ref!(tilemap);
        let width = tilemap.width();
        let height = tilemap.height();
        let imgsrc = match &tilemap.imgsrc {
            ImageSource::Index(value) => *value,
            ImageSource::Image(_) => 0,
        };

        let data: Vec<Vec<_>> = tilemap
            .canvas
            .data
            .chunks(width as usize)
            .map(|row| row.iter().flat_map(|(tx, ty)| [*tx, *ty]).collect())
            .collect();
        let data = compress_vec2(&data);

        Self {
            width,
            height,
            imgsrc,
            data,
        }
    }

    fn to_tilemap(&self) -> RcTilemap {
        let data = expand_vec2(&self.data, self.height as usize, (self.width * 2) as usize);
        let tilemap = Tilemap::new(self.width, self.height, ImageSource::Index(self.imgsrc));
        let flat: Vec<_> = data.into_iter().flatten().collect();
        rc_mut!(tilemap).canvas.data = flat.chunks(2).map(|c| (c[0], c[1])).collect();
        tilemap
    }

    fn validate(&self, index: usize, image_count: usize) -> Result<(), String> {
        validate_grid("tilemaps", index, self.width, self.height, &self.data, 2)?;
        if self.imgsrc as usize >= image_count {
            return Err(format!(
                "Invalid resource data: tilemaps[{index}].imgsrc {} is out of range 0..{image_count}",
                self.imgsrc
            ));
        }
        Ok(())
    }
}

fn validate_grid<T>(
    bank: &str,
    index: usize,
    width: u32,
    height: u32,
    data: &[Vec<T>],
    values_per_cell: u32,
) -> Result<(), String> {
    if width == 0 || height == 0 {
        return Err(format!(
            "Invalid resource data: {bank}[{index}] dimensions must be greater than 0"
        ));
    }
    if width.checked_mul(height).is_none() {
        return Err(format!(
            "Invalid resource data: {bank}[{index}] dimensions are too large"
        ));
    }
    let row_width = width
        .checked_mul(values_per_cell)
        .ok_or_else(|| format!("Invalid resource data: {bank}[{index}] row width is too large"))?
        as usize;
    if data.is_empty() {
        return Err(format!(
            "Invalid resource data: {bank}[{index}].data must not be empty"
        ));
    }
    if data.len() > height as usize {
        return Err(format!(
            "Invalid resource data: {bank}[{index}].data has {} rows, maximum is {height}",
            data.len()
        ));
    }
    for (row_index, row) in data.iter().enumerate() {
        if row.is_empty() {
            return Err(format!(
                "Invalid resource data: {bank}[{index}].data[{row_index}] must not be empty"
            ));
        }
        if row.len() > row_width {
            return Err(format!(
                "Invalid resource data: {bank}[{index}].data[{row_index}] has {} values, maximum is {row_width}",
                row.len()
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Serialize, Deserialize)]
struct SoundData {
    notes: Vec<SoundNote>,
    tones: Vec<SoundTone>,
    volumes: Vec<SoundVolume>,
    effects: Vec<SoundEffect>,
    speed: SoundSpeed,
}

impl SoundData {
    fn validate(&self, index: usize) -> Result<(), String> {
        Sound::validate_speed(self.speed).map_err(|_| {
            format!("Invalid resource data: sounds[{index}].speed must be greater than 0")
        })
    }

    fn from_sound(sound: &RcSound) -> Self {
        let sound = audio_ref!(sound);
        Self {
            notes: sound.notes.clone(),
            tones: sound.tones.clone(),
            volumes: sound.volumes.clone(),
            effects: sound.effects.clone(),
            speed: sound.speed,
        }
    }

    fn to_sound(&self) -> RcSound {
        let rc = Sound::new();
        let mut sound = audio_mut!(rc);
        sound.notes.clone_from(&self.notes);
        sound.tones.clone_from(&self.tones);
        sound.volumes.clone_from(&self.volumes);
        sound.effects.clone_from(&self.effects);
        sound.speed = self.speed;
        drop(sound);
        rc
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MusicData {
    seqs: Vec<Vec<u32>>,
}

impl MusicData {
    fn from_music(music: &RcMusic) -> Self {
        let music = audio_ref!(music);
        let seqs = trim_empty_vec(&music.seqs);

        Self { seqs }
    }

    fn to_music(&self) -> RcMusic {
        let rc = Music::new();
        audio_mut!(rc).seqs = trim_empty_vec(&self.seqs);
        rc
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResourceData {
    pub format_version: u32,
    images: Vec<ImageData>,
    tilemaps: Vec<TilemapData>,
    sounds: Vec<SoundData>,
    musics: Vec<MusicData>,
}

#[derive(Serialize)]
struct ResourceDataView<'a> {
    format_version: u32,
    images: &'a [ImageData],
    tilemaps: &'a [TilemapData],
    sounds: &'a [SoundData],
    musics: &'a [MusicData],
}

impl ResourceData {
    pub fn from_toml(toml_text: &str) -> Result<Self, String> {
        toml::from_str(toml_text).map_err(|_| "Failed to parse resource data".to_string())
    }

    pub fn from_runtime(_pyxel: &Pyxel) -> Self {
        Self {
            format_version: 1, // Write as the oldest format version for backward compatibility
            images: pyxel::images().iter().map(ImageData::from_image).collect(),
            tilemaps: pyxel::tilemaps()
                .iter()
                .map(TilemapData::from_tilemap)
                .collect(),
            sounds: pyxel::sounds().iter().map(SoundData::from_sound).collect(),
            musics: pyxel::musics().iter().map(MusicData::from_music).collect(),
        }
    }

    pub fn to_runtime(
        &self,
        _pyxel: &Pyxel,
        exclude_images: bool,
        exclude_tilemaps: bool,
        exclude_sounds: bool,
        exclude_musics: bool,
    ) -> Result<(), String> {
        if !exclude_images {
            for (index, image) in self.images.iter().enumerate() {
                image.validate(index)?;
            }
        }

        let image_count = if !exclude_images && !self.images.is_empty() {
            self.images.len()
        } else {
            pyxel::images().len()
        };
        if !exclude_tilemaps {
            for (index, tilemap) in self.tilemaps.iter().enumerate() {
                tilemap.validate(index, image_count)?;
            }
        }
        if !exclude_sounds {
            for (index, sound) in self.sounds.iter().enumerate() {
                sound.validate(index)?;
            }
        }

        let images = (!exclude_images && !self.images.is_empty())
            .then(|| self.images.iter().map(ImageData::to_image).collect());
        let tilemaps = (!exclude_tilemaps && !self.tilemaps.is_empty())
            .then(|| self.tilemaps.iter().map(TilemapData::to_tilemap).collect());
        let sounds = (!exclude_sounds && !self.sounds.is_empty())
            .then(|| self.sounds.iter().map(SoundData::to_sound).collect());
        let musics = (!exclude_musics && !self.musics.is_empty())
            .then(|| self.musics.iter().map(MusicData::to_music).collect());

        macro_rules! restore {
            ($data:expr, $accessor:expr) => {
                if let Some(data) = $data {
                    *$accessor() = data;
                }
            };
        }
        restore!(images, pyxel::images);
        restore!(tilemaps, pyxel::tilemaps);
        restore!(sounds, pyxel::sounds);
        restore!(musics, pyxel::musics);
        Ok(())
    }

    pub fn to_toml(
        &self,
        exclude_images: bool,
        exclude_tilemaps: bool,
        exclude_sounds: bool,
        exclude_musics: bool,
    ) -> String {
        // Serialize excluded banks as empty arrays without cloning retained banks.
        let view = ResourceDataView {
            format_version: self.format_version,
            images: if exclude_images { &[] } else { &self.images },
            tilemaps: if exclude_tilemaps {
                &[]
            } else {
                &self.tilemaps
            },
            sounds: if exclude_sounds { &[] } else { &self.sounds },
            musics: if exclude_musics { &[] } else { &self.musics },
        };
        toml::to_string(&view).unwrap()
    }
}
