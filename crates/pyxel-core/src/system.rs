use std::cmp::min;

use crate::event::Event;
use crate::image::{Image, SharedImage};
use crate::input::Input;
use crate::key::{KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use crate::platform::Platform;
use crate::profiler::Profiler;
use crate::resource::Resource;
use crate::settings::{BACKGROUND_COLOR, MAX_SKIP_FRAMES, NUM_MEASURE_FRAMES};
use crate::types::Key;
use crate::utils::simplify_string;

pub trait PyxelCallback {
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct System {
    one_frame_ms: f64,
    next_update_ms: f64,
    disable_next_frame_skip: bool,
    frame_count: u32,
    quit_key: Key,
    is_paused: bool,
    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    enable_perf_monitor: bool,
}

unsafe_singleton!(System);

impl System {
    pub fn init(fps: u32, quit_key: Key) {
        Self::set_instance(Self {
            one_frame_ms: 1000.0 / fps as f64,
            next_update_ms: 0.0,
            disable_next_frame_skip: true,
            frame_count: 0,
            quit_key,
            is_paused: false,
            fps_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            update_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            draw_profiler: Profiler::new(NUM_MEASURE_FRAMES),
            enable_perf_monitor: false,
        });
    }

    pub fn disable_next_frame_skip(&mut self) {
        self.disable_next_frame_skip = true;
    }

    fn process_frame(&mut self, callback: &mut dyn PyxelCallback) {
        let tick_count = Platform::instance().tick_count();
        let wait_ms = self.next_update_ms - tick_count as f64;
        if wait_ms > 0.0 {
            return;
        }
        if self.frame_count == 0 {
            self.next_update_ms = tick_count as f64 + self.one_frame_ms;
        } else {
            self.fps_profiler.end(tick_count);
            self.fps_profiler.start(tick_count);
            let update_count: u32;
            if self.disable_next_frame_skip {
                update_count = 1;
                self.disable_next_frame_skip = false;
                self.next_update_ms = Platform::instance().tick_count() as f64 + self.one_frame_ms;
            } else {
                update_count = min((-wait_ms / self.one_frame_ms) as u32, MAX_SKIP_FRAMES) + 1;
                self.next_update_ms += self.one_frame_ms * update_count as f64;
            }
            for _ in 1..update_count {
                self.update_frame(Some(callback));
                self.frame_count += 1;
            }
        }
        self.update_frame(Some(callback));
        self.draw_frame(Some(callback));
        self.frame_count += 1;
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.update_profiler
            .start(Platform::instance().tick_count());
        self.process_events();
        if self.is_paused {
            return;
        }
        self.check_special_input();
        if let Some(callback) = callback {
            callback.update();
        }
        self.update_profiler.end(Platform::instance().tick_count());
    }

    fn process_events(&mut self) {
        Input::instance().reset_input_states();
        while let Some(event) = Platform::instance().poll_event() {
            match event {
                Event::Quit => {
                    crate::quit();
                }
                Event::Shown => {
                    self.is_paused = false;
                    self.disable_next_frame_skip = true;
                    Platform::instance().resume_audio();
                }
                Event::Hidden => {
                    self.is_paused = true;
                    Platform::instance().pause_audio();
                }
                _ => {
                    if !self.is_paused {
                        Input::instance().process_input_event(event, self.frame_count);
                    }
                }
            }
        }
    }

    fn check_special_input(&mut self) {
        if crate::btn(KEY_ALT) {
            if crate::btnp(KEY_RETURN, None, None) {
                crate::fullscreen(!crate::is_fullscreen());
            }
            if crate::btnp(KEY_0, None, None) {
                self.enable_perf_monitor = !self.enable_perf_monitor;
            }
            if crate::btnp(KEY_1, None, None) {
                crate::screenshot(None);
            }
            if crate::btnp(KEY_2, None, None) {
                crate::reset_capture();
            }
            if crate::btnp(KEY_3, None, None) {
                crate::screencast(None);
            }
        }
        if crate::btnp(self.quit_key, None, None) {
            crate::quit();
        }
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        if self.is_paused {
            return;
        }
        self.draw_profiler.start(Platform::instance().tick_count());
        if let Some(callback) = callback {
            callback.draw();
        }
        self.draw_perf_monitor();
        self.draw_cursor();
        Platform::instance().render_screen(
            &crate::screen().lock().canvas.data,
            &*crate::colors().lock(),
            BACKGROUND_COLOR,
        );
        Resource::instance().capture_screen(
            &crate::screen().lock().canvas.data,
            &crate::colors().lock(),
            self.frame_count,
        );
        self.draw_profiler.end(Platform::instance().tick_count());
    }

    fn draw_perf_monitor(&mut self) {
        if !self.enable_perf_monitor {
            return;
        }
        let screen = crate::screen();
        let mut screen = screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette1 = screen.palette[1];
        let palette2 = screen.palette[2];
        screen.clip0();
        screen.camera0();
        screen.pal(1, 1);
        screen.pal(2, 9);

        let fps = format!("{:.*}", 2, self.fps_profiler.average_fps());
        screen.text(1.0, 0.0, &fps, 1);
        screen.text(0.0, 0.0, &fps, 2);

        let update_time = format!("{:.*}", 2, self.update_profiler.average_time());
        screen.text(1.0, 6.0, &update_time, 1);
        screen.text(0.0, 6.0, &update_time, 2);

        let draw_time = format!("{:.*}", 2, self.draw_profiler.average_time());
        screen.text(1.0, 12.0, &draw_time, 1);
        screen.text(0.0, 12.0, &draw_time, 2);

        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.pal(1, palette1);
        screen.pal(2, palette2);
    }

    fn draw_cursor(&mut self) {
        let x = crate::mouse_x();
        let y = crate::mouse_y();
        Platform::instance().show_cursor(
            x < 0 || x >= crate::width() as i32 || y < 0 || y >= crate::height() as i32,
        );
        if !Input::instance().is_mouse_visible() {
            return;
        }
        let width = crate::cursor().lock().width() as i32;
        let height = crate::cursor().lock().height() as i32;
        if x <= -width || x >= crate::width() as i32 || y <= -height || y >= crate::height() as i32
        {
            return;
        }
        let screen = crate::screen();
        let mut screen = screen.lock();
        let clip_rect = screen.canvas.clip_rect;
        let camera_x = screen.canvas.camera_x;
        let camera_y = screen.canvas.camera_y;
        let palette = screen.palette;
        screen.clip0();
        screen.camera0();
        screen.pal0();
        screen.blt(
            x as f64,
            y as f64,
            crate::cursor(),
            0.0,
            0.0,
            width as f64,
            height as f64,
            Some(0),
        );
        screen.canvas.clip_rect = clip_rect;
        screen.canvas.camera_x = camera_x;
        screen.canvas.camera_y = camera_y;
        screen.palette = palette;
    }
}

pub fn width() -> u32 {
    Platform::instance().screen_width()
}

pub fn height() -> u32 {
    Platform::instance().screen_height()
}

pub fn frame_count() -> u32 {
    System::instance().frame_count
}

pub fn title(title: &str) {
    Platform::instance().set_title(title);
}

pub fn icon(data_str: &[&str], scale: u32) {
    let width = simplify_string(data_str[0]).len() as u32;
    let height = data_str.len() as u32;
    let image = Image::new(width, height);
    image.lock().set(0, 0, data_str);
    Platform::instance().set_icon(&image.lock().canvas.data, &*crate::colors().lock(), scale);
}

pub fn is_fullscreen() -> bool {
    Platform::instance().is_fullscreen()
}

pub fn fullscreen(is_fullscreen: bool) {
    Platform::instance().set_fullscreen(is_fullscreen);
}

pub fn run<T: PyxelCallback>(mut callback: T) {
    let main_loop = move || {
        System::instance().process_frame(&mut callback);
    };
    Platform::instance().run(main_loop);
}

pub fn show() {
    let image = Image::new(crate::width(), crate::height());
    image.lock().blt(
        0.0,
        0.0,
        crate::screen(),
        0.0,
        0.0,
        crate::width() as f64,
        crate::height() as f64,
        None,
    );
    pub struct App {
        image: SharedImage,
    }
    impl PyxelCallback for App {
        fn update(&mut self) {}
        fn draw(&mut self) {
            crate::screen().lock().blt(
                0.0,
                0.0,
                self.image.clone(),
                0.0,
                0.0,
                crate::width() as f64,
                crate::height() as f64,
                None,
            );
        }
    }
    crate::run(App { image });
}

pub fn quit() {
    Platform::instance().quit();
}
