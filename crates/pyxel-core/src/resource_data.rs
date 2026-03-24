#![allow(clippy::unsafe_derive_deserialize)]

use serde::{Deserialize, Serialize};

use crate::image::{Color, Image};
use crate::music::Music;
use crate::pyxel::{self, Pyxel};
use crate::sound::{Sound, SoundEffect, SoundNote, SoundSpeed, SoundTone, SoundVolume};
use crate::tilemap::{ImageSource, ImageTileCoord, Tilemap};
use crate::utils::{compress_vec2, expand_vec2, trim_empty_vecs};

#[derive(Clone, Serialize, Deserialize)]
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<Vec<Color>>,
}

impl ImageData {
    fn from_image(image: *mut Image) -> Self {
        let image = unsafe { &*image };
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

    fn to_image(&self) -> *mut Image {
        let data = expand_vec2(&self.data, self.height as usize, self.width as usize);
        let image = Image::new(self.width, self.height);
        unsafe { &mut *image }.canvas.data = data.into_iter().flatten().collect();
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
    fn from_tilemap(tilemap: *mut Tilemap) -> Self {
        let tilemap = unsafe { &*tilemap };
        let width = tilemap.width();
        let height = tilemap.height();
        let imgsrc = match tilemap.imgsrc {
            ImageSource::Index(value) => value,
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

    fn to_tilemap(&self) -> *mut Tilemap {
        let data = expand_vec2(&self.data, self.height as usize, (self.width * 2) as usize);
        let tilemap = Tilemap::new(self.width, self.height, ImageSource::Index(self.imgsrc));
        let flat: Vec<_> = data.into_iter().flatten().collect();
        unsafe { &mut *tilemap }.canvas.data = flat.chunks(2).map(|c| (c[0], c[1])).collect();
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
    fn from_sound(sound: *mut Sound) -> Self {
        let sound = unsafe { &*sound };
        Self {
            notes: sound.notes.clone(),
            tones: sound.tones.clone(),
            volumes: sound.volumes.clone(),
            effects: sound.effects.clone(),
            speed: sound.speed,
        }
    }

    fn to_sound(&self) -> *mut Sound {
        let ptr = Sound::new();
        let sound = unsafe { &mut *ptr };
        sound.notes.clone_from(&self.notes);
        sound.tones.clone_from(&self.tones);
        sound.volumes.clone_from(&self.volumes);
        sound.effects.clone_from(&self.effects);
        sound.speed = self.speed;
        ptr
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MusicData {
    seqs: Vec<Vec<u32>>,
}

impl MusicData {
    fn from_music(music: *mut Music) -> Self {
        let music = unsafe { &*music };
        let seqs = trim_empty_vecs(&music.seqs);

        Self { seqs }
    }

    fn to_music(&self) -> *mut Music {
        let ptr = Music::new();
        unsafe { &mut *ptr }.seqs = trim_empty_vecs(&self.seqs);
        ptr
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
struct ResourceDataRef<'a> {
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
        ResourceData {
            format_version: 1, // compatible with version 1
            images: pyxel::images()
                .iter()
                .map(|&img| ImageData::from_image(img))
                .collect(),
            tilemaps: pyxel::tilemaps()
                .iter()
                .map(|&tm| TilemapData::from_tilemap(tm))
                .collect(),
            sounds: pyxel::sounds()
                .iter()
                .map(|&snd| SoundData::from_sound(snd))
                .collect(),
            musics: pyxel::musics()
                .iter()
                .map(|&mus| MusicData::from_music(mus))
                .collect(),
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
                    for &ptr in $accessor().iter() {
                        unsafe { drop(Box::from_raw(ptr)) };
                    }
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
        let empty_images = Vec::new();
        let empty_tilemaps = Vec::new();
        let empty_sounds = Vec::new();
        let empty_musics = Vec::new();
        let view = ResourceDataRef {
            format_version: self.format_version,
            images: if exclude_images {
                &empty_images
            } else {
                &self.images
            },
            tilemaps: if exclude_tilemaps {
                &empty_tilemaps
            } else {
                &self.tilemaps
            },
            sounds: if exclude_sounds {
                &empty_sounds
            } else {
                &self.sounds
            },
            musics: if exclude_musics {
                &empty_musics
            } else {
                &self.musics
            },
        };
        toml::to_string(&view).unwrap()
    }
}
