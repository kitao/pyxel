use sdl2::event::Event as SdlEvent;
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
use crate::event::Event;
use crate::image::Image;
use crate::palette::{Color, Palette, Rgb24};
use crate::platform::Platform;

pub struct Sdl2 {
    caption: String,
    width: u32,
    height: u32,
    is_fullscreen: bool,

    /*window_x: i32,
    window_y: i32,
    screen_x: i32,
    screen_y: i32,
    screen_scale: u32,
    mouse_wheel: i32,
    drop_file: String,
    */
    sdl_context: SdlContext,
    sdl_video: SdlVideoSubsystem,
    sdl_timer: SdlTimerSubsystem,
    sdl_canvas: SdlCanvas,
    sdl_texture: SdlTexture,
    sdl_event_pump: SdlEventPump,
}

impl Sdl2 {
    pub fn new(width: u32, height: u32) -> Sdl2 {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_timer = sdl_context.timer().unwrap();
        let sdl_window = sdl_video
            .window("", width, height)
            .position_centered()
            .build()
            .unwrap();
        let sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_texture_creator = sdl_canvas.texture_creator();
        let sdl_texture = sdl_texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        Sdl2 {
            caption: "".to_string(),
            width: width,
            height: height,
            is_fullscreen: false,

            sdl_context: sdl_context,
            sdl_video: sdl_video,
            sdl_timer: sdl_timer,
            sdl_canvas: sdl_canvas,
            sdl_texture: sdl_texture,
            sdl_event_pump: sdl_event_pump,
        }

        /*
        screen_width_ = screen_width;
        screen_height_ = screen_height;
        screen_scale_ = screen_scale;
        palette_color_ = palette_color;
        is_fullscreen_ = false;
        mouse_wheel_ = 0;

        if (screen_scale_ <= 0) {
            SDL_DisplayMode display_mode;
            SDL_GetDesktopDisplayMode(0, &display_mode);

            screen_scale_ = Max(
                Min(display_mode.w / screen_width_, display_mode.h / screen_height_) * MAX_WINDOW_SIZE_RATIO, 1.0f);
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
        */
    }
}

impl Platform for Sdl2 {
    #[inline]
    fn width(&self) -> u32 {
        0
    }

    #[inline]
    fn height(&self) -> u32 {
        0
    }

    #[inline]
    fn caption(&self) -> &str {
        "hoge"
    }

    #[inline]
    fn set_caption(&mut self, caption: &str) {
        //
    }

    #[inline]
    fn set_icon(&mut self, icon: &Image, scale: u32) {
        //
    }

    #[inline]
    fn is_full_screen(&self) -> bool {
        false
    }

    #[inline]
    fn set_full_screen(&mut self, is_full_screen: bool) {
        //
    }

    #[inline]
    fn ticks(&self) -> u32 {
        0
    }

    #[inline]
    fn delay(&self, ms: u32) {
        //
    }

    fn poll_event(&mut self) -> Option<Event> {
        let sdl_event = self.sdl_event_pump.poll_event();

        if sdl_event.is_none() {
            return None;
        }

        match sdl_event.unwrap() {
            SdlEvent::Quit { .. }
            | SdlEvent::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => Some(Event::Quit),

            _ => None,
        }

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

    fn render_screen(&mut self, screen: &Image, bg_color: Rgb24) {
        let width = self.width as usize;
        let height = self.height as usize;
        let data = screen.data();
        let palette = screen.palette();

        self.sdl_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for i in 0..height {
                    for j in 0..width {
                        let offset = i * pitch + j * 3;
                        let color = palette.display_color(data[i][j]);

                        buffer[offset] = ((color >> 16) & 0xff) as u8;
                        buffer[offset + 1] = ((color >> 8) & 0xff) as u8;
                        buffer[offset + 2] = (color & 0xff) as u8;
                    }
                }
            })
            .unwrap();

        self.sdl_canvas.set_draw_color(SdlColor::RGB(
            ((bg_color >> 16) & 0xff) as u8,
            ((bg_color >> 8) & 0xff) as u8,
            (bg_color & 0xff) as u8,
        ));
        self.sdl_canvas.clear();
        self.sdl_canvas.copy(&self.sdl_texture, None, None).unwrap();
        self.sdl_canvas.present();

        /*
        int32_t* framebuffer;
        int32_t pitch;
        int32_t size = screen_width_ * screen_height_;

        SDL_LockTexture(screen_texture_, NULL, reinterpret_cast<void**>(&framebuffer), &pitch);

        for (int32_t i = 0; i < screen_height_; i++) {
            int32_t index = screen_width_ * i;
            for (int32_t j = 0; j < screen_width_; j++) {
            framebuffer[index + j] = palette_color_[screen_data[i][j]];
            }

        SDL_UnlockTexture(screen_texture_);
        */

        /*
        uint8_t r = (WINDOW_BACKGROUND_COLOR >> 16) & 0xff;
        uint8_t g = (WINDOW_BACKGROUND_COLOR >> 8) & 0xff;
        uint8_t b = WINDOW_BACKGROUND_COLOR & 0xff;

        SDL_SetRenderDrawColor(renderer_, r, g, b, 255);
        SDL_RenderClear(renderer_);

        UpdateScreenTexture(screen_data);

        SDL_Rect dst_rect = {
            screen_x_,
            screen_y_,
            screen_width_ * screen_scale_,
            screen_height_ * screen_scale_,
        };

        SDL_RenderCopy(renderer_, screen_texture_, NULL, &dst_rect);
        SDL_RenderPresent(renderer_);
        */
    }
}
