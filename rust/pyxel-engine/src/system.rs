use std::cmp::{max, min};

use cfg_if::cfg_if;
use pyxel_platform::Event;

use crate::image::{Color, Image, SharedImage};
use crate::keys::{Key, KEY_0, KEY_1, KEY_2, KEY_3, KEY_9, KEY_ALT, KEY_RETURN, KEY_SHIFT};
use crate::profiler::Profiler;
use crate::pyxel::Pyxel;
use crate::settings::{MAX_ELAPSED_MS, NUM_MEASURE_FRAMES, NUM_SCREEN_TYPES};
use crate::utils;
use crate::watch_info::WatchInfo;

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

pub struct System {
    one_frame_ms: f64,
    next_update_ms: f64,
    quit_key: Key,
    paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
    watch_info: WatchInfo,
    pub screen_x: i32,
    pub screen_y: i32,
    pub screen_scale: u32,
    pub screen_mode: u32,
}

impl System {
    pub fn new(fps: u32, quit_key: Key) -> Self {
        Self {
            one_frame_ms: 1000.0 / fps as f64,
            next_update_ms: 0.0,
            quit_key,
            paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            perf_monitor_enabled: false,
            watch_info: WatchInfo::new(),
            screen_x: 0,
            screen_y: 0,
            screen_scale: 0,
            screen_mode: 0,
        }
    }
}

impl Pyxel {
    pub fn run<T: PyxelCallback>(&mut self, mut callback: T) {
        pyxel_platform::run(move || {
            self.process_frame(&mut callback);
        });
    }

    pub fn show(&mut self) {
        struct App {
            image: SharedImage,
        }

        impl PyxelCallback for App {
            fn update(&mut self, _pyxel: &mut Pyxel) {}
            fn draw(&mut self, pyxel: &mut Pyxel) {
                pyxel.screen.lock().blt(
                    0.0,
                    0.0,
                    self.image.clone(),
                    0.0,
                    0.0,
                    pyxel.width as f64,
                    pyxel.height as f64,
                    None,
                    None,
                    None,
                );
            }
        }

        let image = Image::new(self.width, self.height);
        image.lock().blt(
            0.0,
            0.0,
            self.screen.clone(),
            0.0,
            0.0,
            self.width as f64,
            self.height as f64,
            None,
            None,
            None,
        );
        self.run(App { image });
    }

    pub fn flip(&mut self) {
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                panic!("flip is not supported for Web");
            } else {
                self.process_frame_for_flip();
            }
        }
    }

    pub fn quit(&self) {
        pyxel_platform::quit();
    }

    pub fn title(&self, title: &str) {
        pyxel_platform::set_window_title(title);
    }

    pub fn icon(&self, data_str: &[&str], scale: u32, transparent: Option<Color>) {
        let colors = self.colors.lock();
        let width = utils::simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Image::new(width, height);
        let mut image = image.lock();
        image.set(0, 0, data_str);
        let image_data = &image.canvas.data;
        let scaled_width = width * scale;
        let scaled_height = height * scale;
        let mut rgba_data: Vec<u8> =
            Vec::with_capacity((scaled_width * scaled_height * 4) as usize);
        for y in 0..height {
            for _sy in 0..scale {
                for x in 0..width {
                    let color = image_data[(width * y + x) as usize];
                    let rgb = colors[color as usize];
                    let r = (rgb >> 16) as u8;
                    let g = (rgb >> 8) as u8;
                    let b = rgb as u8;
                    let a = if Some(color) == transparent {
                        0x00
                    } else {
                        0xff
                    };
                    for _sx in 0..scale {
                        rgba_data.push(r);
                        rgba_data.push(g);
                        rgba_data.push(b);
                        rgba_data.push(a);
                    }
                }
            }
        }
        pyxel_platform::set_window_icon(scaled_width, scaled_height, &rgba_data);
    }

    pub fn fullscreen(&self, full: bool) {
        pyxel_platform::set_fullscreen(full);
    }

    pub fn screen_mode(&mut self, screen_mode: u32) {
        self.system.screen_mode = screen_mode;
    }

    fn process_events(&mut self) {
        self.start_input_frame();
        let events = pyxel_platform::poll_events();
        for event in events {
            match event {
                Event::WindowShown => {
                    self.system.paused = false;
                    pyxel_platform::set_audio_enabled(true);
                }
                Event::WindowHidden => {
                    self.system.paused = true;
                    pyxel_platform::set_audio_enabled(false);
                }
                Event::KeyPressed { key } => {
                    self.press_key(key);
                }
                Event::KeyReleased { key } => {
                    self.release_key(key);
                }
                Event::KeyValueChanged { key, value } => {
                    self.change_key_value(key, value);
                }
                Event::TextInput { text } => {
                    self.add_input_text(&text);
                }
                Event::FileDropped { filename } => {
                    self.add_dropped_file(&filename);
                }
                Event::Quit => {
                    pyxel_platform::quit();
                }
            }
        }
    }

    fn check_special_input(&mut self) {
        if self.btnp(self.system.quit_key, None, None) {
            self.reset_key(self.system.quit_key);
            self.quit();
        } else if self.btn(KEY_ALT) {
            if self.btn(KEY_SHIFT) {
                if self.btnp(KEY_0, None, None) {
                    self.reset_key(KEY_0);
                    self.dump_palette();
                } else {
                    for i in 0..=8 {
                        if self.btnp(KEY_1 + i, None, None) {
                            self.reset_key(KEY_1 + i);
                            self.dump_image_bank(i);
                        }
                    }
                }
            } else if self.btnp(KEY_0, None, None) {
                self.reset_key(KEY_0);
                self.system.perf_monitor_enabled = !self.system.perf_monitor_enabled;
            } else if self.btnp(KEY_1, None, None) {
                self.reset_key(KEY_1);
                self.screenshot(None);
            } else if self.btnp(KEY_2, None, None) {
                self.reset_key(KEY_2);
                self.reset_screencast();
            } else if self.btnp(KEY_3, None, None) {
                self.reset_key(KEY_3);
                self.screencast(None);
            } else if self.btnp(KEY_9, None, None) {
                self.reset_key(KEY_9);
                self.system.screen_mode = (self.system.screen_mode + 1) % NUM_SCREEN_TYPES;
            } else if self.btnp(KEY_RETURN, None, None) {
                self.reset_key(KEY_RETURN);
                self.fullscreen(!pyxel_platform::is_fullscreen());
            }
        }
    }

    fn update_screen_params(&mut self) {
        let (window_width, window_height) = pyxel_platform::window_size();
        self.system.screen_scale = max(
            min(window_width / self.width, window_height / self.height),
            1,
        );
        self.system.screen_x =
            (window_width as i32 - (self.width * self.system.screen_scale) as i32) / 2;
        self.system.screen_y =
            (window_height as i32 - (self.height * self.system.screen_scale) as i32) / 2;
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.system
            .update_profiler
            .start(pyxel_platform::elapsed_time());
        self.process_events();
        if self.system.paused {
            return;
        }
        self.check_special_input();
        if let Some(callback) = callback {
            callback.update(self);
            self.system
                .update_profiler
                .end(pyxel_platform::elapsed_time());
        }
    }

    fn draw_perf_monitor(&self) {
        if !self.system.perf_monitor_enabled {
            return;
        }
        let mut screen = self.screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette1 = screen.palette[1];
        let palette2 = screen.palette[2];
        let alpha = screen.canvas.alpha;
        screen.clip0();
        screen.camera0();
        screen.pal(1, 1);
        screen.pal(2, 9);
        screen.dither(1.0);

        let fps = format!("{:.*}", 2, self.system.fps_profiler.average_fps());
        screen.text(1.0, 0.0, &fps, 1, None);
        screen.text(0.0, 0.0, &fps, 2, None);

        let update_time = format!("{:.*}", 2, self.system.update_profiler.average_time());
        screen.text(1.0, 6.0, &update_time, 1, None);
        screen.text(0.0, 6.0, &update_time, 2, None);

        let draw_time = format!("{:.*}", 2, self.system.draw_profiler.average_time());
        screen.text(1.0, 12.0, &draw_time, 1, None);
        screen.text(0.0, 12.0, &draw_time, 2, None);

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.pal(1, palette1);
        screen.pal(2, palette2);
        screen.dither(alpha);
    }

    fn draw_cursor(&self) {
        let x = self.mouse_x;
        let y = self.mouse_y;
        pyxel_platform::set_mouse_visible(
            x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32,
        );
        if !self.is_mouse_visible() {
            return;
        }
        let width = self.cursor.lock().width() as i32;
        let height = self.cursor.lock().height() as i32;
        if x <= -width || x >= self.width as i32 || y <= -height || y >= self.height as i32 {
            return;
        }
        let mut screen = self.screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;
        screen.clip0();
        screen.camera0();
        screen.blt(
            x as f64,
            y as f64,
            self.cursor.clone(),
            0.0,
            0.0,
            width as f64,
            height as f64,
            Some(0),
            None,
            None,
        );
        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        if self.system.paused {
            return;
        }
        self.system
            .draw_profiler
            .start(pyxel_platform::elapsed_time());
        if let Some(callback) = callback {
            callback.draw(self);
        }
        self.system.watch_info.update();
        self.draw_perf_monitor();
        self.draw_cursor();
        self.render_screen();
        self.capture_screen();
        self.system
            .draw_profiler
            .end(pyxel_platform::elapsed_time());
    }

    fn process_frame(&mut self, callback: &mut dyn PyxelCallback) {
        let tick_count = pyxel_platform::elapsed_time();
        let elapsed_ms = tick_count as f64 - self.system.next_update_ms;
        if elapsed_ms < 0.0 {
            return;
        }
        if self.frame_count == 0 {
            self.system.next_update_ms = tick_count as f64 + self.system.one_frame_ms;
        } else {
            self.system.fps_profiler.end(tick_count);
            self.system.fps_profiler.start(tick_count);
            let update_count: u32;
            if elapsed_ms > MAX_ELAPSED_MS as f64 {
                update_count = 1;
                self.system.next_update_ms =
                    pyxel_platform::elapsed_time() as f64 + self.system.one_frame_ms;
            } else {
                update_count = (elapsed_ms / self.system.one_frame_ms) as u32 + 1;
                self.system.next_update_ms += self.system.one_frame_ms * update_count as f64;
            }
            for _ in 1..update_count {
                self.update_frame(Some(callback));
                self.frame_count += 1;
            }
        }
        self.update_screen_params();
        self.update_frame(Some(callback));
        self.draw_frame(Some(callback));
        self.frame_count += 1;
    }

    #[cfg(not(target_os = "emscripten"))]
    fn process_frame_for_flip(&mut self) {
        self.system
            .update_profiler
            .end(pyxel_platform::elapsed_time());
        self.update_screen_params();
        self.draw_frame(None);
        self.frame_count += 1;
        let mut tick_count;
        let mut elapsed_ms;
        loop {
            tick_count = pyxel_platform::elapsed_time();
            elapsed_ms = tick_count as f64 - self.system.next_update_ms;
            let wait_ms = self.system.next_update_ms - pyxel_platform::elapsed_time() as f64;
            if wait_ms > 0.0 {
                pyxel_platform::sleep((wait_ms / 2.0) as u32);
            } else {
                break;
            }
        }
        self.system.fps_profiler.end(tick_count);
        self.system.fps_profiler.start(tick_count);
        if elapsed_ms > MAX_ELAPSED_MS as f64 {
            self.system.next_update_ms =
                pyxel_platform::elapsed_time() as f64 + self.system.one_frame_ms;
        } else {
            self.system.next_update_ms += self.system.one_frame_ms;
        }
        self.update_frame(None);
    }
}
