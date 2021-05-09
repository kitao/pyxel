use std::cmp::min;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl as SdlContext;
use sdl2::TimerSubsystem as SdlTimerSubsystem;
use sdl2::VideoSubsystem as SdlVideoSubsystem;

use crate::canvas::Canvas;
use crate::graphics::Graphics;
use crate::settings::*;

pub struct System {
    sdl_context: SdlContext,
    sdl_video: SdlVideoSubsystem,
    sdl_timer: SdlTimerSubsystem,
    sdl_canvas: SdlCanvas,
    sdl_texture: SdlTexture,
    sdl_event_pump: SdlEventPump,

    screen_width: u32,
    screen_height: u32,
    window_caption: String,
    target_fps: u32,

    elapsed_frame_count: i32,
    one_frame_time: f64,
    next_update_time: f64,
    waiting_update_count: u32,

    should_quit: bool,
    update_suspended: bool,
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

impl System {
    pub fn new(width: u32, height: u32, caption: Option<&str>, fps: Option<u32>) -> System {
        let caption = caption.unwrap_or(DEFAULT_CAPTION);
        let fps = fps.unwrap_or(DEFAULT_FPS);

        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_timer = sdl_context.timer().unwrap();
        let sdl_window = sdl_video
            .window(caption, width, height)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_texture_creator = sdl_canvas.texture_creator();
        let sdl_texture = sdl_texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        let one_frame_time = 1000.0 / fps as f64;
        let next_update_time = sdl_timer.ticks() as f64;

        /*
          if (screen_scale_ <= 0) {
            SDL_DisplayMode display_mode;
            SDL_GetDesktopDisplayMode(0, &display_mode);

            screen_scale_ = Max(
                Min(display_mode.w / screen_width_, display_mode.h / screen_height_) *
                    MAX_WINDOW_SIZE_RATIO,
                1.0f);
          }

          int32_t window_width = screen_width_ * screen_scale_;
          unt32_t window_height = screen_height_ * screen_scale_;

          window_ = SDL_CreateWindow(caption.c_str(), SDL_WINDOWPOS_CENTERED,
                                     SDL_WINDOWPOS_CENTERED, window_width,
                                     window_height, SDL_WINDOW_RESIZABLE);

          SDL_SetWindowMinimumSize(window_, screen_width_, screen_height_);

          SetupWindowIcon();
          UpdateWindowInfo();
        */

        System {
            sdl_context: sdl_context,
            sdl_video: sdl_video,
            sdl_timer: sdl_timer,
            sdl_canvas: sdl_canvas,
            sdl_texture: sdl_texture,
            sdl_event_pump: sdl_event_pump,

            screen_width: width,
            screen_height: height,
            window_caption: caption.to_string(),
            target_fps: fps,

            elapsed_frame_count: 0,
            one_frame_time: one_frame_time,
            next_update_time: next_update_time,
            waiting_update_count: 0,

            should_quit: false,
            update_suspended: false,
            performance_monitor_enabled: false,
        }
    }

    #[inline]
    pub fn width(&self) -> u32 {
        self.screen_width
    }

    #[inline]
    pub fn height(&self) -> u32 {
        self.screen_height
    }

    #[inline]
    pub fn frame_count(&self) -> i32 {
        self.elapsed_frame_count
    }

    #[inline]
    pub fn get_caption(&self) -> &String {
        &self.window_caption
    }

    #[inline]
    pub fn set_caption(&mut self, caption: &str) {
        self.window_caption = caption.to_string();
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
    fn process_events(&mut self) {
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
    fn wait_for_update_time(&mut self) -> i32 {
        loop {
            let sleep_time = self.next_update_time - self.sdl_timer.ticks() as f64;

            if sleep_time <= 0.0 {
                return sleep_time as i32;
            }

            self.sdl_timer.delay((sleep_time / 2.0) as u32);
        }
    }

    //
    // methods for run macro
    //
    #[inline]
    pub fn should_update(&self) -> bool {
        self.waiting_update_count > 0
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    #[inline]
    pub fn init_run_status(&mut self) {
        self.next_update_time = self.sdl_timer.ticks() as f64 + self.one_frame_time;
        self.should_quit = false;
        self.update_suspended = true;
        self.elapsed_frame_count = -1;
    }

    #[inline]
    pub fn prepare_for_update(&mut self) {
        let sleep_time = self.wait_for_update_time();

        // TODO: fps_profiler_.End();
        // TODO: fps_profiler_.Start();

        if self.update_suspended {
            self.update_suspended = false;
            self.waiting_update_count = 1;
            self.next_update_time = self.sdl_timer.ticks() as f64 + self.one_frame_time;
        } else {
            self.waiting_update_count = min(
                (-sleep_time as f64 / self.one_frame_time) as u32,
                MAX_FRAME_SKIP_COUNT,
            ) + 1;
            self.next_update_time += self.one_frame_time * self.waiting_update_count as f64;
        }
    }

    #[inline]
    pub fn start_update(&mut self) {
        self.elapsed_frame_count += 1;

        // TODO: update_profiler_.Start();

        self.process_events();

        // TODO: drop_file_ = window_->GetDropFile();
        // TODO: input_->Update(window_, frame_count_);
        // TODO: CheckSpecialInput();
    }

    #[inline]
    pub fn end_update(&mut self) {
        // TODO: update_profiler_.End();
    }

    #[inline]
    pub fn render_screen(&mut self, graphics: &Graphics) {
        let palette = graphics.screen().palette();
        let data = graphics.screen().data();
        let width = self.screen_width as usize;
        let height = self.screen_height as usize;

        self.sdl_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for i in 0..height {
                    for j in 0..width {
                        let offset = i * pitch + j * 3;
                        let color = palette.get_display_color(data[i][j]);

                        buffer[offset] = ((color >> 16) & 0xff) as u8;
                        buffer[offset + 1] = ((color >> 8) & 0xff) as u8;
                        buffer[offset + 2] = (color & 0xff) as u8;
                    }
                }
            })
            .unwrap();

        self.sdl_canvas.set_draw_color(SdlColor::RGB(
            ((BACKGROUND_COLOR >> 16) & 0xff) as u8,
            ((BACKGROUND_COLOR >> 8) & 0xff) as u8,
            (BACKGROUND_COLOR & 0xff) as u8,
        ));
        self.sdl_canvas.clear();
        self.sdl_canvas
            .copy(&self.sdl_texture, None, None)
            .expect("Render failed");
        self.sdl_canvas.present();
    }
}

macro_rules! run {
    ($system: expr, $graphics: expr, $callback: expr, $context: expr) => {
        'main_loop: loop {
            $system.init_run_status();

            // update frame
            $system.start_update();
            if $system.should_quit() {
                break 'main_loop;
            }
            $callback.update($context);
            if $system.should_quit() {
                break 'main_loop;
            }
            $system.end_update();

            // draw frame
            $callback.draw($context);
            $system.render_screen(&$graphics);

            loop {
                $system.prepare_for_update();

                while $system.should_update() {
                    // update frame
                    $system.start_update();
                    if $system.should_quit() {
                        break 'main_loop;
                    }
                    $callback.update($context);
                    if $system.should_quit() {
                        break 'main_loop;
                    }
                    $system.end_update();
                }

                // draw frame
                $callback.draw($context);
                $system.render_screen(&$graphics);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    //
}
