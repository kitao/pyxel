mod audio;
mod canvas;
mod channel;
mod event;
mod graphics;
mod image;
mod input;
mod key;
mod music;
mod oscillator;
mod platform;
mod profiler;
mod rectarea;
mod resource;
mod sdl2;
mod settings;
mod sound;
mod system;
mod tilemap;
mod types;
mod utility;

use crate::audio::{AtomicAudio, Audio};
use crate::graphics::Graphics;
use crate::image::{Image, SharedImage};
use crate::input::Input;
use crate::music::{Music, SharedMusic};
use crate::platform::Platform;
use crate::resource::Resource;
use crate::sdl2::Sdl2;
use crate::sound::{SharedSound, Sound};
use crate::system::System;
use crate::tilemap::{SharedTilemap, Tilemap};

pub use crate::key::*;
pub use crate::settings::*;
pub use crate::types::*;

pub type TargetPlatform = Sdl2;

pub struct Pyxel {
    platform: TargetPlatform,
    system: System,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: AtomicAudio,

    pub colors: [Rgb8; COLOR_COUNT as usize],
    pub images: Vec<SharedImage>,
    pub tilemaps: Vec<SharedTilemap>,
    pub screen: SharedImage,
    pub cursor: SharedImage,
    pub font: SharedImage,

    pub sounds: Vec<SharedSound>,
    pub musics: Vec<SharedMusic>,
}

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

impl Pyxel {
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        scale: Option<u32>,
        fps: Option<u32>,
        quit_key: Option<Key>,
    ) -> Pyxel {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let scale = scale.unwrap_or(DEFAULT_SCALE);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);

        let platform = TargetPlatform::new(title, width, height, scale);
        let system = System::new(fps, quit_key);
        let resource = Resource::new(width, height);
        let input = Input::new();
        let graphics = Graphics::new();
        let audio = Audio::new();

        let mut colors = [0; COLOR_COUNT as usize];
        for (i, rgb) in DEFAULT_COLOR.iter().enumerate() {
            colors[i] = *rgb;
        }
        let images = (0..IMAGE_COUNT)
            .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
            .collect();
        let tilemaps = (0..TILEMAP_COUNT)
            .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE))
            .collect();
        let screen = Image::new(width, height);
        let cursor = Graphics::new_cursor_image();
        let font = Graphics::new_font_image();

        let sounds = (0..SOUND_COUNT).map(|_| Sound::new()).collect();
        let musics = (0..MUSIC_COUNT).map(|_| Music::new()).collect();

        let mut pyxel = Pyxel {
            platform: platform,
            system: system,
            resource: resource,
            input: input,
            graphics: graphics,
            audio: audio.clone(),

            colors: colors,
            images: images,
            tilemaps: tilemaps,
            screen: screen,
            cursor: cursor,
            font: font,

            sounds: sounds,
            musics: musics,
        };

        pyxel.platform.start_audio(SAMPLE_RATE, SAMPLE_COUNT, audio);
        pyxel.system.reset_start_time(pyxel.platform.tick_count());

        pyxel
    }
}
