use std::sync::atomic::{AtomicBool, Ordering};

use crate::audio::Audio;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::math::Math;
use crate::prelude::*;
use crate::resource::Resource;
use crate::system::System;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Pyxel {
    // System
    pub(crate) system: System,
    pub width: u32,
    pub height: u32,
    pub frame_count: u32,
    pub(crate) fps: u32,

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
    pub colors: Vec<Rgb8>,
    pub screen: SharedImage,
    pub cursor: SharedImage,
    pub font: SharedImage,
    pub images: Vec<SharedImage>,
    pub tilemaps: Vec<SharedTilemap>,

    // Audio
    pub(crate) audio: Audio,
    pub channels: Vec<SharedChannel>,
    pub sounds: Vec<SharedSound>,
    pub musics: Vec<SharedMusic>,

    // Math
    pub(crate) math: Math,
}

impl Pyxel {
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        fps: Option<u32>,
        quit_key: Option<Key>,
        display_scale: Option<u32>,
        capture_scale: Option<u32>,
        capture_sec: Option<u32>,
    ) -> Self {
        if IS_INITIALIZED.swap(true, Ordering::Relaxed) {
            panic!("Pyxel already initialized");
        }

        // System
        let title = title.unwrap_or(DEFAULT_TITLE);
        let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
        let fps = fps.unwrap_or(DEFAULT_FPS);
        let system = System::new(width, height, title, fps, quit_key, display_scale);
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
        let colors = (0..NUM_COLORS)
            .map(|i| DEFAULT_COLORS[i as usize])
            .collect();
        let screen = Image::new(width, height);
        let cursor = CURSOR_IMAGE.clone();
        let font = FONT_IMAGE.clone();
        let images: Vec<_> = (0..NUM_IMAGES)
            .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
            .collect();
        let tilemaps: Vec<_> = (0..NUM_TILEMAPS)
            .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, images[0].clone()))
            .collect();

        // Audio
        let audio = Audio::new();
        let channels: Vec<_> = (0..NUM_CHANNELS).map(|_| Channel::new()).collect();
        let sounds: Vec<_> = (0..NUM_SOUNDS).map(|_| Sound::new()).collect();
        let musics: Vec<_> = (0..NUM_MUSICS).map(|_| Music::new()).collect();

        // Math
        let math = Math::new();

        let pyxel = Self {
            system,
            width,
            height,
            frame_count,
            fps,
            resource,
            input,
            mouse_x,
            mouse_y,
            mouse_wheel,
            input_text,
            dropped_files,
            graphics,
            colors,
            screen,
            cursor,
            font,
            images,
            tilemaps,
            audio,
            channels,
            sounds,
            musics,
            math,
        };
        pyxel.icon(&ICON_DATA, ICON_SCALE);
        pyxel
    }
}
