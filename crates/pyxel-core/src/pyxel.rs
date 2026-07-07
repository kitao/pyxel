use std::array;
use std::cell::{RefCell, RefMut};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};

use crate::audio::Audio;
use crate::canvas::Canvas;
use crate::channel::{Channel, RcChannel};
use crate::graphics::Graphics;
use crate::image::{Color, Image, RcImage, Rgb24};
use crate::input::Input;
use crate::key::Key;
use crate::music::{Music, RcMusic};
use crate::platform;
use crate::resource::Resource;
use crate::settings::{
    CURSOR_DATA, CURSOR_HEIGHT, CURSOR_WIDTH, DEFAULT_COLORS, DEFAULT_FPS, DEFAULT_QUIT_KEY,
    DEFAULT_TITLE, DEFAULT_TONE_NOISE, DEFAULT_TONE_PULSE, DEFAULT_TONE_SQUARE,
    DEFAULT_TONE_TRIANGLE, FONT_DATA, FONT_HEIGHT, FONT_WIDTH, ICON_COLKEY, ICON_DATA, ICON_SCALE,
    IMAGE_SIZE, NUM_CHANNELS, NUM_FONT_COLS, NUM_IMAGES, NUM_MUSICS, NUM_SOUNDS, NUM_TILEMAPS,
    NUM_TONES, TILEMAP_SIZE, WINDOW_TO_DISPLAY_RATIO,
};
use crate::sound::{RcSound, Sound};
use crate::system::System;
use crate::tilemap::{ImageSource, RcTilemap, Tilemap};
use crate::tone::{RcTone, Tone};

pub struct Pyxel {
    pub(crate) system: System,
    pub(crate) resource: Resource,
    pub(crate) input: Input,
    pub(crate) graphics: Option<Graphics>,
}

static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

// Singleton
thread_local! {
    static PYXEL: &'static RefCell<Option<Pyxel>> =
        Box::leak(Box::new(RefCell::new(None)));
}

pub fn pyxel() -> RefMut<'static, Pyxel> {
    PYXEL.with(|instance| {
        let instance: &'static RefCell<Option<Pyxel>> = instance;
        RefMut::map(instance.borrow_mut(), |instance| {
            instance.as_mut().expect("Pyxel not initialized")
        })
    })
}

fn set_pyxel(instance: Pyxel) {
    // The leaked RefCell keeps the owner address stable through Python module
    // cleanup; replace its value only when the next initialization is ready.
    PYXEL.with(|current| *current.borrow_mut() = Some(instance));
}

// Lifecycle callbacks

type ResetCallback = Option<Box<dyn FnMut(Option<String>) + Send>>;

thread_local! {
    static RESET_CALLBACK: &'static RefCell<ResetCallback> =
        Box::leak(Box::new(RefCell::new(None)));
}

pub fn reset_callback() -> RefMut<'static, ResetCallback> {
    RESET_CALLBACK.with(|callback| {
        let callback: &'static RefCell<_> = callback;
        callback.borrow_mut()
    })
}

thread_local! {
    static QUIT_CALLBACK: &'static RefCell<Option<Box<dyn FnMut() + Send>>> =
        Box::leak(Box::new(RefCell::new(None)));
}

pub fn quit_callback() -> RefMut<'static, Option<Box<dyn FnMut() + Send>>> {
    QUIT_CALLBACK.with(|callback| {
        let callback: &'static RefCell<_> = callback;
        callback.borrow_mut()
    })
}

// Macros for global variables
macro_rules! define_static {
    ($func:ident, $static:ident, $type:ty, $default:expr) => {
        thread_local! {
            static $static: &'static RefCell<$type> =
                Box::leak(Box::new(RefCell::new($default)));
        }
        pub fn $func() -> RefMut<'static, $type> {
            $static.with(|value| {
                let value: &'static RefCell<$type> = value;
                value.borrow_mut()
            })
        }
    };
}

macro_rules! define_global {
    ($func:ident, $static:ident, $type:ty, $init:expr) => {
        thread_local! {
            static $static: &'static RefCell<Option<$type>> =
                Box::leak(Box::new(RefCell::new(None)));
        }
        pub fn $func() -> RefMut<'static, $type> {
            $static.with(|value| {
                let value: &'static RefCell<Option<$type>> = value;
                let mut value = value.borrow_mut();
                if value.is_none() {
                    *value = Some($init);
                }
                RefMut::map(value, |value| value.as_mut().unwrap())
            })
        }
    };
}

pub struct AudioGlobalGuard<T: 'static> {
    guard: MutexGuard<'static, Option<T>>,
}

impl<T> Deref for AudioGlobalGuard<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap()
    }
}

impl<T> DerefMut for AudioGlobalGuard<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_mut().unwrap()
    }
}

macro_rules! define_audio_global {
    ($func:ident, $static:ident, $type:ty, $init:expr) => {
        static $static: Mutex<Option<$type>> = Mutex::new(None);
        pub fn $func() -> AudioGlobalGuard<$type> {
            let mut guard = $static.lock().unwrap_or_else(PoisonError::into_inner);
            if guard.is_none() {
                *guard = Some($init);
            }
            AudioGlobalGuard { guard }
        }
    };
}

// System
define_static!(is_headless, IS_HEADLESS, bool, false);
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
define_global!(images, IMAGES, Vec<RcImage>, init_images());
define_global!(tilemaps, TILEMAPS, Vec<RcTilemap>, init_tilemaps());
define_global!(screen, SCREEN, RcImage, init_screen());
define_global!(cursor_image, CURSOR_IMAGE, RcImage, init_cursor_image());
define_global!(font_image, FONT_IMAGE, RcImage, init_font_image());

// Audio
define_audio_global!(channels, CHANNELS, Vec<RcChannel>, init_channels());
define_audio_global!(tones, TONES, Vec<RcTone>, init_tones());
define_audio_global!(sounds, SOUNDS, Vec<RcSound>, init_sounds());
define_audio_global!(musics, MUSICS, Vec<RcMusic>, init_musics());

pub fn init(
    w: u32,
    h: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
    headless: Option<bool>,
) -> Result<(), String> {
    validate_init_params(w, h, fps, display_scale, headless)?;
    assert!(
        !IS_INITIALIZED.swap(true, Ordering::Relaxed),
        "Pyxel already initialized"
    );

    let headless = headless.unwrap_or(false);
    *is_headless() = headless;

    // Set dimensions
    *width() = w;
    *height() = h;
    *frame_count() = 0;

    // Default parameters
    let title = title.unwrap_or(DEFAULT_TITLE);
    let quit_key = quit_key.unwrap_or(DEFAULT_QUIT_KEY);
    let fps = fps.unwrap_or(DEFAULT_FPS);

    // Platform
    platform::init(headless);

    if !headless {
        let (display_width, display_height) = platform::display_size();
        let display_scale = display_scale
            .unwrap_or(
                (f32::min(
                    display_width as f32 / w as f32,
                    display_height as f32 / h as f32,
                ) * WINDOW_TO_DISPLAY_RATIO) as u32,
            )
            .max(1);
        let window_width = w * display_scale;
        let window_height = h * display_scale;

        platform::init_window(title, window_width, window_height);
    }

    // Resize screen
    rc_mut!(screen()).canvas = Canvas::new(w, h);
    rc_mut!(screen()).palette = array::from_fn(|i| i as Color);

    // Reset input
    *mouse_x() = 0;
    *mouse_y() = 0;
    *mouse_wheel() = 0;
    input_keys().clear();
    input_text().clear();
    dropped_files().clear();

    // Build Pyxel instance
    let system = System::new(fps, quit_key, headless);
    let resource = Resource::new(capture_scale, capture_sec, fps);
    let input = Input::new();
    let graphics = if headless {
        None
    } else {
        Some(Graphics::new())
    };

    set_pyxel(Pyxel {
        system,
        resource,
        input,
        graphics,
    });

    // Audio::start() looks unconditional here even in headless mode —
    // that's intentional. Default behavior matches v2.8.5's design
    // (a headless script still gets real audio via SDL2). See
    // audio.rs's own #[cfg(feature = "no_headless_audio")]-gated
    // Audio::start() implementations for the opt-in exception, used
    // by embedders that drive render_samples() externally themselves
    // (e.g. a libretro core) and don't want this SDL2 callback thread
    // racing their own manual calls.
    Audio::start();
    if !headless {
        pyxel().update_screen_params();
        pyxel()
            .set_icon(&ICON_DATA, ICON_SCALE, ICON_COLKEY)
            .expect("built-in icon data must be valid");
    }
    Ok(())
}

pub fn validate_init_params(
    w: u32,
    h: u32,
    fps: Option<u32>,
    display_scale: Option<u32>,
    headless: Option<bool>,
) -> Result<(), String> {
    validate_nonzero_screen_dimensions(w, h)?;
    if fps.unwrap_or(DEFAULT_FPS) == 0 {
        return Err("fps must be greater than 0".to_string());
    }
    if !headless.unwrap_or(false) {
        validate_platform_screen_dimensions(w, h)?;
        let max_window_size = i32::MAX as u32;
        if let Some(display_scale) = display_scale {
            let display_scale = display_scale.max(1);
            let width_is_valid = w
                .checked_mul(display_scale)
                .is_some_and(|width| width <= max_window_size);
            let height_is_valid = h
                .checked_mul(display_scale)
                .is_some_and(|height| height <= max_window_size);
            if !width_is_valid || !height_is_valid {
                return Err("display_scale is too large for the window dimensions".to_string());
            }
        }
    }
    validate_screen_area(w, h)?;
    Ok(())
}

pub(crate) fn validate_resize_params(w: u32, h: u32, headless: bool) -> Result<(), String> {
    validate_nonzero_screen_dimensions(w, h)?;
    if !headless {
        validate_platform_screen_dimensions(w, h)?;
    }
    validate_screen_area(w, h)
}

fn validate_nonzero_screen_dimensions(w: u32, h: u32) -> Result<(), String> {
    if w == 0 || h == 0 {
        return Err("width and height must be greater than 0".to_string());
    }
    Ok(())
}

fn validate_platform_screen_dimensions(w: u32, h: u32) -> Result<(), String> {
    let max_window_size = i32::MAX as u32;
    if w > max_window_size || h > max_window_size {
        return Err("width and height exceed platform window limits".to_string());
    }
    Ok(())
}

fn validate_screen_area(w: u32, h: u32) -> Result<(), String> {
    if w.checked_mul(h).is_none() {
        return Err("screen dimensions are too large".to_string());
    }
    Ok(())
}

#[cfg(target_os = "emscripten")]
pub fn reset_statics() {
    IS_INITIALIZED.store(false, Ordering::Relaxed);

    // Reset scalar statics
    *is_headless() = false;
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
    macro_rules! drop_global {
        ($static:ident) => {
            $static.with(|value| *value.borrow_mut() = None);
        };
    }

    macro_rules! drop_audio_global {
        ($static:ident) => {
            *$static.lock().unwrap_or_else(PoisonError::into_inner) = None;
        };
    }

    drop_global!(COLORS);
    drop_global!(IMAGES);
    drop_global!(TILEMAPS);
    drop_global!(SCREEN);
    drop_global!(CURSOR_IMAGE);
    drop_global!(FONT_IMAGE);
    drop_audio_global!(CHANNELS);
    drop_audio_global!(TONES);
    drop_audio_global!(SOUNDS);
    drop_audio_global!(MUSICS);

    *reset_callback() = None;
    *quit_callback() = None;
}

// Init functions for define_global!

fn init_images() -> Vec<RcImage> {
    (0..NUM_IMAGES)
        .map(|_| Image::new(IMAGE_SIZE, IMAGE_SIZE))
        .collect()
}

fn init_tilemaps() -> Vec<RcTilemap> {
    (0..NUM_TILEMAPS)
        .map(|_| Tilemap::new(TILEMAP_SIZE, TILEMAP_SIZE, ImageSource::Index(0)))
        .collect()
}

fn init_screen() -> RcImage {
    new_rc_type!(Image {
        canvas: Canvas::new(0, 0),
        palette: array::from_fn(|i| i as Color),
        palette_is_identity: true,
    })
}

fn init_cursor_image() -> RcImage {
    let rc = new_rc_type!(Image {
        canvas: Canvas::new(CURSOR_WIDTH, CURSOR_HEIGHT),
        palette: array::from_fn(|i| i as Color),
        palette_is_identity: true,
    });
    rc_mut!(rc)
        .set(0, 0, &CURSOR_DATA)
        .expect("built-in cursor data must be valid");
    rc
}

fn init_font_image() -> RcImage {
    let w = FONT_WIDTH * NUM_FONT_COLS;
    let h = FONT_HEIGHT * (FONT_DATA.len() as u32).div_ceil(NUM_FONT_COLS);
    let rc = new_rc_type!(Image {
        canvas: Canvas::new(w, h),
        palette: array::from_fn(|i| i as Color),
        palette_is_identity: true,
    });
    let mut image = rc_mut!(rc);
    // Each u32 packs one 4x6 glyph MSB-first in its low 24 bits (bit 23 = top-left)
    for (i, data) in FONT_DATA.iter().enumerate() {
        let row = i as u32 / NUM_FONT_COLS;
        let col = i as u32 % NUM_FONT_COLS;
        let mut data = *data;
        for yi in 0..FONT_HEIGHT {
            for xi in 0..FONT_WIDTH {
                let x = FONT_WIDTH * col + xi;
                let y = FONT_HEIGHT * row + yi;
                let color = Color::from((data & 0x0080_0000) != 0);
                image.canvas.write_data(x as usize, y as usize, color);
                data <<= 1;
            }
        }
    }
    drop(image);
    rc
}

fn init_channels() -> Vec<RcChannel> {
    (0..NUM_CHANNELS).map(|_| Channel::new()).collect()
}

fn init_tones() -> Vec<RcTone> {
    macro_rules! set_tone {
        ($tone:expr, $default:ident) => {{
            $tone.mode = $default.0;
            $tone.sample_bits = $default.1;
            $tone.wavetable = $default.2.to_vec();
            $tone.gain = $default.3;
        }};
    }

    (0..NUM_TONES)
        .map(|index| {
            let tone = Tone::new();
            let mut t = audio_mut!(tone);
            match index {
                0 => set_tone!(t, DEFAULT_TONE_TRIANGLE),
                1 => set_tone!(t, DEFAULT_TONE_SQUARE),
                2 => set_tone!(t, DEFAULT_TONE_PULSE),
                3 => set_tone!(t, DEFAULT_TONE_NOISE),
                _ => unreachable!(),
            }
            drop(t);
            tone
        })
        .collect()
}

fn init_sounds() -> Vec<RcSound> {
    (0..NUM_SOUNDS).map(|_| Sound::new()).collect()
}

fn init_musics() -> Vec<RcMusic> {
    (0..NUM_MUSICS).map(|_| Music::new()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scalar_state_is_isolated_between_threads() {
        *width() = 123;

        std::thread::spawn(|| {
            *width() = 999;
            assert_eq!(*width(), 999);
        })
        .join()
        .unwrap();

        assert_eq!(*width(), 123);
    }

    #[test]
    fn audio_resource_state_is_shared_with_callback_thread() {
        let main_resources = {
            let channels = channels();
            let tones = tones();
            let sounds = sounds();
            let musics = musics();
            (
                std::sync::Arc::as_ptr(&channels[0]) as usize,
                std::sync::Arc::as_ptr(&tones[0]) as usize,
                std::sync::Arc::as_ptr(&sounds[0]) as usize,
                std::sync::Arc::as_ptr(&musics[0]) as usize,
            )
        };
        let callback_resources = std::thread::spawn(|| {
            let channels = channels();
            let tones = tones();
            let sounds = sounds();
            let musics = musics();
            (
                std::sync::Arc::as_ptr(&channels[0]) as usize,
                std::sync::Arc::as_ptr(&tones[0]) as usize,
                std::sync::Arc::as_ptr(&sounds[0]) as usize,
                std::sync::Arc::as_ptr(&musics[0]) as usize,
            )
        })
        .join()
        .unwrap();

        assert_eq!(callback_resources, main_resources);
    }
}
