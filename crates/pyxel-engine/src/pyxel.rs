use std::cmp::max;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::audio::{Audio, CHANNELS};
use crate::channel::{Channel, SharedChannels};
use crate::graphics::{Graphics, SharedColors, COLORS, CURSOR_IMAGE, FONT_IMAGE};
use crate::image::{Image, SharedImage};
use crate::input::Input;
use crate::keys::Key;
use crate::math::Math;
use crate::music::{Music, SharedMusic};
use crate::resource::Resource;
use crate::settings::{
    DEFAULT_FPS, DEFAULT_QUIT_KEY, DEFAULT_TITLE, DISPLAY_RATIO, ICON_DATA, ICON_SCALE, IMAGE_SIZE,
    NUM_CHANNELS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, TILEMAP_SIZE,
};
use crate::sound::{SharedSound, Sound};
use crate::system::System;
use crate::tilemap::{SharedTilemap, Tilemap};
use crate::ICON_COLKEY;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Pyxel {
    // System
    pub(crate) system: System,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,

    // Resource
    pub(crate) resource: Resource,

    // Input
    pub(crate) input: Input,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub mouse_wheel: i32,
    pub input_text: String,
    pub dropped_files: Vec<String>,

    // Graphics
    pub(crate) graphics: Graphics,
    pub colors: SharedColors,
    pub images: Vec<SharedImage>,
    pub tilemaps: Vec<SharedTilemap>,
    pub screen: SharedImage,
    pub cursor: SharedImage,
    pub font: SharedImage,

    // Audio
    pub(crate) audio: Audio,
    pub channels: SharedChannels,
    pub sounds: Vec<SharedSound>,
    pub musics: Vec<SharedMusic>,

    // Math
    pub(crate) math: Math,
}

pub fn init(
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
) -> Pyxel {
    if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
        panic!("Pyxel already initialized");
    }

    // Default parameters
    let title = title.unwrap_or(DEFAULT_TITLE);
    let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
    let fps = fps.unwrap_or(DEFAULT_FPS);

    // Platform
    pyxel_platform::init(|display_width, display_height| {
        let display_scale = max(
            if let Some(display_scale) = display_scale {
                display_scale
            } else {
                (f64::min(
                    display_width as f64 / width as f64,
                    display_height as f64 / height as f64,
                ) * DISPLAY_RATIO) as u32
            },
            1,
        );
        (title, width * display_scale, height * display_scale)
    });

    // System
    let system = System::new(fps, quit_key);
    let frame_count = 0;

    // Resource
    let resource = Resource::new(capture_scale, capture_sec, fps);

    // Input
    let input = Input::new();
    let mouse_x = 0;
    let mouse_y = 0;
    let mouse_wheel = 0;
    let input_text = String::new();
    let dropped_files = Vec::new();

    // Graphics
    let graphics = Graphics::new();
    let colors = COLORS.clone();
    let images: Vec<_> = (0..NUM_IMAGES)
        .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
        .collect();
    let tilemaps: Vec<_> = (0..NUM_TILEMAPS)
        .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, images[0].clone()))
        .collect();
    let screen = Image::new(width, height);
    let cursor = CURSOR_IMAGE.clone();
    let font = FONT_IMAGE.clone();

    // Audio
    let audio = Audio::new();
    let channels = CHANNELS.clone();
    let sounds: Vec<_> = (0..NUM_SOUNDS).map(|_| Sound::new()).collect();
    let musics: Vec<_> = (0..NUM_MUSICS).map(|_| Music::new()).collect();

    // Math
    let math = Math::new();

    let pyxel = Pyxel {
        system,
        width,
        height,
        frame_count,
        resource,
        input,
        mouse_x,
        mouse_y,
        mouse_wheel,
        input_text,
        dropped_files,
        graphics,
        colors,
        images,
        tilemaps,
        screen,
        cursor,
        font,
        audio,
        channels,
        sounds,
        musics,
        math,
    };
    pyxel.icon(&ICON_DATA, ICON_SCALE, ICON_COLKEY);
    pyxel
}
