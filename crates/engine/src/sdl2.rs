use std::cmp::min;

use sdl2::controller::Axis as SdlAxis;
use sdl2::controller::Button as SdlButton;
use sdl2::event::Event as SdlEvent;
use sdl2::event::WindowEvent as SdlWindowEvent;
use sdl2::mouse::MouseButton as SdlMouseButton;
use sdl2::pixels::Color as SdlColor;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::video::FullscreenType as SdlFullscreenType;
use sdl2::EventPump as SdlEventPump;
use sdl2::TimerSubsystem as SdlTimerSubsystem;

use crate::canvas::Canvas;
use crate::event::{ControllerAxis, ControllerButton, Event, MouseButton};
use crate::image::Image;
use crate::palette::Rgb24;
use crate::platform::Platform;

pub struct Sdl2 {
    /*
    screen_x: i32,
    screen_y: i32,
    screen_scale: u32,
    */
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
        let mut sdl_window = sdl_video
            .window("", width, height)
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        sdl_window.set_minimum_size(width, height).unwrap();

        let sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_texture_creator = sdl_canvas.texture_creator();
        let sdl_texture = sdl_texture_creator
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();

        Sdl2 {
            sdl_timer: sdl_timer,
            sdl_canvas: sdl_canvas,
            sdl_texture: sdl_texture,
            sdl_event_pump: sdl_event_pump,
        }

        /*
                screen_width_ = screen_width;
                screen_height_ = screen_height;
                screen_scale_ = screen_scale;
                is_fullscreen_ = false;
                mouse_wheel_ = 0;

                if (screen_scale_ <= 0) {
                    SDL_DisplayMode display_mode;
        e           SDL_GetDesktopDisplayMode(0, &display_mode);

                    screen_scale_ = Max(
                        Min(display_mode.w / screen_width_, display_mode.h / screen_height_) * MAX_WINDOW_SIZE_RATIO, 1.0f);
                }

                int32_t window_width = screen_width_ * screen_scale_;
                int32_t window_height = screen_height_ * screen_scale_;

                window_ = SDL_CreateWindow(caption.c_str(), SDL_WINDOWPOS_CENTERED,
                                            SDL_WINDOWPOS_CENTERED, window_width,
                                            window_height, SDL_WINDOW_RESIZABLE);


                SetupWindowIcon();
                UpdateWindowInfo();
                */
    }
}

impl Platform for Sdl2 {
    #[inline]
    fn window_pos(&self) -> (i32, i32) {
        self.sdl_canvas.window().position()
    }

    #[inline]
    fn window_size(&self) -> (u32, u32) {
        self.sdl_canvas.window().size()
    }

    #[inline]
    fn window_title(&self) -> &str {
        self.sdl_canvas.window().title()
    }

    #[inline]
    fn set_window_title(&mut self, title: &str) {
        self.sdl_canvas.window_mut().set_title(title).unwrap();
    }

    #[inline]
    fn set_window_icon(&mut self, icon: &Image, scale: u32) {
        //
    }

    #[inline]
    fn is_fullscreen(&self) -> bool {
        self.sdl_canvas.window().fullscreen_state() == SdlFullscreenType::True
    }

    #[inline]
    fn set_fullscreen(&mut self, is_fullscreen: bool) {
        if is_fullscreen {
            let _ = self
                .sdl_canvas
                .window_mut()
                .set_fullscreen(SdlFullscreenType::True);
        } else {
            let _ = self
                .sdl_canvas
                .window_mut()
                .set_fullscreen(SdlFullscreenType::Off);
        }
    }

    #[inline]
    fn ticks(&self) -> u32 {
        self.sdl_timer.ticks()
    }

    #[inline]
    fn delay(&mut self, ms: u32) {
        self.sdl_timer.delay(ms);
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            let sdl_event = self.sdl_event_pump.poll_event();

            if sdl_event.is_none() {
                return None;
            }

            let event = match sdl_event.unwrap() {
                //
                // System Events
                //
                SdlEvent::Quit { .. } => Event::Quit,

                SdlEvent::DropFile { filename, .. } => Event::DropFile { filename: filename },

                //
                // Window Events
                //
                SdlEvent::Window { win_event, .. } => match win_event {
                    /*
                    WindowShown,
                    WindowHidden,
                    */
                    SdlWindowEvent::Moved(x, y) => Event::WindowMoved { x: x, y: y },

                    SdlWindowEvent::Resized(width, height) => Event::WindowResized {
                        width: width,
                        height: height,
                    },
                    _ => continue,
                    /*
                    WindowMinimized,
                    WindowMaximized,
                    WindowEnter,
                    WindowLeave,
                    WindowFocusGained,
                    WindowFocusLost,
                    WindowClose,
                    */
                },

                //
                // Key Events
                //
                SdlEvent::KeyDown {
                    scancode: Some(scancode),
                    ..
                } => Event::KeyDown {
                    key: scancode as u32,
                },

                SdlEvent::KeyUp {
                    scancode: Some(scancode),
                    ..
                } => Event::KeyUp {
                    key: scancode as u32,
                },

                SdlEvent::TextInput { text, .. } => Event::TextInput { text: text },

                //
                // Mouse Events
                //
                SdlEvent::MouseMotion { x, y, .. } => Event::MouseMotion { x: x, y: y },

                SdlEvent::MouseButtonDown { mouse_btn, .. } => Event::MouseButtonDown {
                    button: match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Middle => MouseButton::Middle,
                        SdlMouseButton::Right => MouseButton::Right,
                        SdlMouseButton::X1 => MouseButton::X1,
                        SdlMouseButton::X2 => MouseButton::X2,
                        SdlMouseButton::Unknown => MouseButton::Unknown,
                    },
                },

                SdlEvent::MouseButtonUp { mouse_btn, .. } => Event::MouseButtonUp {
                    button: match mouse_btn {
                        SdlMouseButton::Left => MouseButton::Left,
                        SdlMouseButton::Middle => MouseButton::Middle,
                        SdlMouseButton::Right => MouseButton::Right,
                        SdlMouseButton::X1 => MouseButton::X1,
                        SdlMouseButton::X2 => MouseButton::X2,
                        SdlMouseButton::Unknown => MouseButton::Unknown,
                    },
                },

                SdlEvent::MouseWheel { x, y, .. } => Event::MouseWheel { x: x, y: y },

                //
                // Controller Events
                //
                SdlEvent::ControllerAxisMotion {
                    which, axis, value, ..
                } => Event::ControllerAxisMotion {
                    which: which,
                    axis: match axis {
                        SdlAxis::LeftX => ControllerAxis::LeftX,
                        SdlAxis::LeftY => ControllerAxis::LeftY,
                        SdlAxis::RightX => ControllerAxis::RightX,
                        SdlAxis::RightY => ControllerAxis::RightY,
                        SdlAxis::TriggerLeft => ControllerAxis::TriggerLeft,
                        SdlAxis::TriggerRight => ControllerAxis::TriggerRight,
                    },
                    value: value as i32,
                },

                SdlEvent::ControllerButtonDown { which, button, .. } => {
                    Event::ControllerButtonDown {
                        which: which,
                        button: match button {
                            SdlButton::A => ControllerButton::A,
                            SdlButton::B => ControllerButton::B,
                            SdlButton::X => ControllerButton::X,
                            SdlButton::Y => ControllerButton::Y,
                            SdlButton::Back => ControllerButton::Back,
                            SdlButton::Guide => ControllerButton::Guide,
                            SdlButton::Start => ControllerButton::Start,
                            SdlButton::LeftStick => ControllerButton::LeftStick,
                            SdlButton::RightStick => ControllerButton::RightStick,
                            SdlButton::LeftShoulder => ControllerButton::LeftShoulder,
                            SdlButton::RightShoulder => ControllerButton::RightShoulder,
                            SdlButton::DPadUp => ControllerButton::DPadUp,
                            SdlButton::DPadDown => ControllerButton::DPadDown,
                            SdlButton::DPadLeft => ControllerButton::DPadLeft,
                            SdlButton::DPadRight => ControllerButton::DPadRight,
                        },
                    }
                }

                SdlEvent::ControllerButtonUp { which, button, .. } => Event::ControllerButtonUp {
                    which: which,
                    button: match button {
                        SdlButton::A => ControllerButton::A,
                        SdlButton::B => ControllerButton::B,
                        SdlButton::X => ControllerButton::X,
                        SdlButton::Y => ControllerButton::Y,
                        SdlButton::Back => ControllerButton::Back,
                        SdlButton::Guide => ControllerButton::Guide,
                        SdlButton::Start => ControllerButton::Start,
                        SdlButton::LeftStick => ControllerButton::LeftStick,
                        SdlButton::RightStick => ControllerButton::RightStick,
                        SdlButton::LeftShoulder => ControllerButton::LeftShoulder,
                        SdlButton::RightShoulder => ControllerButton::RightShoulder,
                        SdlButton::DPadUp => ControllerButton::DPadUp,
                        SdlButton::DPadDown => ControllerButton::DPadDown,
                        SdlButton::DPadLeft => ControllerButton::DPadLeft,
                        SdlButton::DPadRight => ControllerButton::DPadRight,
                    },
                },

                //
                // Default
                //
                _ => continue,
            };

            return Some(event);
        }
    }

    fn render_screen(&mut self, screen: &Image, bg_color: Rgb24) {
        let width = screen.width() as usize;
        let height = screen.height() as usize;
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

        let screen_width = screen.width();
        let screen_height = screen.height();
        let (window_width, window_height) = self.sdl_canvas.window().size();
        let screen_scale = min(window_width / screen_width, window_height / screen_height);
        let screen_x = (window_width - screen_width * screen_scale) / 2;
        let screen_y = (window_height - screen_height * screen_scale) / 2;

        let dst = SdlRect::new(
            screen_x as i32,
            screen_y as i32,
            screen_width * screen_scale,
            screen_height * screen_scale,
        );

        self.sdl_canvas.clear();
        self.sdl_canvas
            .copy(&self.sdl_texture, None, Some(dst))
            .unwrap();
        self.sdl_canvas.present();
    }
}
