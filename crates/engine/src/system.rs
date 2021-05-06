use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl as SdlContext;
use sdl2::VideoSubsystem as SdlVideoSubsystem;

pub struct System {
    sdl_context: SdlContext,
    sdl_video_subsystem: SdlVideoSubsystem,
    sdl_canvas: SdlCanvas,
    sdl_texture: SdlTexture,
    sdl_event_pump: SdlEventPump,
    /*
        Window* window_;
        Recorder* recorder_;

        pyxelcore::PaletteColor palette_color_;
        int32_t quit_key_;
        int32_t fps_;
        int32_t frame_count_;
        double one_frame_time_;
        double next_update_time_;
        std::string drop_file_;
        bool is_loop_running_;
        bool is_quit_requested_;
        bool is_update_suspended_;

        Profiler fps_profiler_;
        Profiler update_profiler_;
        Profiler draw_profiler_;
        bool is_performance_monitor_on_;
    */
}

impl System {
    pub fn new(name: &str, width: u32, height: u32) -> System {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video_subsystem = sdl_context.video().unwrap();
        let sdl_window = sdl_video_subsystem
            .window(name, width, height)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_texture_creator = sdl_canvas.texture_creator();
        let sdl_texture = sdl_texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        System {
            sdl_context: sdl_context,
            sdl_video_subsystem: sdl_video_subsystem,
            sdl_canvas: sdl_canvas,
            sdl_texture: sdl_texture,
            sdl_event_pump: sdl_event_pump,
        }

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
          int32_t window_height = screen_height_ * screen_scale_;

          window_ = SDL_CreateWindow(caption.c_str(), SDL_WINDOWPOS_CENTERED,
                                     SDL_WINDOWPOS_CENTERED, window_width,
                                     window_height, SDL_WINDOW_RESIZABLE);

          renderer_ = SDL_CreateRenderer(window_, -1, 0);

          screen_texture_ = SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                                              SDL_TEXTUREACCESS_STREAMING,
                                              screen_width_, screen_height_);

          SDL_SetWindowMinimumSize(window_, screen_width_, screen_height_);

          SetupWindowIcon();
          UpdateWindowInfo();
        }
                                              */
    }

    pub fn run(&mut self) {
        /*let palette = self.image_bank.palette();
        let data = self.image_bank.data();
        let width = self.screen_width as usize;
        let height = self.screen_height as usize;

        self.sdl_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for y in 0..height {
                    for x in 0..width {
                        let c = palette.get_display_color(data[y][x]);
                        let offset = y * pitch + x * 3;

                        buffer[offset] = ((c >> 16) & 0xff) as u8;
                        buffer[offset + 1] = ((c >> 8) & 0xff) as u8;
                        buffer[offset + 2] = (c & 0xff) as u8;
                    }
                }
            })
            .unwrap();*/

        'main_loop: loop {
            self.sdl_canvas.set_draw_color(Color::RGB(200, 200, 200));
            self.sdl_canvas.clear();

            /*self.sdl_canvas
            .copy(&self.sdl_texture, None, None)
            .expect("Render failed");*/

            //canvas.copy(&texture, None, Some(Rect::new(100, 100, 256, 256)))?;
            /*canvas.copy_ex(
                &texture,
                None,
                Some(Rect::new(450, 100, 256, 256)),
                30.0,
                None,
                false,
                false,
            )?;
            */

            self.sdl_canvas.present();

            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main_loop,
                    _ => {}
                }
            }

            //thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));}
        }

        /*
        int32_t* framebuffer;
        int32_t pitch;
        int32_t size = screen_width_ * screen_height_;

        SDL_LockTexture(screen_texture_, NULL, reinterpret_cast<void**>(&framebuffer),
                        &pitch);

        for (int32_t i = 0; i < screen_height_; i++) {
          int32_t index = screen_width_ * i;
          for (int32_t j = 0; j < screen_width_; j++) {
            framebuffer[index + j] = palette_color_[screen_data[i][j]];
          }
        }

        SDL_UnlockTexture(screen_texture_);
        */
    }

    pub fn get_window_caption(&self) -> &'static str {
        "hoge"
    }

    pub fn set_window_caption(&self, caption: &str) {
        //
    }

    pub fn is_fullscreen(&self) -> bool {
        true
    }

    pub fn set_fullscreen(&self, is_fullscreen: bool) {
        //
    }

    /*
        System(int32_t width,
        int32_t height,
        const std::string& caption,
        int32_t scale,
        const pyxelcore::PaletteColor& palette_color,
        int32_t fps,
        int32_t quit_key,
        bool is_fullscreen);
        ~System();

        const pyxelcore::PaletteColor& PaletteColor() const { return palette_color_; }

        int32_t Width() const { return window_->ScreenWidth(); }
        int32_t Height() const { return window_->ScreenHeight(); }
        int32_t FrameCount() const { return frame_count_; }

        void Run(void (*update)(), void (*draw)());
        bool Quit();
        bool FlipScreen();
        void ShowScreen();

        std::string DropFile() const { return drop_file_; }
        void SetCaption(const std::string& caption);

    private:
        int32_t WaitForUpdateTime();
        bool UpdateFrame(void (*update)());
        void CheckSpecialInput();
        void DrawFrame(void (*draw)(), int32_t update_frame_count);
        void DrawPerformanceMonitor();
        void DrawMouseCursor();
    */
}

#[cfg(test)]
mod tests {
    //
}
