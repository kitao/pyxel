use super::{Pyxel, PyxelCallback};

use std::cmp::min;

use crate::event::Event;
use crate::key::{KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use crate::platform::Platform;
use crate::profiler::Profiler;
use crate::recorder::Recorder;
use crate::settings::{BACKGROUND_COLOR, MAX_FRAME_SKIP_COUNT, MEASURE_FRAME_COUNT};
use crate::types::Key;

pub struct System {
    one_frame_time: f64,
    next_update_time: f64,
    disable_next_frame_skip: bool,

    quit_key: Key,
    should_quit: bool,
    recorder: Recorder,

    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
}

impl System {
    pub fn new(fps: u32, quit_key: Key) -> System {
        System {
            one_frame_time: 1000.0 / fps as f64,
            next_update_time: 0.0,
            disable_next_frame_skip: true,

            quit_key: quit_key,
            should_quit: false,
            recorder: Recorder::new(),

            fps_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            update_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            draw_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            perf_monitor_enabled: false,
        }
    }

    pub fn reset_start_time(&mut self, tick_count: u32) {
        self.next_update_time = tick_count as f64 - self.one_frame_time;
    }
}

impl Pyxel {
    pub fn title(&mut self, title: &str) {
        self.platform.set_title(title);
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

        self.draw_frame(Some(callback), 0);

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
                    self.platform.tick_count() as f64 + self.system.one_frame_time as f64;
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
                    self.frame_count += 1;
                }
            }

            self.draw_frame(Some(callback), update_count);
        }
    }

    pub fn quit(&mut self) {
        self.system.should_quit = true;
    }

    pub fn flip(&mut self) -> bool {
        self.system.next_update_time += self.system.next_update_time;

        let tick_count = self.platform.tick_count();
        self.system.fps_profiler.end(tick_count);
        self.system.fps_profiler.start(tick_count);

        if self.update_frame(None) {
            return true;
        }

        self.draw_frame(None, 1);

        false
    }

    pub fn show(&mut self) {
        loop {
            if self.update_frame(None) {
                break;
            }

            self.draw_frame(None, 1);
        }
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
        self.start_input_event();

        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => {
                    self.system.should_quit = true;
                }
                _ => {
                    self.process_input_event(event);
                }
            }
        }

        self.end_input_event();
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
                //recorder_->SaveScreenshot();
                self.system.disable_next_frame_skip = true;
            }

            if self.btnp(KEY_2, None, None) {
                //recorder_->ResetScreenCapture();
            }

            if self.btnp(KEY_3, None, None) {
                /*
                recorder_->SaveScreenCapture();
                is_update_suspended_ = true;
                */
            }
        }

        if self.btnp(self.system.quit_key, None, None) {
            self.system.should_quit = true;
        }
    }

    fn draw_frame(&mut self, callback: Option<&mut dyn PyxelCallback>, elapsed_frame_count: u32) {
        self.system.draw_profiler.start(self.platform.tick_count());

        if let Some(callback) = callback {
            callback.draw(self);
        }

        self.draw_perf_monitor();
        self.draw_mouse_cursor();
        self.platform
            .render_screen(&self.screen.borrow(), &self.colors, BACKGROUND_COLOR);
        self.system
            .recorder
            .capture_screen(&self.screen.borrow(), elapsed_frame_count);

        self.system.draw_profiler.end(self.platform.tick_count());

        self.frame_count += 1;
    }

    fn draw_perf_monitor(&mut self) {
        if !self.system.perf_monitor_enabled {
            return;
        }

        /*
        STORE_CLIP_AREA_AND_PALETTE();

        char buf[16];

        snprintf(buf, sizeof(buf), "%.2f", fps_profiler_.AverageFPS());
        graphics_->DrawText(1, 0, buf, 1);
        graphics_->DrawText(0, 0, buf, 9);

        snprintf(buf, sizeof(buf), "%.2f", update_profiler_.AverageTime());
        graphics_->DrawText(1, 6, buf, 1);
        graphics_->DrawText(0, 6, buf, 9);

        snprintf(buf, sizeof(buf), "%.2f", draw_profiler_.AverageTime());
        graphics_->DrawText(1, 12, buf, 1);
        graphics_->DrawText(0, 12, buf, 9);

        RESTORE_CLIP_AREA_AND_PALETTE();
        */
    }

    fn draw_mouse_cursor(&mut self) {
        /*
        if (!input_->IsMouseVisible()) {
            return;
        }

        int32_t mouse_x = input_->MouseX();
        int32_t mouse_y = input_->MouseY();

        if (mouse_x < 0 || mouse_x >= window_->ScreenWidth() || mouse_y < 0 ||
            mouse_y >= window_->ScreenHeight()) {
            return;
        }

        STORE_CLIP_AREA_AND_PALETTE();

        graphics_->ResetPalette();
        graphics_->DrawImage(mouse_x, mouse_y, IMAGE_BANK_FOR_SYSTEM, MOUSE_CURSOR_X,
                            MOUSE_CURSOR_Y, MOUSE_CURSOR_WIDTH, MOUSE_CURSOR_HEIGHT,
                            1);

        RESTORE_CLIP_AREA_AND_PALETTE();
        */
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
}
