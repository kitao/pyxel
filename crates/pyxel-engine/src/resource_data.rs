use serde::{Deserialize, Serialize, Serializer};

use crate::channel::{Channel, Note, Speed, Volume};
use crate::image::{Image, SharedImage};
use crate::music::{Music, SharedMusic};
use crate::oscillator::{Effect, Gain, Tone};
use crate::pyxel::Pyxel;
use crate::settings::RESOURCE_FORMAT_VERSION;
use crate::sound::{SharedSound, Sound};
use crate::tilemap::{ImageSource, SharedTilemap, Tilemap};
use crate::waveform::{Noise, SharedWaveform, Waveform, WaveformTable};
use crate::{Rgb24, SharedChannel};

#[derive(Clone, Serialize, Deserialize)]
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<Vec<u8>>,
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
        Self {
            width,
            height,
            data,
        }
    }

    fn to_image(&self) -> SharedImage {
        let image = Image::new(self.width, self.height);
        {
            let mut image = image.lock();
            image.canvas.data = self.data.clone().into_iter().flatten().collect();
        }
        image
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct TilemapData {
    width: u32,
    height: u32,
    imgsrc: u32,
    data: Vec<Vec<u8>>,
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
            .map(<[u8]>::to_vec)
            .collect();
        Self {
            width,
            height,
            imgsrc,
            data,
        }
    }

    fn to_tilemap(&self) -> SharedTilemap {
        let tilemap = Tilemap::new(self.width, self.height, ImageSource::Index(self.imgsrc));
        {
            let mut tilemap = tilemap.lock();
            let data: Vec<_> = self.data.clone().into_iter().flatten().collect();
            tilemap.canvas.data = data.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
        }
        tilemap
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct WaveformData {
    gain: Gain,
    noise: u32,
    table: WaveformTable,
}

impl WaveformData {
    fn from_waveform(waveform: SharedWaveform) -> Self {
        let waveform = waveform.lock();
        Self {
            gain: waveform.gain,
            noise: waveform.noise.to_index(),
            table: waveform.table,
        }
    }

    fn to_waveform(&self) -> SharedWaveform {
        let waveform = Waveform::new();
        {
            let mut waveform = waveform.lock();
            waveform.gain = self.gain;
            waveform.noise = Noise::from_index(self.noise);
            waveform.table = self.table;
        }
        waveform
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ChannelData {
    gain: Gain,
}

impl ChannelData {
    fn from_channel(channel: SharedChannel) -> Self {
        Self {
            gain: channel.lock().gain,
        }
    }

    fn to_channel(&self) -> SharedChannel {
        let channel = Channel::new();
        channel.lock().gain = self.gain;
        channel
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SoundData {
    notes: Vec<Note>,
    tones: Vec<Tone>,
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
            sound.notes = self.notes.clone();
            sound.tones = self.tones.clone();
            sound.volumes = self.volumes.clone();
            sound.effects = self.effects.clone();
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
        let seqs = music.seqs.iter().map(|seq| seq.lock().clone()).collect();
        Self { seqs }
    }

    fn to_music(&self) -> SharedMusic {
        let music = Music::new();
        {
            let mut music = music.lock();
            music.seqs = self
                .seqs
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
    #[serde(serialize_with = "serialize_as_hex")]
    colors: Vec<Rgb24>,
    images: Vec<ImageData>,
    tilemaps: Vec<TilemapData>,
    channels: Vec<ChannelData>,
    sounds: Vec<SoundData>,
    musics: Vec<MusicData>,
    waveforms: Vec<WaveformData>,
}

fn serialize_as_hex<S>(values: &[u32], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex_values = values
        .iter()
        .map(|&v| format!("{v:6X}"))
        .collect::<Vec<_>>();
    serializer.collect_seq(hex_values)
}

impl ResourceData {
    pub fn from_toml(toml_text: &str) -> Self {
        toml::from_str(toml_text).unwrap()
    }

    pub fn from_runtime(pyxel: &Pyxel) -> Self {
        let mut resource_data = ResourceData {
            format_version: RESOURCE_FORMAT_VERSION,
            colors: pyxel.colors.lock().clone(),
            images: Vec::new(),
            tilemaps: Vec::new(),
            channels: Vec::new(),
            sounds: Vec::new(),
            musics: Vec::new(),
            waveforms: Vec::new(),
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
        for channel in &*pyxel.channels.lock() {
            resource_data
                .channels
                .push(ChannelData::from_channel(channel.clone()));
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
        for waveform in &*pyxel.waveforms.lock() {
            resource_data
                .waveforms
                .push(WaveformData::from_waveform(waveform.clone()));
        }
        resource_data
    }

    pub fn to_runtime(
        &self,
        pyxel: &Pyxel,
        include_colors: bool,
        include_images: bool,
        include_tilemaps: bool,
        include_channels: bool,
        include_sounds: bool,
        include_musics: bool,
        include_waveforms: bool,
    ) {
        if !include_colors && !self.colors.is_empty() {
            *pyxel.colors.lock() = self.colors.clone();
        }
        if !include_images && !self.images.is_empty() {
            let mut images = Vec::new();
            for image_data in &self.images {
                images.push(image_data.to_image());
            }
            *pyxel.images.lock() = images;
        }
        if !include_tilemaps && !self.tilemaps.is_empty() {
            let mut tilemaps = Vec::new();
            for tilemap_data in &self.tilemaps {
                tilemaps.push(tilemap_data.to_tilemap());
            }
            *pyxel.tilemaps.lock() = tilemaps;
        }
        if !include_channels && !self.channels.is_empty() {
            let mut channels = Vec::new();
            for channel_data in &self.channels {
                channels.push(channel_data.to_channel());
            }
            *pyxel.channels.lock() = channels;
        }
        if !include_sounds && !self.sounds.is_empty() {
            let mut sounds = Vec::new();
            for sound_data in &self.sounds {
                sounds.push(sound_data.to_sound());
            }
            *pyxel.sounds.lock() = sounds;
        }
        if !include_musics && !self.musics.is_empty() {
            let mut musics = Vec::new();
            for music_data in &self.musics {
                musics.push(music_data.to_music());
            }
            *pyxel.musics.lock() = musics;
        }
        if !include_waveforms && !self.waveforms.is_empty() {
            let mut waveforms = Vec::new();
            for waveform_data in &self.waveforms {
                waveforms.push(waveform_data.to_waveform());
            }
            *pyxel.waveforms.lock() = waveforms;
        }
    }

    pub fn to_toml(
        &self,
        include_colors: bool,
        include_images: bool,
        include_tilemaps: bool,
        include_channels: bool,
        include_sounds: bool,
        include_musics: bool,
        include_waveforms: bool,
    ) -> String {
        let mut resource_data = (*self).clone();
        if !include_colors {
            resource_data.colors.clear();
        }
        if !include_images {
            resource_data.images.clear();
        }
        if !include_tilemaps {
            resource_data.tilemaps.clear();
        }
        if !include_channels {
            resource_data.channels.clear();
        }
        if !include_sounds {
            resource_data.sounds.clear();
        }
        if !include_musics {
            resource_data.musics.clear();
        }
        if !include_waveforms {
            resource_data.waveforms.clear();
        }
        toml::to_string(&resource_data).unwrap()
    }
}
