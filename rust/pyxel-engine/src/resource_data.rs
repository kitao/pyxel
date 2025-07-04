use serde::{Deserialize, Serialize};

use crate::image::{Color, Image, SharedImage};
use crate::music::{Music, SharedMusic};
use crate::pyxel::Pyxel;
use crate::sound::{Effect, Note, SharedSound, Sound, Speed, ToneIndex, Volume};
use crate::tilemap::{ImageSource, ImageTileCoord, SharedTilemap, Tilemap};
use crate::utils::{compress_vec2, expand_vec2, trim_empty_vecs};

#[derive(Clone, Serialize, Deserialize)]
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<Vec<Color>>,
}

impl ImageData {
    fn from_image(image: SharedImage) -> Self {
        let image = image.lock();
        let width = image.width();
        let height = image.height();

        let data: Vec<Vec<_>> = image
            .canvas
            .data
            .chunks(width as usize)
            .map(<[u8]>::to_vec)
            .collect();
        let data = compress_vec2(&data);

        Self {
            width,
            height,
            data,
        }
    }

    fn to_image(&self) -> SharedImage {
        let data = expand_vec2(&self.data, self.height as usize, self.width as usize);
        let image = Image::new(self.width, self.height);

        {
            let mut image = image.lock();
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
    fn from_tilemap(tilemap: SharedTilemap) -> Self {
        let tilemap = tilemap.lock();
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

    fn to_tilemap(&self) -> SharedTilemap {
        let data = expand_vec2(&self.data, self.height as usize, (self.width * 2) as usize);
        let tilemap = Tilemap::new(self.width, self.height, ImageSource::Index(self.imgsrc));

        {
            let mut tilemap = tilemap.lock();
            let data: Vec<_> = data.clone().into_iter().flatten().collect();
            tilemap.canvas.data = data.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        }

        tilemap
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SoundData {
    notes: Vec<Note>,
    tones: Vec<ToneIndex>,
    volumes: Vec<Volume>,
    effects: Vec<Effect>,
    speed: Speed,
}

impl SoundData {
    fn from_sound(sound: SharedSound) -> Self {
        let sound = sound.lock();
        Self {
            notes: sound.notes.clone(),
            tones: sound.tones.clone(),
            volumes: sound.volumes.clone(),
            effects: sound.effects.clone(),
            speed: sound.speed,
        }
    }

    fn to_sound(&self) -> SharedSound {
        let sound = Sound::new();

        {
            let mut sound = sound.lock();
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
    fn from_music(music: SharedMusic) -> Self {
        let music = music.lock();
        let seqs: Vec<_> = music.seqs.iter().map(|seq| seq.lock().clone()).collect();
        let seqs = trim_empty_vecs(&seqs);

        Self { seqs }
    }

    fn to_music(&self) -> SharedMusic {
        let seqs = trim_empty_vecs(&self.seqs);
        let music = Music::new();

        {
            let mut music = music.lock();
            music.seqs = seqs
                .iter()
                .map(|seq| new_shared_type!(seq.clone()))
                .collect();
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

    pub fn from_runtime(pyxel: &Pyxel) -> Self {
        let mut resource_data = ResourceData {
            format_version: 1, // comatible with version 1
            images: Vec::new(),
            tilemaps: Vec::new(),
            sounds: Vec::new(),
            musics: Vec::new(),
        };

        for image in &*pyxel.images.lock() {
            resource_data
                .images
                .push(ImageData::from_image(image.clone()));
        }

        for tilemap in &*pyxel.tilemaps.lock() {
            resource_data
                .tilemaps
                .push(TilemapData::from_tilemap(tilemap.clone()));
        }

        for sound in &*pyxel.sounds.lock() {
            resource_data
                .sounds
                .push(SoundData::from_sound(sound.clone()));
        }

        for music in &*pyxel.musics.lock() {
            resource_data
                .musics
                .push(MusicData::from_music(music.clone()));
        }

        resource_data
    }

    pub fn to_runtime(
        &self,
        pyxel: &Pyxel,
        skip_images: bool,
        skip_tilemaps: bool,
        skip_sounds: bool,
        skip_musics: bool,
    ) {
        if !skip_images && !self.images.is_empty() {
            let mut images = Vec::new();
            for image_data in &self.images {
                images.push(image_data.to_image());
            }
            *pyxel.images.lock() = images;
        }

        if !skip_tilemaps && !self.tilemaps.is_empty() {
            let mut tilemaps = Vec::new();
            for tilemap_data in &self.tilemaps {
                tilemaps.push(tilemap_data.to_tilemap());
            }
            *pyxel.tilemaps.lock() = tilemaps;
        }

        if !skip_sounds && !self.sounds.is_empty() {
            let mut sounds = Vec::new();
            for sound_data in &self.sounds {
                sounds.push(sound_data.to_sound());
            }
            *pyxel.sounds.lock() = sounds;
        }

        if !skip_musics && !self.musics.is_empty() {
            let mut musics = Vec::new();
            for music_data in &self.musics {
                musics.push(music_data.to_music());
            }
            *pyxel.musics.lock() = musics;
        }
    }

    pub fn to_toml(
        &self,
        skip_images: bool,
        skip_tilemaps: bool,
        skip_sounds: bool,
        skip_musics: bool,
    ) -> String {
        let mut resource_data = (*self).clone();

        if skip_images {
            resource_data.images.clear();
        }

        if skip_tilemaps {
            resource_data.tilemaps.clear();
        }

        if skip_sounds {
            resource_data.sounds.clear();
        }

        if skip_musics {
            resource_data.musics.clear();
        }

        toml::to_string(&resource_data).unwrap()
    }
}
