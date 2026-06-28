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
    fn from_sound(sound: &RcSound) -> Self {
        let sound = rc_ref!(sound);
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
        let sound = rc_mut!(rc);
        sound.notes.clone_from(&self.notes);
        sound.tones.clone_from(&self.tones);
        sound.volumes.clone_from(&self.volumes);
        sound.effects.clone_from(&self.effects);
        sound.speed = self.speed;
        rc
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MusicData {
    seqs: Vec<Vec<u32>>,
}

impl MusicData {
    fn from_music(music: &RcMusic) -> Self {
        let music = rc_ref!(music);
        let seqs = trim_empty_vec(&music.seqs);

        Self { seqs }
    }

    fn to_music(&self) -> RcMusic {
        let rc = Music::new();
        rc_mut!(rc).seqs = trim_empty_vec(&self.seqs);
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
    ) {
        macro_rules! restore {
            ($exclude:expr, $data:expr, $accessor:expr, $converter:path) => {
                if !$exclude && !$data.is_empty() {
                    *$accessor() = $data.iter().map($converter).collect();
                }
            };
        }
        restore!(
            exclude_images,
            self.images,
            pyxel::images,
            ImageData::to_image
        );
        restore!(
            exclude_tilemaps,
            self.tilemaps,
            pyxel::tilemaps,
            TilemapData::to_tilemap
        );
        restore!(
            exclude_sounds,
            self.sounds,
            pyxel::sounds,
            SoundData::to_sound
        );
        restore!(
            exclude_musics,
            self.musics,
            pyxel::musics,
            MusicData::to_music
        );
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
