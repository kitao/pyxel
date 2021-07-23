use std::cmp::min;

use crate::event::Event;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::key::{Key, KEY_0, KEY_1, KEY_2, KEY_3, KEY_ALT, KEY_RETURN};
use crate::platform::Platform;
use crate::profiler::Profiler;
use crate::recorder::Recorder;
use crate::settings::{
    BACKGROUND_COLOR, DEFAULT_FPS, DEFAULT_QUIT_KEY, DEFAULT_SCALE, DEFAULT_TITLE,
    MAX_FRAME_SKIP_COUNT, MEASURE_FRAME_COUNT,
};

pub struct System<T> {
    platform: T,

    frame_count: u32,
    one_frame_time: f64,
    next_update_time: f64,
    waiting_update_count: u32,
    disable_frame_skip_once: bool,

    quit_key: Key,
    should_quit: bool,
    recorder: Recorder,
    drop_file: String,

    fps_profiler: Profiler,
    update_profiler: Profiler,
    draw_profiler: Profiler,
    perf_monitor_enabled: bool,
}

pub trait SystemCallback<T> {
    fn update(&mut self, context: &mut T);
    fn draw(&mut self, context: &mut T);
}

impl<T: Platform> System<T> {
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        scale: Option<u32>,
        fps: Option<u32>,
        quit_key: Option<Key>,
    ) -> System<T> {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let scale = scale.unwrap_or(DEFAULT_SCALE);
        let platform = T::new(title, width, height, scale);

        let fps = fps.unwrap_or(DEFAULT_FPS);
        let one_frame_time = 1000.0 / fps as f64;
        let next_update_time = platform.ticks() as f64;

        System {
            platform: platform,

            frame_count: 0,
            one_frame_time: one_frame_time,
            next_update_time: next_update_time,
            waiting_update_count: 0,
            disable_frame_skip_once: false,

            quit_key: quit_key.unwrap_or(DEFAULT_QUIT_KEY),
            should_quit: false,
            recorder: Recorder::new(),
            drop_file: "".to_string(),

            fps_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            update_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            draw_profiler: Profiler::new(MEASURE_FRAME_COUNT),
            perf_monitor_enabled: false,
        }
    }

    pub fn platform_mut(&mut self) -> &mut T {
        &mut self.platform
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.platform.set_window_title(title);
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn flip_screen(&mut self) {
        /*
        next_update_time_ += one_frame_time_;

        fps_profiler_.End();
        fps_profiler_.Start();

        if (UpdateFrame(nullptr)) {
            return true;
        }

        DrawFrame(nullptr, 1);

        return false;
        */
    }

    pub fn show_screen(&mut self) {
        /*
        is_loop_running_ = true;

        while (true) {
            if (FlipScreen()) {
            break;
            }
        }
        */
    }

    pub fn set_fullscreen(&mut self, is_fullscreen: bool) {
        self.platform.set_fullscreen(is_fullscreen);
    }

    fn process_events(&mut self, input: &mut Input) {
        input.start_update(self.frame_count);

        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => {
                    self.should_quit = true;
                }
                _ => {
                    input.process_event(event);
                }
            }
        }

        input.end_update();
    }

    fn wait_for_update_time(&mut self) -> i32 {
        loop {
            let sleep_time = self.next_update_time - self.platform.ticks() as f64;

            if sleep_time <= 0.0 {
                return sleep_time as i32;
            }

            self.platform.delay((sleep_time / 2.0) as u32);
        }
    }

    fn check_special_input(&mut self, input: &Input) {
        if input.is_key_on(KEY_ALT) {
            if input.is_key_pressed(KEY_RETURN, None, None) {
                //window_->ToggleFullscreen();
            }

            if input.is_key_pressed(KEY_0, None, None) {
                self.perf_monitor_enabled = !self.perf_monitor_enabled;
            }

            if input.is_key_pressed(KEY_1, None, None) {
                //recorder_->SaveScreenshot();
                self.disable_frame_skip_once = true;
            }

            if input.is_key_pressed(KEY_2, None, None) {
                //recorder_->ResetScreenCapture();
            }

            if input.is_key_pressed(KEY_3, None, None) {
                /*
                recorder_->SaveScreenCapture();
                is_update_suspended_ = true;
                */
            }
        }

        if input.is_key_pressed(self.quit_key, None, None) {
            self.should_quit = true;
        }
    }

    //
    // methods for run macro
    //
    pub(crate) fn should_update(&self) -> bool {
        self.waiting_update_count > 0
    }

    pub(crate) fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub(crate) fn init_run_state(&mut self) {
        self.next_update_time = self.platform.ticks() as f64 + self.one_frame_time;
        self.should_quit = false;
        self.disable_frame_skip_once = true;
        self.frame_count = 0;
    }

    pub(crate) fn prepare_for_update(&mut self) {
        let sleep_time = self.wait_for_update_time();

        let ticks = self.platform.ticks();
        self.fps_profiler.end(ticks);
        self.fps_profiler.start(ticks);

        if self.disable_frame_skip_once {
            self.disable_frame_skip_once = false;
            self.waiting_update_count = 1;
            self.next_update_time = self.platform.ticks() as f64 + self.one_frame_time;
        } else {
            self.waiting_update_count = min(
                (-sleep_time as f64 / self.one_frame_time) as u32,
                MAX_FRAME_SKIP_COUNT,
            ) + 1;
            self.next_update_time += self.one_frame_time * self.waiting_update_count as f64;
        }
    }

    pub(crate) fn start_update(&mut self, input: &mut Input) {
        self.update_profiler.start(self.platform.ticks());

        self.process_events(input);

        // TODO: drop_file_ = window_->GetDropFile();

        self.check_special_input(input);
    }

    pub(crate) fn end_update(&mut self) {
        self.update_profiler.end(self.platform.ticks());

        if self.waiting_update_count > 0 {
            self.waiting_update_count -= 1;
            self.frame_count += 1;
        }
    }

    pub(crate) fn start_draw(&mut self) {
        self.draw_profiler.start(self.platform.ticks());
    }

    pub(crate) fn end_draw(&mut self, graphics: &Graphics) {
        self.platform
            .render_screen(graphics.screen(), BACKGROUND_COLOR);

        self.draw_profiler.end(self.platform.ticks());

        self.frame_count += 1;
    }
}

macro_rules! update_frame {
    ($self: expr, $callback: expr, $quit: stmt) => {
        $self.system.start_update(&mut $self.input);

        if $self.system.should_quit() {
            $quit
        }

        $callback.update($self);

        if $self.system.should_quit() {
            $quit
        }

        $self.system.end_update();
    };
}

macro_rules! draw_frame {
    ($self: expr, $callback: expr) => {
        $self.system.start_draw();

        $callback.draw($self);

        $self.system.end_draw(&$self.graphics);
    };
}

macro_rules! run {
    ($self: expr, $callback: expr) => {
        'main_loop: loop {
            $self.system.init_run_state();

            update_frame!($self, $callback, break 'main_loop);
            draw_frame!($self, $callback);

            loop {
                $self.system.prepare_for_update();

                while $self.system.should_update() {
                    update_frame!($self, $callback, break 'main_loop);
                }

                draw_frame!($self, $callback);
            }
        }
    };
}
