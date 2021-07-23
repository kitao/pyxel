#[macro_use]
mod system;
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
mod palette;
mod platform;
mod profiler;
mod recorder;
mod rectarea;
mod resource;
mod sdl2;
mod settings;
mod sound;
mod tilemap;
mod utility;

use std::sync::{Arc, Mutex};

use crate::audio::Audio;
use crate::canvas::Canvas;
use crate::graphics::Graphics;
use crate::image::Image as PyxelImage;
use crate::input::Input;
use crate::oscillator::{Effect, Tone};
use crate::palette::{Color, Rgb24};
use crate::resource::Resource;
use crate::sdl2::Sdl2;
use crate::sound::{Note, Speed, Volume};
use crate::system::System;
use crate::tilemap::Tile;

pub use crate::key::*;
pub use crate::settings::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Image {
    image_no: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Tilemap {
    tilemap_no: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Sound {
    sound_no: u32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Music {
    music_no: u32,
}

pub struct Pyxel {
    system: System<Sdl2>,
    resource: Resource,
    input: Input,
    graphics: Graphics,
    audio: Arc<Mutex<Audio>>,
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
        colors: Option<&[Rgb24]>,
    ) -> Pyxel {
        let mut system = System::new(width, height, title, scale, fps, quit_key);
        let resource = Resource::new();
        let input = Input::new();
        let graphics = Graphics::new(width, height, colors);
        let audio = Audio::new(system.platform_mut());

        Pyxel {
            system: system,
            resource: resource,
            input: input,
            graphics: graphics,
            audio: audio,
        }
    }

    pub fn width(&self) -> u32 {
        self.graphics.screen().width()
    }

    pub fn height(&self) -> u32 {
        self.graphics.screen().height()
    }

    pub fn frame_count(&self) -> u32 {
        self.system.frame_count()
    }

    pub fn title(&mut self, title: &str) {
        self.system.set_window_title(title);
    }

    pub fn fullscreen(&mut self, is_fullscreen: bool) {
        self.system.set_fullscreen(is_fullscreen);
    }

    pub fn run(&mut self, callback: &mut dyn PyxelCallback) {
        run!(self, callback);
    }

    pub fn quit(&mut self) {
        self.system.quit();
    }

    /*
    int flip();
    void show();

    void _drop_file_getter(char* str, int str_length);
    void _caption(const char* caption);
    */

    //
    // Resource
    //
    pub fn save(&mut self, filename: &str) {
        self.resource.save_asset(filename);
    }

    pub fn load(&mut self, filename: &str, image: bool, tilemap: bool, sound: bool, music: bool) {
        self.resource
            .load_asset(filename, image, tilemap, sound, music);
    }

    //
    // Input
    //
    pub fn mouse_x(&self) -> i32 {
        self.input.key_value(MOUSE_POS_X)
    }

    pub fn mouse_y(&self) -> i32 {
        self.input.key_value(MOUSE_POS_Y)
    }

    pub fn mouse_wheel(&self) -> i32 {
        self.input.key_value(MOUSE_WHEEL_Y)
    }

    pub fn mouse(&mut self, visible: bool) {
        self.input.set_mouse_visible(visible);
    }

    pub fn btn(&self, key: Key) -> bool {
        self.input.is_key_on(key)
    }

    pub fn btnp(&self, key: Key, hold: Option<u32>, period: Option<u32>) -> bool {
        self.input.is_key_pressed(key, hold, period)
    }

    pub fn btnr(&self, key: Key) -> bool {
        self.input.is_key_released(key)
    }

    pub fn text_input(&self) -> &str {
        self.input.text_input()
    }

    //
    // Graphics
    //
    pub fn image(&self, img: u32) -> Image {
        Image { image_no: img }
    }

    pub fn tilemap(&self, tlm: u32) -> Tilemap {
        Tilemap { tilemap_no: tlm }
    }

    pub fn clip_(&mut self) {
        self.graphics.screen_mut().reset_clip_area();
    }

    pub fn clip(&mut self, x: i32, y: i32, w: u32, h: u32) {
        self.graphics.screen_mut().set_clip_area(x, y, w, h);
    }

    pub fn pal_(&mut self) {
        self.graphics
            .screen_mut()
            .palette_mut()
            .reset_render_colors();
    }

    pub fn pal(&mut self, col1: Color, col2: Color) {
        self.graphics
            .screen_mut()
            .palette_mut()
            .set_render_color(col1, col2);
    }

    pub fn cls(&mut self, col: Color) {
        self.graphics.screen_mut().clear(col);
    }

    pub fn pget(&mut self, x: i32, y: i32) -> Color {
        self.graphics.screen_mut().value(x, y)
    }

    pub fn pset(&mut self, x: i32, y: i32, col: Color) {
        self.graphics.screen_mut().draw_point(x, y, col);
    }

    pub fn line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, col: Color) {
        self.graphics.screen_mut().draw_line(x1, y1, x2, y2, col);
    }

    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_rect(x, y, w as f64 as u32, h as f64 as u32, col);
    }

    pub fn rectb(&mut self, x: i32, y: i32, w: i32, h: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_rect_border(x, y, w as f64 as u32, h as f64 as u32, col);
    }

    pub fn circ(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.graphics.screen_mut().draw_circle(x, y, r, col);
    }

    pub fn circb(&mut self, x: i32, y: i32, r: u32, col: Color) {
        self.graphics.screen_mut().draw_circle_border(x, y, r, col);
    }

    pub fn tri(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_triangle(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn trib(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: Color) {
        self.graphics
            .screen_mut()
            .draw_triangle_border(x1, y1, x2, y2, x3, y3, col);
    }

    pub fn blt(
        &mut self,
        x: i32,
        y: i32,
        img: u32,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        colkey: Option<Color>,
    ) {
        unsafe {
            let screen = self.graphics.screen_mut() as *mut PyxelImage;
            (&mut *screen).copy(x, y, self.graphics.image(img), u, v, w, h, colkey);
        }
    }

    pub fn bltm(
        &mut self,
        x: i32,
        y: i32,
        tm: u32,
        u: i32,
        v: i32,
        w: i32,
        h: i32,
        tilekey: Option<Tile>,
    ) {
        unsafe {
            let screen = self.graphics.screen_mut() as *mut PyxelImage;
            (&mut *screen).draw_tilemap(x, y, self.graphics.tilemap(tm), u, v, w, h, tilekey);
        }
    }

    pub fn text(&mut self, x: i32, y: i32, s: &str, col: Color) {
        unsafe {
            let screen = self.graphics.screen_mut() as *mut PyxelImage;
            (&mut *screen).draw_text(&self.graphics, x, y, s, col);
        }
    }

    //
    // Audio
    //
    pub fn sound(&self, snd: u32) -> Sound {
        Sound { sound_no: snd }
    }

    pub fn music(&self, msc: u32) -> Music {
        Music { music_no: msc }
    }

    pub fn play_pos(&self, ch: u32) -> Option<(u32, u32)> {
        self.audio.lock().unwrap().play_pos(ch)
    }

    pub fn play1(&mut self, ch: u32, snd: u32, loop_: bool) {
        self.audio.lock().unwrap().play_sound(ch, snd, loop_);
    }

    pub fn play(&mut self, ch: u32, snds: &[u32], loop_: bool) {
        self.audio.lock().unwrap().play_sounds(ch, snds, loop_);
    }

    pub fn playm(&mut self, msc: u32, loop_: bool) {
        self.audio.lock().unwrap().play_music(msc, loop_);
    }

    pub fn stop(&mut self, ch: u32) {
        self.audio.lock().unwrap().stop(ch);
    }
}

//
// Image class
//
impl Image {
    pub fn set(&self, pyxel: &mut Pyxel, x: i32, y: i32, data: &[&str]) {
        pyxel.graphics.image_mut(self.image_no).set(x, y, data);
    }

    pub fn load(&self, pyxel: &mut Pyxel, x: i32, y: i32, filename: &str) {
        pyxel
            .graphics
            .image_mut(self.image_no)
            .load_image(x, y, filename);
    }

    pub fn save(&self, pyxel: &mut Pyxel, filename: &str, scale: u32) {
        pyxel
            .graphics
            .image(self.image_no)
            .save_image(filename, scale);
    }
}

/*
int image_width_getter(void* self);
int image_height_getter(void* self);
int** image_data_getter(void* self);

int image_get(void* self, int x, int y);
void image_set1(void* self, int x, int y, int data);
void image_set(void* self, int x, int y, const char** data, int data_length);
void image_load(void* self, int x, int y, const char* filename);
void image_copy(void* self, int x, int y, int img, int u, int v, int w, int h);
*/

//
// Tilemap class
//
impl Tilemap {
    pub fn set(&self, pyxel: &mut Pyxel, x: i32, y: i32, data: &[&str]) {
        pyxel.graphics.tilemap_mut(self.tilemap_no).set(x, y, data);
    }
}

/*
int tilemap_width_getter(void* self);
int tilemap_height_getter(void* self);
int** tilemap_data_getter(void* self);
int tilemap_refimg_getter(void* self);
void tilemap_refimg_setter(void* self, int refimg);

int tilemap_get(void* self, int x, int y);
void tilemap_set1(void* self, int x, int y, int data);
void tilemap_set(void* self, int x, int y, const char** data, int data_length);
void tilemap_copy(void* self, int x, int y, int tm, int u, int v, int w, int h);
*/

//
// Sound class
//
macro_rules! sound {
    ($self:ident, $pyxel:ident) => {
        $pyxel.audio.lock().unwrap().sound_mut($self.sound_no)
    };
}

impl Sound {
    pub fn set(
        &self,
        pyxel: &Pyxel,
        notes: &str,
        tones: &str,
        volumes: &str,
        effects: &str,
        speed: Speed,
    ) {
        sound!(self, pyxel).set(notes, tones, volumes, effects, speed);
    }

    pub fn notes(&self, pyxel: &Pyxel) -> Vec<Note> {
        sound!(self, pyxel).notes().clone()
    }

    pub fn set_notes(&self, pyxel: &Pyxel, notes: &str) {
        sound!(self, pyxel).set_notes(notes);
    }

    pub fn tones(&self, pyxel: &Pyxel) -> Vec<Tone> {
        sound!(self, pyxel).tones().clone()
    }

    pub fn set_tones(&self, pyxel: &Pyxel, tones: &str) {
        sound!(self, pyxel).set_tones(tones);
    }

    pub fn volumes(&self, pyxel: &Pyxel) -> Vec<Volume> {
        sound!(self, pyxel).volumes().clone()
    }

    pub fn set_volumes(&self, pyxel: &Pyxel, volumes: &str) {
        sound!(self, pyxel).set_volumes(volumes);
    }

    pub fn effects(&self, pyxel: &Pyxel) -> Vec<Effect> {
        sound!(self, pyxel).effects().clone()
    }

    pub fn set_effects(&self, pyxel: &Pyxel, effects: &str) {
        sound!(self, pyxel).set_effects(effects);
    }

    pub fn speed(&self, pyxel: &Pyxel) -> Speed {
        sound!(self, pyxel).speed()
    }

    pub fn set_speed(&self, pyxel: &Pyxel, speed: Speed) {
        sound!(self, pyxel).set_speed(speed);
    }
}

//
// Music class
//
macro_rules! music {
    ($self:ident, $pyxel:ident) => {
        $pyxel.audio.lock().unwrap().music_mut($self.music_no)
    };
}

impl Music {
    pub fn set(&self, pyxel: &Pyxel, seq0: &[u32], seq1: &[u32], seq2: &[u32], seq3: &[u32]) {
        music!(self, pyxel).set(seq0, seq1, seq2, seq3);
    }

    pub fn seq(&self, pyxel: &Pyxel, ch: u32) -> Vec<u32> {
        music!(self, pyxel).sequence(ch).clone()
    }

    pub fn set_seq(&self, pyxel: &Pyxel, ch: u32, snds: &[u32]) {
        music!(self, pyxel).set_sequence(ch, snds);
    }
}
