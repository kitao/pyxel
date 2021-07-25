use std::cmp::min;
use std::sync::{Arc, Mutex};

use sdl2::audio::AudioCallback as SdlAudioCallback;
use sdl2::audio::AudioDevice as SdlAudioDevice;
use sdl2::audio::AudioSpecDesired as SdlAudioSpecDesired;
use sdl2::controller::Axis as SdlAxis;
use sdl2::controller::Button as SdlButton;
use sdl2::event::Event as SdlEvent;
use sdl2::mouse::MouseButton as SdlMouseButton;
use sdl2::pixels::Color as SdlColor;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::video::FullscreenType as SdlFullscreenType;
use sdl2::AudioSubsystem as SdlAudioSubsystem;
use sdl2::EventPump as SdlEventPump;
use sdl2::TimerSubsystem as SdlTimerSubsystem;

use crate::canvas::Canvas;
use crate::event::{ControllerAxis, ControllerButton, Event, MouseButton};
use crate::image::Image;
use crate::platform::{AudioCallback, Platform};
use crate::types::Rgb8;

struct AudioCallbackData {
    audio_callback: Arc<Mutex<dyn AudioCallback + Send>>,
}

impl SdlAudioCallback for AudioCallbackData {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        self.audio_callback.lock().unwrap().audio_callback(out);
    }
}

pub struct Sdl2 {
    sdl_canvas: SdlCanvas,
    sdl_texture: SdlTexture,
    sdl_timer: SdlTimerSubsystem,
    sdl_event_pump: SdlEventPump,
    sdl_audio: SdlAudioSubsystem,
    sdl_audio_device: Option<SdlAudioDevice<AudioCallbackData>>,
    screen_width: u32,
    screen_height: u32,
    mouse_x: i32,
    mouse_y: i32,
}

impl Platform for Sdl2 {
    fn new(title: &str, width: u32, height: u32, scale: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_window = sdl_video
            .window(title, width * scale, height * scale)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut sdl_canvas = sdl_window.into_canvas().build().unwrap();
        let sdl_texture = sdl_canvas
            .texture_creator()
            .create_texture_streaming(PixelFormatEnum::RGB24, width, height)
            .unwrap();
        let sdl_timer = sdl_context.timer().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_audio = sdl_context.audio().unwrap();

        sdl_canvas
            .window_mut()
            .set_minimum_size(width, height)
            .unwrap();

        Sdl2 {
            sdl_timer: sdl_timer,
            sdl_canvas: sdl_canvas,
            sdl_texture: sdl_texture,
            sdl_event_pump: sdl_event_pump,
            sdl_audio: sdl_audio,
            sdl_audio_device: None,
            screen_width: width,
            screen_height: height,
            mouse_x: i32::MIN,
            mouse_y: i32::MIN,
        }
    }

    fn set_title(&mut self, title: &str) {
        self.sdl_canvas.window_mut().set_title(title).unwrap();
    }

    fn set_icon(&mut self, icon: &Image, colors: &[Rgb8], scale: u32) {
        /*
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
        */
    }

    fn is_fullscreen(&mut self) -> bool {
        self.sdl_canvas.window().fullscreen_state() != SdlFullscreenType::Off
    }

    fn set_fullscreen(&mut self, fullscreen: bool) {
        let window = self.sdl_canvas.window_mut();

        if fullscreen {
            let _ = window.set_fullscreen(SdlFullscreenType::True);
        } else {
            let _ = window.set_fullscreen(SdlFullscreenType::Off);
        }
    }

    fn ticks(&self) -> u32 {
        self.sdl_timer.ticks()
    }

    fn delay(&mut self, ms: u32) {
        self.sdl_timer.delay(ms);
    }

    fn poll_event(&mut self) -> Option<Event> {
        loop {
            let sdl_event = self.sdl_event_pump.poll_event();

            if sdl_event.is_none() {
                let (cur_mouse_x, cur_mouse_y) = self.mouse_pos();

                if cur_mouse_x != self.mouse_x || cur_mouse_y != self.mouse_y {
                    self.mouse_x = cur_mouse_x;
                    self.mouse_y = cur_mouse_y;

                    return Some(Event::MouseMotion {
                        x: cur_mouse_x,
                        y: cur_mouse_y,
                    });
                } else {
                    return None;
                }
            }

            let event = match sdl_event.unwrap() {
                //
                // System Events
                //
                SdlEvent::Quit { .. } => Event::Quit,

                SdlEvent::DropFile { filename, .. } => Event::DropFile { filename: filename },

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

    fn render_screen(&mut self, screen: &Image, colors: &[Rgb8], bg_color: Rgb8) {
        let width = screen.width;
        let height = screen.height;
        let data = &screen.data;

        assert!(self.screen_width == width && self.screen_height == height);

        self.sdl_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for i in 0..height as usize {
                    for j in 0..width as usize {
                        let offset = i * pitch + j * 3;
                        let color = colors[data[i][j] as usize];

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

        let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();

        let dst = SdlRect::new(
            screen_x as i32,
            screen_y as i32,
            width * screen_scale,
            height * screen_scale,
        );

        self.sdl_canvas
            .copy(&self.sdl_texture, None, Some(dst))
            .unwrap();

        self.sdl_canvas.present();
    }

    fn start_audio(
        &mut self,
        sample_rate: u32,
        sample_count: u32,
        audio_callback: Arc<Mutex<dyn AudioCallback + Send>>,
    ) {
        let spec = SdlAudioSpecDesired {
            freq: Some(sample_rate as i32),
            channels: Some(1),
            samples: Some(sample_count as u16),
        };

        let sdl_audio_device = self
            .sdl_audio
            .open_playback(None, &spec, |_| AudioCallbackData {
                audio_callback: audio_callback,
            })
            .unwrap();

        sdl_audio_device.resume();

        self.sdl_audio_device = Some(sdl_audio_device);
    }
}

impl Sdl2 {
    fn screen_pos_scale(&self) -> (u32, u32, u32) {
        let (window_width, window_height) = self.sdl_canvas.window().size();
        let screen_scale = min(
            window_width / self.screen_width,
            window_height / self.screen_height,
        );
        let screen_x = (window_width - self.screen_width * screen_scale) / 2;
        let screen_y = (window_height - self.screen_height * screen_scale) / 2;

        (screen_x, screen_y, screen_scale)
    }

    fn mouse_pos(&self) -> (i32, i32) {
        let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();
        let mut mouse_x = self.mouse_x;
        let mut mouse_y = self.mouse_y;

        unsafe {
            sdl2::sys::SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y);
        }

        mouse_x = (mouse_x - screen_x as i32) / screen_scale as i32;
        mouse_y = (mouse_y - screen_y as i32) / screen_scale as i32;

        (mouse_x, mouse_y)
    }
}
