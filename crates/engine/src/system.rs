use std::cmp::min;

use crate::event::Event;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::platform::Platform;
use crate::rectarea::RectArea;
use crate::settings::{
    BACKGROUND_COLOR, DEFAULT_FPS, DEFAULT_SCALE, DEFAULT_TITLE, MAX_FRAME_SKIP_COUNT,
};

pub struct System<T> {
    platform: T,

    frame_count: u32,
    one_frame_time: f64,
    next_update_time: f64,
    waiting_update_count: u32,

    should_quit: bool,
    disable_frame_skip_once: bool,
    performance_monitor_enabled: bool,
    /*
        Recorder* recorder_;

        int32_t quit_key_;
        std::string drop_file_;

        Profiler fps_profiler_;
        Profiler update_profiler_;
        Profiler draw_profiler_;
        bool is_performance_monitor_on_;
    */
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

            should_quit: false,
            disable_frame_skip_once: false,
            performance_monitor_enabled: false,
        }
    }

    pub(crate) fn platform_mut(&mut self) -> &mut T {
        &mut self.platform
    }

    pub(crate) fn window_width(&self) -> u32 {
        self.platform.window_size().0
    }

    pub(crate) fn window_height(&self) -> u32 {
        self.platform.window_size().1
    }

    pub fn set_window_title(&mut self, title: &str) {
        self.platform.set_window_title(title);
    }

    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    pub fn quit(&mut self) {
        //
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

    fn process_events(&mut self, input: &mut Input) {
        let window_pos = self.platform.window_pos();
        let window_size = self.platform.window_size();
        let window_rect =
            RectArea::with_size(window_pos.0, window_pos.1, window_size.0, window_size.1);

        input.start_update(self.frame_count, window_rect);

        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => {
                    self.should_quit = true;
                }
                Event::WindowMoved { x, y } => {
                    //
                }
                Event::WindowResized { width, height } => {
                    //
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

    //
    // methods for run macro
    //
    pub(crate) fn should_update(&self) -> bool {
        self.waiting_update_count > 0
    }

    pub(crate) fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub(crate) fn init_run_states(&mut self) {
        self.next_update_time = self.platform.ticks() as f64 + self.one_frame_time;
        self.should_quit = false;
        self.disable_frame_skip_once = true;
        self.frame_count = 0;
    }

    pub(crate) fn prepare_for_update(&mut self) {
        let sleep_time = self.wait_for_update_time();

        // TODO: fps_profiler_.End();
        // TODO: fps_profiler_.Start();

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
        // TODO: update_profiler_.Start();

        self.process_events(input);

        // TODO: drop_file_ = window_->GetDropFile();
        // TODO: input_->Update(window_, frame_count_);
        // TODO: CheckSpecialInput();
    }

    pub(crate) fn end_update(&mut self) {
        // TODO: update_profiler_.End();

        if self.waiting_update_count > 0 {
            self.waiting_update_count -= 1;
            self.frame_count += 1;
        }
    }

    pub(crate) fn start_draw(&mut self) {
        //
    }

    pub(crate) fn end_draw(&mut self, graphics: &Graphics) {
        self.platform
            .render_screen(graphics.screen(), BACKGROUND_COLOR);

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
            $self.system.init_run_states();

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
