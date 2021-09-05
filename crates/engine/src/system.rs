use std::cmp::min;
use std::process::exit;

use crate::canvas::Canvas;
use crate::event::Event;
use crate::image::Image;
use crate::key::{KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use crate::platform::Platform;
use crate::profiler::Profiler;
use crate::settings::{BACKGROUND_COLOR, MAX_FRAME_SKIP_COUNT, MEASURE_FRAME_COUNT};
use crate::types::Key;
use crate::utils::simplify_string;
use crate::{Pyxel, PyxelCallback};

pub struct System {
    frame_count: u32,
    one_frame_time: f64,
    next_update_time: f64,
    disable_next_frame_skip: bool,

    quit_key: Key,
    should_quit: bool,

    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
}

impl System {
    pub fn new(fps: u32, quit_key: Key) -> System {
        System {
            frame_count: 0,
            one_frame_time: 1000.0 / fps as f64,
            next_update_time: -1.0,
            disable_next_frame_skip: true,

            quit_key,
            should_quit: false,

            fps_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            update_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            draw_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            perf_monitor_enabled: false,
        }
    }

    pub fn disable_next_frame_skip(&mut self) {
        self.disable_next_frame_skip = true;
    }
}

impl Pyxel {
    pub fn width(&self) -> u32 {
        self.screen.lock().width()
    }

    pub fn height(&self) -> u32 {
        self.screen.lock().height()
    }

    pub fn frame_count(&self) -> u32 {
        self.system.frame_count
    }

    pub fn title(&mut self, title: &str) {
        self.platform.set_title(title);
    }

    pub fn icon(&mut self, data_str: &[&str], scale: u32) {
        let width = simplify_string(data_str[0]).len() as u32;
        let height = data_str.len() as u32;
        let image = Image::new(width, height);
        image.lock().set(0, 0, data_str);

        self.platform.set_icon(image, &self.colors, scale);
    }

    pub fn fullscreen(&mut self) {
        self.platform.toggle_fullscreen();
    }

    pub fn run<T: PyxelCallback>(&mut self, callback: &mut T) {
        self.system.next_update_time =
            self.platform.tick_count() as f64 + self.system.one_frame_time;

        if self.update_frame(Some(callback)) {
            return;
        }

        self.draw_frame(Some(callback));

        loop {
            let sleep_time = self.wait_for_update_time();

            let tick_count = self.platform.tick_count();
            self.system.fps_profiler.end(tick_count);
            self.system.fps_profiler.start(tick_count);

            let update_count: u32;

            if self.system.disable_next_frame_skip {
                self.system.disable_next_frame_skip = false;

                update_count = 1;

                self.system.next_update_time =
                    self.platform.tick_count() as f64 + self.system.one_frame_time;
            } else {
                update_count = min(
                    (-sleep_time as f64 / self.system.one_frame_time) as u32,
                    MAX_FRAME_SKIP_COUNT,
                ) + 1;

                self.system.next_update_time += self.system.one_frame_time * update_count as f64;
            }

            for i in 0..update_count {
                if self.update_frame(Some(callback)) {
                    return;
                }

                if i < update_count - 1 {
                    self.system.frame_count += 1;
                }
            }

            self.draw_frame(Some(callback));
        }
    }

    pub fn show(&mut self) {
        loop {
            if self.update_frame(None) {
                break;
            }

            self.draw_frame(None);
        }
    }

    pub fn flip(&mut self) -> bool {
        if self.system.next_update_time < 0.0 {
            self.system.next_update_time = self.platform.tick_count() as f64;
        } else {
            self.wait_for_update_time();
        }

        self.system.next_update_time += self.system.one_frame_time;

        let tick_count = self.platform.tick_count();
        self.system.fps_profiler.end(tick_count);
        self.system.fps_profiler.start(tick_count);

        if self.update_frame(None) {
            exit(0);
        }

        self.draw_frame(None);

        false
    }

    pub fn quit(&mut self) {
        self.system.should_quit = true;
    }

    fn update_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) -> bool {
        self.system
            .update_profiler
            .start(self.platform.tick_count());

        self.process_events();
        self.check_special_input();

        if self.system.should_quit {
            return true;
        }

        if let Some(callback) = callback {
            callback.update(self);
        }

        if self.system.should_quit {
            return true;
        }

        self.system.update_profiler.end(self.platform.tick_count());

        false
    }

    fn process_events(&mut self) {
        self.input.reset_input_states();

        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => {
                    self.system.should_quit = true;
                }
                _ => {
                    self.input
                        .process_input_event(event, self.system.frame_count);
                }
            }
        }
    }

    fn check_special_input(&mut self) {
        if self.btn(KEY_ALT) {
            if self.btnp(KEY_RETURN, None, None) {
                self.platform.toggle_fullscreen();
            }

            if self.btnp(KEY_0, None, None) {
                self.system.perf_monitor_enabled = !self.system.perf_monitor_enabled;
            }

            if self.btnp(KEY_1, None, None) {
                self.screenshot();
            }

            if self.btnp(KEY_2, None, None) {
                self.reset_capture();
            }

            if self.btnp(KEY_3, None, None) {
                self.screencast();
            }
        }

        if self.btnp(self.system.quit_key, None, None) {
            self.system.should_quit = true;
        }
    }

    fn wait_for_update_time(&mut self) -> i32 {
        loop {
            let sleep_time = self.system.next_update_time - self.platform.tick_count() as f64;

            if sleep_time <= 0.0 {
                return sleep_time as i32;
            }

            self.platform.sleep((sleep_time / 2.0) as u32);
        }
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>) {
        self.system.draw_profiler.start(self.platform.tick_count());

        if let Some(callback) = callback {
            callback.draw(self);
        }

        self.draw_perf_monitor();
        self.draw_cursor();
        self.platform
            .render_screen(self.screen.clone(), &self.colors, BACKGROUND_COLOR);
        self.resource
            .capture_screen(self.screen.clone(), self.system.frame_count);

        self.system.draw_profiler.end(self.platform.tick_count());

        self.system.frame_count += 1;
    }

    fn draw_perf_monitor(&mut self) {
        if !self.system.perf_monitor_enabled {
            return;
        }

        let palette1 = self.screen.lock()._palette()[1];
        let palette2 = self.screen.lock()._palette()[2];

        self.pal(1, 1);
        self.pal(2, 9);

        let fps = format!("{:.*}", 2, self.system.fps_profiler.average_fps());
        self.text(1, 0, &fps, 1);
        self.text(0, 0, &fps, 2);

        let update_time = format!("{:.*}", 2, self.system.update_profiler.average_time());
        self.text(1, 6, &update_time, 1);
        self.text(0, 6, &update_time, 2);

        let draw_time = format!("{:.*}", 2, self.system.draw_profiler.average_time());
        self.text(1, 12, &draw_time, 1);
        self.text(0, 12, &draw_time, 2);

        self.pal(1, palette1);
        self.pal(2, palette2);
    }

    fn draw_cursor(&mut self) {
        let x = self.mouse_x();
        let y = self.mouse_y();

        self.platform
            .show_cursor(x < 0 || x >= self.width() as i32 || y < 0 || y >= self.height() as i32);

        if !self.input.is_mouse_visible() {
            return;
        }

        let width = self.cursor.lock().width() as i32;
        let height = self.cursor.lock().height() as i32;

        if x <= -width || x >= self.width() as i32 || y <= -height || y >= self.height() as i32 {
            return;
        }

        let mut screen = self.screen.lock();
        let clip_rect = screen._clip_rect();
        let palette = *screen._palette();

        screen.clip0();
        screen.pal0();

        screen.blt(x, y, self.cursor.clone(), 0, 0, width, height, Some(0));

        screen._set_clip_rect(clip_rect);
        screen._set_palette(&palette);
    }
}
