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

        {
            let image = unsafe { &mut *image };
            image.canvas.data = data.into_iter().flatten().collect();
        }

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

        let data: Vec<_> = tilemap
            .canvas
            .data
            .iter()
            .flat_map(|(tx, ty)| [*tx, *ty].to_vec())
            .collect();
        let data: Vec<Vec<_>> = data
            .chunks((width * 2) as usize)
            .map(<[ImageTileCoord]>::to_vec)
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

        {
            let tilemap = unsafe { &mut *tilemap };
            let data: Vec<_> = data.clone().into_iter().flatten().collect();
            tilemap.canvas.data = data.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        }

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
        let sound = Sound::new();

        {
            let sound = unsafe { &mut *sound };
            sound.notes.clone_from(&self.notes);
            sound.tones.clone_from(&self.tones);
            sound.volumes.clone_from(&self.volumes);
            sound.effects.clone_from(&self.effects);
            sound.speed = self.speed;
        }

        sound
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
        let seqs = trim_empty_vecs(&self.seqs);
        let music = Music::new();

        {
            let music = unsafe { &mut *music };
            music.seqs = seqs;
        }

        music
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

impl ResourceData {
    pub fn from_toml(toml_text: &str) -> Self {
        toml::from_str(toml_text).unwrap()
    }

    pub fn from_runtime(_pyxel: &Pyxel) -> Self {
        let mut resource_data = ResourceData {
            format_version: 1, // comatible with version 1
            images: Vec::new(),
            tilemaps: Vec::new(),
            sounds: Vec::new(),
            musics: Vec::new(),
        };

        for &image in pyxel::images().iter() {
            resource_data.images.push(ImageData::from_image(image));
        }

        for &tilemap in pyxel::tilemaps().iter() {
            resource_data
                .tilemaps
                .push(TilemapData::from_tilemap(tilemap));
        }

        for &sound in pyxel::sounds().iter() {
            resource_data.sounds.push(SoundData::from_sound(sound));
        }

        for &music in pyxel::musics().iter() {
            resource_data.musics.push(MusicData::from_music(music));
        }

        resource_data
    }

    pub fn to_runtime(
        &self,
        _pyxel: &Pyxel,
        exclude_images: bool,
        exclude_tilemaps: bool,
        exclude_sounds: bool,
        exclude_musics: bool,
    ) {
        if !exclude_images && !self.images.is_empty() {
            let mut images = Vec::new();
            for image_data in &self.images {
                images.push(image_data.to_image());
            }
            *pyxel::images() = images;
        }

        if !exclude_tilemaps && !self.tilemaps.is_empty() {
            let mut tilemaps = Vec::new();
            for tilemap_data in &self.tilemaps {
                tilemaps.push(tilemap_data.to_tilemap());
            }
            *pyxel::tilemaps() = tilemaps;
        }

        if !exclude_sounds && !self.sounds.is_empty() {
            let mut sounds = Vec::new();
            for sound_data in &self.sounds {
                sounds.push(sound_data.to_sound());
            }
            *pyxel::sounds() = sounds;
        }

        if !exclude_musics && !self.musics.is_empty() {
            let mut musics = Vec::new();
            for music_data in &self.musics {
                musics.push(music_data.to_music());
            }
            *pyxel::musics() = musics;
        }
    }

    pub fn to_toml(
        &self,
        exclude_images: bool,
        exclude_tilemaps: bool,
        exclude_sounds: bool,
        exclude_musics: bool,
    ) -> String {
        let mut resource_data = (*self).clone();

        if exclude_images {
            resource_data.images.clear();
        }

        if exclude_tilemaps {
            resource_data.tilemaps.clear();
        }

        if exclude_sounds {
            resource_data.sounds.clear();
        }

        if exclude_musics {
            resource_data.musics.clear();
        }

        toml::to_string(&resource_data).unwrap()
    }
}
