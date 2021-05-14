use std::cmp::min;

use crate::canvas::Canvas;
use crate::event::Event;
use crate::graphics::Graphics;
use crate::platform::Platform;
use crate::settings::{BACKGROUND_COLOR, DEFAULT_CAPTION, DEFAULT_FPS, MAX_FRAME_SKIP_COUNT};

pub struct System<T: Platform> {
    platform: T,

    target_fps: u32,

    elapsed_frame_count: u32,
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

impl<T: Platform> System<T> {
    pub fn new(
        platform: T,
        pwidth: u32,
        height: u32,
        caption: Option<&str>,
        fps: Option<u32>,
    ) -> System<T> {
        let caption = caption.unwrap_or(DEFAULT_CAPTION);
        let fps = fps.unwrap_or(DEFAULT_FPS);

        let one_frame_time = 1000.0 / fps as f64;
        let next_update_time = platform.ticks() as f64;

        System::<T> {
            platform: platform,

            target_fps: fps,

            elapsed_frame_count: 0,
            one_frame_time: one_frame_time,
            next_update_time: next_update_time,
            waiting_update_count: 0,

            should_quit: false,
            disable_frame_skip_once: false,
            performance_monitor_enabled: false,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.platform.width()
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.platform.height()
    }

    #[inline]
    pub fn frame_count(&self) -> u32 {
        self.elapsed_frame_count
    }

    #[inline]
    pub fn caption(&self) -> &str {
        self.platform.caption()
    }

    #[inline]
    pub fn set_caption(&mut self, caption: &str) {
        self.platform.set_caption(caption);
    }

    #[inline]
    pub fn is_fullscreen(&self) -> bool {
        true
    }

    #[inline]
    pub fn set_fullscreen(&self, is_fullscreen: bool) {
        //
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

    #[inline]
    fn process_events(&mut self) {
        while let Some(event) = self.platform.poll_event() {
            match event {
                Event::Quit => self.should_quit = true,

                _ => {}
            }
        }

        /*
        // TODO
        for event in self.sdl_event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.should_quit = true,

                _ => {}
            }
        }
        */

        /*
        SDL_Event event;
        bool window_should_close = false;

        while (SDL_PollEvent(&event)) {
            switch (event.type) {
            case SDL_WINDOWEVENT:
                if (event.window.event == SDL_WINDOWEVENT_MOVED ||
                    event.window.event == SDL_WINDOWEVENT_RESIZED) {
                UpdateWindowInfo();
                }
                break;

            case SDL_MOUSEWHEEL:
                mouse_wheel_ += event.wheel.y;
                break;

            case SDL_DROPFILE:
                drop_file_ = event.drop.file;
                break;

            case SDL_QUIT:
                window_should_close = true;
                break;
            }
        }

        return window_should_close;
        }
        */
    }

    /*
        int32_t scale,
        int32_t fps,
        int32_t quit_key,
        bool is_fullscreen);

        bool Quit();
        bool FlipScreen();
        void ShowScreen();

        std::string DropFile() const { return drop_file_; }
        void SetCaption(const std::string& caption);

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
    pub fn _should_update(&self) -> bool {
        self.waiting_update_count > 0
    }

    #[inline]
    pub fn _should_quit(&self) -> bool {
        self.should_quit
    }

    #[inline]
    pub fn _init_run_status(&mut self) {
        self.next_update_time = self.platform.ticks() as f64 + self.one_frame_time;
        self.should_quit = false;
        self.disable_frame_skip_once = true;
        self.elapsed_frame_count = 0;
    }

    #[inline]
    pub fn _prepare_for_update(&mut self) {
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
    pub fn _start_update(&mut self) {
        // TODO: update_profiler_.Start();

        self.process_events();

        // TODO: drop_file_ = window_->GetDropFile();
        // TODO: input_->Update(window_, frame_count_);
        // TODO: CheckSpecialInput();
    }

    #[inline]
    pub fn end_update(&mut self) {
        // TODO: update_profiler_.End();

        if self.waiting_update_count > 0 {
            self.waiting_update_count -= 1;
            self.elapsed_frame_count += 1;
        }
    }

    #[inline]
    pub fn _start_draw(&mut self) {
        //
    }

    #[inline]
    pub fn _end_draw(&mut self, graphics: &Graphics) {
        self.platform
            .render_screen(graphics.screen(), BACKGROUND_COLOR);

        self.elapsed_frame_count += 1;
    }
}

macro_rules! update_frame {
    ($self: expr, $callback: expr, $context: expr, $quit: stmt) => {
        $self._start_update();

        if $self._should_quit() {
            $quit
        }

        $callback.update($context);

        if $self._should_quit() {
            $quit
        }

        $self.end_update();
    };
}

macro_rules! draw_frame {
    ($self: expr, $graphics: expr, $callback: expr, $context: expr) => {
        $self._start_draw();

        $callback.draw($context);

        $self._end_draw($graphics);
    };
}

macro_rules! run {
    ($self: expr, $graphics: expr, $callback: expr, $context: expr) => {
        'main_loop: loop {
            $self._init_run_status();

            update_frame!($self, $callback, $context, break 'main_loop);
            draw_frame!($self, $graphics, $callback, $context);

            loop {
                $self._prepare_for_update();

                while $self._should_update() {
                    update_frame!($self, $callback, $context, break 'main_loop);
                }

                draw_frame!($self, $graphics, $callback, $context);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    //
}
