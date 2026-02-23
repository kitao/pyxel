use crate::image::{Color, Image};
use crate::key::{
    Key, GAMEPAD1_BUTTON_A, GAMEPAD1_BUTTON_B, GAMEPAD1_BUTTON_DPAD_DOWN,
    GAMEPAD1_BUTTON_DPAD_LEFT, GAMEPAD1_BUTTON_DPAD_RIGHT, GAMEPAD1_BUTTON_DPAD_UP,
    GAMEPAD1_BUTTON_X, GAMEPAD1_BUTTON_Y, KEY_0, KEY_1, KEY_2, KEY_3, KEY_8, KEY_9, KEY_ALT, KEY_R,
    KEY_RETURN, KEY_SHIFT,
};
use crate::platform::key::GAMEPAD1_BUTTON_BACK;
use crate::platform::Event;
use crate::profiler::Profiler;
use crate::pyxel::{self, Pyxel};
use crate::settings::{MAX_FRAME_DELAY_MS, NUM_MEASURE_FRAMES, NUM_SCREEN_TYPES};
use crate::window_watcher::WindowWatcher;
use crate::{platform, utils};

pub trait PyxelCallback {
    fn update(&mut self, pyxel: &mut Pyxel);
    fn draw(&mut self, pyxel: &mut Pyxel);
}

pub struct System {
    fps: u32,
    frame_ms: f32,
    quit_key: Key,
    paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
    integer_scale_enabled: bool,
    window_watcher: WindowWatcher,
    pub screen_x: i32,
    pub screen_y: i32,
    pub screen_scale: f32,
    pub screen_mode: u32,
}

impl System {
    pub fn new(fps: u32, quit_key: Key) -> Self {
        Self {
            fps,
            frame_ms: 1000.0 / fps as f32,
            quit_key,
            paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            perf_monitor_enabled: false,
            integer_scale_enabled: false,
            window_watcher: WindowWatcher::new(),
            screen_x: 0,
            screen_y: 0,
            screen_scale: 0.0,
            screen_mode: 0,
        }
    }
}

impl Pyxel {
    pub fn run<T: PyxelCallback>(&mut self, mut callback: T) {
        platform::run_frame_loop(self.system.fps, move |delta_ms| {
            let ticks = platform::ticks();
            self.system.fps_profiler.end(ticks);
            self.system.fps_profiler.start(ticks);

            let update_count = if delta_ms > MAX_FRAME_DELAY_MS as f32 {
                1
            } else {
                (delta_ms / self.system.frame_ms) as u32
            };
            for _ in 1..update_count {
                self.update_frame(Some(&mut callback));
                *pyxel::frame_count() += 1;
            }

            self.update_frame(Some(&mut callback));
            self.draw_frame(Some(&mut callback));
            *pyxel::frame_count() += 1;
        });
    }

    pub fn show(&mut self) {
        struct App {
            image: *mut Image,
        }

        unsafe impl Send for App {}

        impl PyxelCallback for App {
            fn update(&mut self, _pyxel: &mut Pyxel) {}
            fn draw(&mut self, _pyxel: &mut Pyxel) {
                unsafe {
                    pyxel::screen().blt(
                        0.0,
                        0.0,
                        self.image,
                        0.0,
                        0.0,
                        *pyxel::width() as f32,
                        *pyxel::height() as f32,
                        None,
                        None,
                        None,
                    );
                }
            }
        }

        let image = Image::new(*pyxel::width(), *pyxel::height());
        unsafe {
            (&mut *image).blt(
                0.0,
                0.0,
                std::ptr::from_mut(pyxel::screen()),
                0.0,
                0.0,
                *pyxel::width() as f32,
                *pyxel::height() as f32,
                None,
                None,
                None,
            );
        }

        self.run(App { image });
    }

    pub fn flip(&mut self) {
        self.system.update_profiler.end(platform::ticks());

        self.draw_frame(None);
        *pyxel::frame_count() += 1;

        platform::step_frame(self.system.fps);

        let ticks = platform::ticks();
        self.system.fps_profiler.end(ticks);
        self.system.fps_profiler.start(ticks);

        self.update_frame(None);
    }

    pub fn quit(&self) {
        platform::quit();
    }

    pub fn reset(&mut self) {
        #[cfg(not(target_os = "emscripten"))]
        if let Some(mut reset_func) = pyxel::reset_func().take() {
            reset_func();
        }

        #[cfg(target_os = "emscripten")]
        {
            extern "C" {
                fn emscripten_run_script(script: *const std::os::raw::c_char);
            }

            unsafe {
                let script = std::ffi::CString::new("resetPyxel();").unwrap();
                emscripten_run_script(script.as_ptr());
            }
        }
    }

    pub fn title(&self, title: &str) {
        platform::set_window_title(title);
    }

    pub fn icon(&self, data_str: &[&str], scale: u32, transparent: Option<Color>) {
        let colors = pyxel::colors();
        let width = utils::simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Image::new(width, height);
        let image = unsafe { &mut *image };
        image.set(0, 0, data_str);
        let image_data = &image.canvas.data;
        let scaled_width = width * scale;
        let scaled_height = height * scale;
        let mut rgba: Vec<u8> = Vec::with_capacity((scaled_width * scaled_height * 4) as usize);

        for y in 0..height {
            for _sy in 0..scale {
                for x in 0..width {
                    let color = image_data[(width * y + x) as usize];
                    let argb = colors[color as usize];
                    let r = ((argb >> 16) & 0xff) as u8;
                    let g = ((argb >> 8) & 0xff) as u8;
                    let b = (argb & 0xff) as u8;
                    let a = if Some(color) == transparent { 0 } else { 0xff };
                    for _sx in 0..scale {
                        rgba.push(r);
                        rgba.push(g);
                        rgba.push(b);
                        rgba.push(a);
                    }
                }
            }
        }

        platform::set_window_icon(scaled_width, scaled_height, &rgba);
    }

    pub fn perf_monitor(&mut self, enabled: bool) {
        self.system.perf_monitor_enabled = enabled;
    }

    pub fn integer_scale(&mut self, enabled: bool) {
        self.system.integer_scale_enabled = enabled;
    }

    pub fn screen_mode(&mut self, screen_mode: u32) {
        self.system.screen_mode = screen_mode;
    }

    pub fn fullscreen(&self, enabled: bool) {
        platform::set_fullscreen(enabled);
    }

    fn process_events(&mut self) {
        self.start_input_frame();

        let events = platform::poll_events();

        for event in events {
            match event {
                Event::WindowShown => {
                    self.system.paused = false;
                    platform::pause_audio(false);
                }
                Event::WindowHidden => {
                    self.system.paused = true;
                    platform::pause_audio(true);
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
                    platform::quit();
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
            } else if self.btnp(KEY_1, None, None) {
                self.reset_key(KEY_1);
                self.screenshot(None);
            } else if self.btnp(KEY_2, None, None) {
                self.reset_key(KEY_2);
                self.reset_screencast();
            } else if self.btnp(KEY_3, None, None) {
                self.reset_key(KEY_3);
                self.screencast(None);
            } else if self.btnp(KEY_8, None, None) {
                self.reset_key(KEY_8);
                self.integer_scale(!self.system.integer_scale_enabled);
            } else if self.btnp(KEY_9, None, None) {
                self.reset_key(KEY_9);
                self.screen_mode((self.system.screen_mode + 1) % NUM_SCREEN_TYPES);
            } else if self.btnp(KEY_0, None, None) {
                self.reset_key(KEY_0);
                self.perf_monitor(!self.system.perf_monitor_enabled);
            } else if self.btnp(KEY_R, None, None) {
                self.reset_key(KEY_RETURN);
                self.reset();
            } else if self.btnp(KEY_RETURN, None, None) {
                self.reset_key(KEY_RETURN);
                self.fullscreen(!platform::is_fullscreen());
            }
        } else if self.btn(GAMEPAD1_BUTTON_A)
            && self.btn(GAMEPAD1_BUTTON_B)
            && self.btn(GAMEPAD1_BUTTON_X)
            && self.btn(GAMEPAD1_BUTTON_Y)
        {
            if self.btnp(GAMEPAD1_BUTTON_BACK, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_BACK);
                self.reset();
            } else if self.btnp(GAMEPAD1_BUTTON_DPAD_LEFT, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_UP);
                self.integer_scale(!self.system.integer_scale_enabled);
            } else if self.btnp(GAMEPAD1_BUTTON_DPAD_RIGHT, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_DOWN);
                self.screen_mode((self.system.screen_mode + 1) % NUM_SCREEN_TYPES);
            } else if self.btnp(GAMEPAD1_BUTTON_DPAD_UP, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_LEFT);
                self.perf_monitor(!self.system.perf_monitor_enabled);
            } else if self.btnp(GAMEPAD1_BUTTON_DPAD_DOWN, None, None) {
                self.reset_key(GAMEPAD1_BUTTON_DPAD_RIGHT);
                self.fullscreen(!platform::is_fullscreen());
            }
        }
    }

    fn update_screen_params(&mut self) {
        let (window_width, window_height) = platform::window_size();

        if self.system.integer_scale_enabled {
            self.system.screen_scale = f32::max(
                f32::min(
                    (window_width as f32 / *pyxel::width() as f32) as i32 as f32,
                    (window_height as f32 / *pyxel::height() as f32) as i32 as f32,
                ),
                1.0,
            );
        } else {
            self.system.screen_scale = f32::max(
                f32::min(
                    window_width as f32 / *pyxel::width() as f32,
                    window_height as f32 / *pyxel::height() as f32,
                ),
                1.0,
            );
        }

        self.system.screen_x =
            (window_width as i32 - (*pyxel::width() as f32 * self.system.screen_scale) as i32) / 2;
        self.system.screen_y = (window_height as i32
            - (*pyxel::height() as f32 * self.system.screen_scale) as i32)
            / 2;
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.system.update_profiler.start(platform::ticks());

        self.process_events();

        if self.system.paused {
            return;
        }

        self.check_special_input();

        if let Some(callback) = callback {
            callback.update(self);
            self.system.update_profiler.end(platform::ticks());
        }
    }

    fn draw_perf_monitor(&self) {
        if !self.system.perf_monitor_enabled {
            return;
        }

        let screen = pyxel::screen();
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
        let x = *pyxel::mouse_x();
        let y = *pyxel::mouse_y();

        platform::set_mouse_visible(
            x < 0 || x >= *pyxel::width() as i32 || y < 0 || y >= *pyxel::height() as i32,
        );

        if !self.is_mouse_visible() {
            return;
        }

        let width = pyxel::cursor_image().width() as i32;
        let height = pyxel::cursor_image().height() as i32;

        if x <= -width
            || x >= *pyxel::width() as i32
            || y <= -height
            || y >= *pyxel::height() as i32
        {
            return;
        }

        let screen = pyxel::screen();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;

        screen.clip0();
        screen.camera0();
        unsafe {
            screen.blt(
                x as f32,
                y as f32,
                std::ptr::from_mut(pyxel::cursor_image()),
                0.0,
                0.0,
                width as f32,
                height as f32,
                Some(0),
                None,
                None,
            );
        }

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.system.draw_profiler.start(platform::ticks());

        if self.system.paused {
            return;
        }

        self.update_screen_params();

        if let Some(callback) = callback {
            callback.draw(self);
        }

        self.system.window_watcher.update();
        self.draw_perf_monitor();
        self.draw_cursor();
        self.render_screen();
        self.capture_screen();

        self.system.draw_profiler.end(platform::ticks());
    }
}
