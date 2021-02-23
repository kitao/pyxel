use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl as SdlContext;
use sdl2::VideoSubsystem as SdlVideo;
//use sdl2::video::Window as SdlWindow;

global_instance!(System, system);

pub struct System {
    sdl_context: SdlContext,
    sdl_event_pump: SdlEventPump,
    sdl_video: SdlVideo,
    //sdl_window: SdlWindow,
    sdl_canvas: SdlCanvas,
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

pub fn init_system(name: &str, width: usize, height: usize) {
    crate::init_resource();
    crate::init_input();
    crate::init_graphics(width, height);
    crate::init_audio();

    let sdl_context = sdl2::init().unwrap();
    let sdl_event_pump = sdl_context.event_pump().unwrap();
    let sdl_video = sdl_context.video().unwrap();
    let sdl_window = sdl_video
        .window(name, width as u32, height as u32)
        .position_centered()
        .build()
        .unwrap();
    let sdl_canvas = sdl_window.into_canvas().build().unwrap();

    let system = System {
        sdl_context: sdl_context,
        sdl_event_pump: sdl_event_pump,
        sdl_video: sdl_video,
        //sdl_window: sdl_window,
        sdl_canvas: sdl_canvas,
    };

    set_instance(system);
}

impl System {
    pub fn run(&mut self) {
        'running: loop {
            self.sdl_canvas.set_draw_color(Color::RGB(200, 200, 200));
            self.sdl_canvas.clear();

            //self.sdl_canvas.copy(&texture, None, None).expect("Render failed");

            self.sdl_canvas.present();

            for event in self.sdl_event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            //thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));}
        }
    }

    pub fn set_window_caption(&self, caption: &str) {
        //
    }

    pub fn get_window_caption(&self) -> &'static str {
        "hoge"
    }

    pub fn get_screen_size(&self) -> (usize, usize) {
        (1, 2)
    }

    pub fn is_fullscreen(&self) -> bool {
        true
    }

    pub fn set_fullscreen(&self, is_fullscreen: bool) {
        //
    }

    pub fn get_palette_color(&self, index: usize) -> u32 {
        0
    }

    pub fn set_palette_color(&self, index: usize, color: u32) {
        //
    }

    /*
    public:
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
    #[test]
    fn hoge() {
        //
    }
}
