use std::cmp::min;
use std::sync::{Arc, Mutex};

use crate::event::Event;
use crate::graphics::Graphics;
use crate::input::Input;
use crate::platform::{AudioCallback, Platform};
use crate::rectarea::Rectarea;
use crate::settings::{
    BACKGROUND_COLOR, DEFAULT_FPS, DEFAULT_SCALE, DEFAULT_TITLE, MAX_FRAME_SKIP_COUNT,
};

pub struct System {
    platform: Platform,

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

impl System {
    #[inline]
    pub fn new(
        width: u32,
        height: u32,
        title: Option<&str>,
        scale: Option<u32>,
        fps: Option<u32>,
    ) -> System {
        let title = title.unwrap_or(DEFAULT_TITLE);
        let scale = scale.unwrap_or(DEFAULT_SCALE);
        let platform = Platform::new(title, width, height, scale);

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

    #[inline]
    pub fn window_width(&self) -> u32 {
        self.platform.window_size().0
    }

    #[inline]
    pub fn window_height(&self) -> u32 {
        self.platform.window_size().1
    }

    #[inline]
    pub fn window_title(&self) -> &str {
        self.platform.window_title()
    }

    #[inline]
    pub fn set_window_title(&mut self, title: &str) {
        self.platform.set_window_title(title);
    }

    #[inline]
    pub fn is_fullscreen(&self) -> bool {
        true
    }

    #[inline]
    pub fn set_fullscreen(&self, is_fullscreen: bool) {
        //
    }

    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }

    /*
    void Window::SetupWindowIcon() const {
    SDL_Surface* surface = SDL_CreateRGBSurfaceWithFormat(
        0, ICON_WIDTH * ICON_SCALE, ICON_HEIGHT * ICON_SCALE, 32,
        SDL_PIXELFORMAT_RGBA8888);

    Image* image = new Image(ICON_WIDTH, ICON_HEIGHT);
    image->SetData(0, 0, ICON_DATA);

    int32_t** src_data = image->Data();
    uint32_t* dst_data = reinterpret_cast<uint32_t*>(surface->pixels);

    for (int32_t i = 0; i < ICON_HEIGHT; i++) {
        int32_t index = ICON_WIDTH * i;

        for (int32_t j = 0; j < ICON_WIDTH; j++) {
        int32_t color = src_data[i][j];
        uint32_t argb = color == 0 ? 0 : (DEFAULT_PALETTE[color] << 8) + 0xff;

        for (int32_t y = 0; y < ICON_SCALE; y++) {
            int32_t index = (ICON_WIDTH * (i * ICON_SCALE + y) + j) * ICON_SCALE;

            for (int32_t x = 0; x < ICON_SCALE; x++) {
            dst_data[index + x] = argb;
            }
        }
        }
    }

    SDL_SetWindowIcon(window_, surface);
    SDL_FreeSurface(surface);

    delete image;
    }

    void Window::UpdateWindowInfo() {
    SDL_GetWindowPosition(window_, &window_x_, &window_y_);

    int32_t window_width, window_height;
    SDL_GetWindowSize(window_, &window_width, &window_height);

    screen_scale_ =
        Min(window_width / screen_width_, window_height / screen_height_);
    screen_x_ = (window_width - screen_width_ * screen_scale_) / 2;
    screen_y_ = (window_height - screen_height_ * screen_scale_) / 2;
    }

    void Window::ToggleFullscreen() {
    is_fullscreen_ = !is_fullscreen_;

    SDL_SetWindowFullscreen(window_, is_fullscreen_ ? SDL_WINDOW_FULLSCREEN_DESKTOP : 0);
    }
    */

    /*
        int32_t scale,
        int32_t fps,
        int32_t quit_key,
        bool is_fullscreen);

        bool Quit();
        bool FlipScreen();
        void ShowScreen();

    private:
        void DrawFrame(void (*draw)(), int32_t update_frame_count);
        void DrawPerformanceMonitor();
        void DrawMouseCursor();
    */

    #[inline]
    pub fn flip(&mut self) {
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

    #[inline]
    pub fn show(&mut self) {
        /*
        is_loop_running_ = true;

        while (true) {
            if (FlipScreen()) {
            break;
            }
        }
        */
    }

    #[inline]
    pub(crate) fn platform_mut(&mut self) -> &mut Platform {
        &mut self.platform
    }

    #[inline]
    fn process_events(&mut self, input: &mut Input) {
        let window_pos = self.platform.window_pos();
        let window_size = self.platform.window_size();
        let window_rect =
            Rectarea::with_size(window_pos.0, window_pos.1, window_size.0, window_size.1);

        input.start_update(self.frame_count, window_rect);

        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => self.should_quit = true,

                Event::WindowMoved { x, y } => {
                    //
                }

                Event::WindowResized { width, height } => {
                    //
                }

                _ => input.process_event(event),
            }
        }

        input.end_update();

        /*
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => self.should_quit = true,
        */
    }

    #[inline]
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
    #[inline]
    pub(crate) fn should_update(&self) -> bool {
        self.waiting_update_count > 0
    }

    #[inline]
    pub(crate) fn should_quit(&self) -> bool {
        self.should_quit
    }

    #[inline]
    pub(crate) fn init_run_states(&mut self) {
        self.next_update_time = self.platform.ticks() as f64 + self.one_frame_time;
        self.should_quit = false;
        self.disable_frame_skip_once = true;
        self.frame_count = 0;
    }

    #[inline]
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

    #[inline]
    pub(crate) fn start_update(&mut self, input: &mut Input) {
        // TODO: update_profiler_.Start();

        self.process_events(input);

        // TODO: drop_file_ = window_->GetDropFile();
        // TODO: input_->Update(window_, frame_count_);
        // TODO: CheckSpecialInput();
    }

    #[inline]
    pub(crate) fn end_update(&mut self) {
        // TODO: update_profiler_.End();

        if self.waiting_update_count > 0 {
            self.waiting_update_count -= 1;
            self.frame_count += 1;
        }
    }

    #[inline]
    pub(crate) fn start_draw(&mut self) {
        //
    }

    #[inline]
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
