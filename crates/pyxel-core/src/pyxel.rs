use std::array;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::audio::Audio;
use crate::canvas::Canvas;
use crate::channel::Channel;
use crate::graphics::Graphics;
use crate::image::{Color, Image, Rgb24};
use crate::input::Input;
use crate::key::Key;
use crate::music::Music;
use crate::platform;
use crate::resource::Resource;
use crate::settings::{
    CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, DEFAULT_COLORS, DEFAULT_FPS, DEFAULT_QUIT_KEY,
    DEFAULT_TITLE, DEFAULT_TONE_0, DEFAULT_TONE_1, DEFAULT_TONE_2, DEFAULT_TONE_3, DISPLAY_RATIO,
    FONT_DATA, FONT_HEIGHT, FONT_WIDTH, ICON_COLKEY, ICON_DATA, ICON_SCALE, IMAGE_SIZE,
    NUM_CHANNELS, NUM_FONT_ROWS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS, NUM_TONES,
    TILEMAP_SIZE,
};
use crate::sound::Sound;
use crate::system::System;
use crate::tilemap::{ImageSource, Tilemap};
use crate::tone::Tone;

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

// Singleton
static mut PYXEL: *mut Pyxel = null_mut();

pub fn pyxel() -> &'static mut Pyxel {
    unsafe {
        assert!(!PYXEL.is_null(), "Pyxel not initialized");
        &mut *PYXEL
    }
}

fn set_pyxel_instance(instance: Pyxel) {
    unsafe {
        PYXEL = Box::into_raw(Box::new(instance));
    }
}

// RESET_FUNC
static mut RESET_FUNC: Option<Box<dyn FnMut() + Send>> = None;

pub fn reset_func() -> &'static mut Option<Box<dyn FnMut() + Send>> {
    unsafe { &mut RESET_FUNC }
}

// Macros for global variables
macro_rules! define_static {
    ($func:ident, $static:ident, $type:ty, $default:expr) => {
        static mut $static: $type = $default;
        pub fn $func() -> &'static mut $type {
            unsafe { &mut $static }
        }
    };
}

macro_rules! define_global {
    ($func:ident, $static:ident, $type:ty, $init:expr) => {
        static mut $static: *mut $type = null_mut();
        pub fn $func() -> &'static mut $type {
            unsafe {
                if $static.is_null() {
                    $static = Box::into_raw(Box::new($init));
                }
                &mut *$static
            }
        }
    };
}

// System
define_static!(width, WIDTH, u32, 0);
define_static!(height, HEIGHT, u32, 0);
define_static!(frame_count, FRAME_COUNT, u32, 0);

// Input
define_static!(mouse_x, MOUSE_X, i32, 0);
define_static!(mouse_y, MOUSE_Y, i32, 0);
define_static!(mouse_wheel, MOUSE_WHEEL, i32, 0);
define_static!(input_keys, INPUT_KEYS, Vec<Key>, Vec::new());
define_static!(input_text, INPUT_TEXT, String, String::new());
define_static!(dropped_files, DROPPED_FILES, Vec<String>, Vec::new());

// Graphics
define_global!(colors, COLORS, Vec<Rgb24>, DEFAULT_COLORS.to_vec());
define_global!(images, IMAGES, Vec<*mut Image>, init_images());
define_global!(tilemaps, TILEMAPS, Vec<*mut Tilemap>, init_tilemaps());
define_global!(screen, SCREEN, Image, init_screen());
define_global!(cursor_image, CURSOR_IMAGE, Image, init_cursor_image());
define_global!(font_image, FONT_IMAGE, Image, init_font_image());

// Audio
define_global!(channels, CHANNELS, Vec<*mut Channel>, init_channels());
define_global!(tones, TONES, Vec<*mut Tone>, init_tones());
define_global!(sounds, SOUNDS, Vec<*mut Sound>, init_sounds());
define_global!(musics, MUSICS, Vec<*mut Music>, init_musics());

pub struct Pyxel {
    pub(crate) system: System,
    pub(crate) resource: Resource,
    pub(crate) input: Input,
    pub(crate) graphics: Graphics,
}

pub fn init(
    w: u32,
    h: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
) {
    assert!(
        !IS_INITIALIZED.swap(true, Ordering::Relaxed),
        "Pyxel already initialized"
    );

    // Set dimensions
    *width() = w;
    *height() = h;
    *frame_count() = 0;

    // Default parameters
    let title = title.unwrap_or(DEFAULT_TITLE);
    let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
    let fps = fps.unwrap_or(DEFAULT_FPS);

    // Platform
    platform::init();

    let (display_width, display_height) = platform::display_size();
    let display_scale = display_scale
        .unwrap_or(
            (f32::min(
                display_width as f32 / w as f32,
                display_height as f32 / h as f32,
            ) * DISPLAY_RATIO) as u32,
        )
        .max(1);
    let window_width = w * display_scale;
    let window_height = h * display_scale;

    platform::init_window(title, window_width, window_height);

    // Resize screen
    screen().canvas = Canvas::new(w, h);
    screen().palette = array::from_fn(|i| i as Color);

    // Reset input
    *mouse_x() = 0;
    *mouse_y() = 0;
    *mouse_wheel() = 0;
    input_keys().clear();
    input_text().clear();
    dropped_files().clear();

    // Build Pyxel instance
    let system = System::new(fps, quit_key);
    let resource = Resource::new(capture_scale, capture_sec, fps);
    let input = Input::new();
    let graphics = Graphics::new();

    set_pyxel_instance(Pyxel {
        system,
        resource,
        input,
        graphics,
    });

    // Audio
    Audio::start();

    // Icon
    pyxel().icon(&ICON_DATA, ICON_SCALE, ICON_COLKEY);
}

pub fn reset_statics() {
    IS_INITIALIZED.store(false, Ordering::Relaxed);

    // Reset scalar statics
    *width() = 0;
    *height() = 0;
    *frame_count() = 0;
    *mouse_x() = 0;
    *mouse_y() = 0;
    *mouse_wheel() = 0;
    input_keys().clear();
    input_text().clear();
    dropped_files().clear();

    // Reset heap globals
    unsafe {
        if !COLORS.is_null() {
            drop(Box::from_raw(COLORS));
            COLORS = null_mut();
        }
        if !IMAGES.is_null() {
            for &img in &*IMAGES {
                if !img.is_null() {
                    drop(Box::from_raw(img));
                }
            }
            drop(Box::from_raw(IMAGES));
            IMAGES = null_mut();
        }
        if !TILEMAPS.is_null() {
            for &tm in &*TILEMAPS {
                if !tm.is_null() {
                    drop(Box::from_raw(tm));
                }
            }
            drop(Box::from_raw(TILEMAPS));
            TILEMAPS = null_mut();
        }
        if !SCREEN.is_null() {
            drop(Box::from_raw(SCREEN));
            SCREEN = null_mut();
        }
        if !CURSOR_IMAGE.is_null() {
            drop(Box::from_raw(CURSOR_IMAGE));
            CURSOR_IMAGE = null_mut();
        }
        if !FONT_IMAGE.is_null() {
            drop(Box::from_raw(FONT_IMAGE));
            FONT_IMAGE = null_mut();
        }
        if !CHANNELS.is_null() {
            for &ch in &*CHANNELS {
                if !ch.is_null() {
                    drop(Box::from_raw(ch));
                }
            }
            drop(Box::from_raw(CHANNELS));
            CHANNELS = null_mut();
        }
        if !TONES.is_null() {
            for &t in &*TONES {
                if !t.is_null() {
                    drop(Box::from_raw(t));
                }
            }
            drop(Box::from_raw(TONES));
            TONES = null_mut();
        }
        if !SOUNDS.is_null() {
            for &s in &*SOUNDS {
                if !s.is_null() {
                    drop(Box::from_raw(s));
                }
            }
            drop(Box::from_raw(SOUNDS));
            SOUNDS = null_mut();
        }
        if !MUSICS.is_null() {
            for &m in &*MUSICS {
                if !m.is_null() {
                    drop(Box::from_raw(m));
                }
            }
            drop(Box::from_raw(MUSICS));
            MUSICS = null_mut();
        }
        RESET_FUNC = None;
    }
}

// Init functions for define_global!

fn init_screen() -> Image {
    Image {
        canvas: Canvas::new(0, 0),
        palette: array::from_fn(|i| i as Color),
    }
}

fn init_images() -> Vec<*mut Image> {
    (0..NUM_IMAGES)
        .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
        .collect()
}

fn init_tilemaps() -> Vec<*mut Tilemap> {
    (0..NUM_TILEMAPS)
        .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, ImageSource::Index(0)))
        .collect()
}

fn init_cursor_image() -> Image {
    let mut image = Image {
        canvas: Canvas::new(CURSOR_WIDTH, CURSOR_HEIGHT),
        palette: array::from_fn(|i| i as Color),
    };
    image.set(0, 0, &CURSOR_DATA);
    image
}

fn init_font_image() -> Image {
    let w = FONT_WIDTH * NUM_FONT_ROWS;
    let h = FONT_HEIGHT * (FONT_DATA.len() as u32).div_ceil(NUM_FONT_ROWS);
    let mut image = Image {
        canvas: Canvas::new(w, h),
        palette: array::from_fn(|i| i as Color),
    };
    for (fi, data) in FONT_DATA.iter().enumerate() {
        let row = fi as u32 / NUM_FONT_ROWS;
        let col = fi as u32 % NUM_FONT_ROWS;
        let mut data = *data;
        for yi in 0..FONT_HEIGHT {
            for xi in 0..FONT_WIDTH {
                let x = FONT_WIDTH * col + xi;
                let y = FONT_HEIGHT * row + yi;
                let color = Color::from((data & 0x800000) != 0);
                image.canvas.write_data(x as usize, y as usize, color);
                data <<= 1;
            }
        }
    }
    image
}

fn init_channels() -> Vec<*mut Channel> {
    (0..NUM_CHANNELS).map(|_| Channel::new()).collect()
}

fn init_tones() -> Vec<*mut Tone> {
    macro_rules! set_default_tone {
        ($tone:expr, $default_tone:ident) => {{
            $tone.mode = $default_tone.0;
            $tone.sample_bits = $default_tone.1;
            $tone.wavetable = $default_tone.2.to_vec();
            $tone.gain = $default_tone.3;
        }};
    }

    (0..NUM_TONES)
        .map(|index| {
            let tone = Tone::new();
            let tone_ref = unsafe { &mut *tone };
            match index {
                0 => set_default_tone!(tone_ref, DEFAULT_TONE_0),
                1 => set_default_tone!(tone_ref, DEFAULT_TONE_1),
                2 => set_default_tone!(tone_ref, DEFAULT_TONE_2),
                3 => set_default_tone!(tone_ref, DEFAULT_TONE_3),
                _ => panic!(),
            }
            tone
        })
        .collect()
}

fn init_sounds() -> Vec<*mut Sound> {
    (0..NUM_SOUNDS).map(|_| Sound::new()).collect()
}

fn init_musics() -> Vec<*mut Music> {
    (0..NUM_MUSICS).map(|_| Music::new()).collect()
}
