use std::cmp::min;

use sdl2::audio::{
    AudioCallback as SdlAudioCallback, AudioDevice as SdlAudioDevice,
    AudioSpecDesired as SdlAudioSpecDesired,
};
use sdl2::controller::{Axis as SdlAxis, Button as SdlButton, GameController as SdlGameController};
use sdl2::event::{Event as SdlEvent, WindowEvent as SdlWindowEvent};
use sdl2::mouse::MouseButton as SdlMouseButton;
use sdl2::pixels::{Color as SdlColor, PixelFormatEnum as SdlPixelFormat};
use sdl2::rect::Rect as SdlRect;
use sdl2::render::{Texture as SdlTexture, WindowCanvas as SdlCanvas};
use sdl2::surface::Surface as SdlSurface;
use sdl2::video::FullscreenType as SdlFullscreenType;
use sdl2::AudioSubsystem as SdlAudio;
use sdl2::EventPump as SdlEventPump;
use sdl2::Sdl as SdlContext;
use sdl2::TimerSubsystem as SdlTimer;

use crate::event::{ControllerAxis, ControllerButton, Event, MouseButton};
use crate::platform::{AudioCallback, Platform};
use crate::types::{Color, Rgb8};

struct AudioContextHolder {
    audio: shared_type!(dyn AudioCallback + Send),
}

impl SdlAudioCallback for AudioContextHolder {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        self.audio.lock().update(out);
    }
}

pub struct Sdl2 {
    sdl_context: SdlContext,
    sdl_event_pump: SdlEventPump,
    sdl_timer: SdlTimer,
    sdl_canvas: SdlCanvas,
    sdl_texture: SdlTexture,
    #[allow(dead_code)]
    sdl_game_controllers: Vec<SdlGameController>,
    sdl_audio: SdlAudio,
    sdl_audio_device: Option<SdlAudioDevice<AudioContextHolder>>,
    screen_width: u32,
    screen_height: u32,
    mouse_x: i32,
    mouse_y: i32,
}

impl Platform for Sdl2 {
    fn new(title: &str, width: u32, height: u32, display_ratio: f64) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let sdl_event_pump = sdl_context.event_pump().unwrap();
        let sdl_timer = sdl_context.timer().unwrap();
        let sdl_video = sdl_context.video().unwrap();
        let sdl_display_mode = sdl_video.desktop_display_mode(0).unwrap();
        let scale = f64::max(
            f64::min(
                sdl_display_mode.w as f64 / width as f64,
                sdl_display_mode.h as f64 / height as f64,
            ) * display_ratio,
            1.0,
        ) as u32;
        let sdl_window = sdl_video
            .window(title, width * scale, height * scale)
            .position_centered()
            .resizable()
            .build()
            .unwrap();
        let mut sdl_canvas = sdl_window.into_canvas().present_vsync().build().unwrap();
        sdl_canvas
            .window_mut()
            .set_minimum_size(width, height)
            .unwrap();
        let sdl_texture = sdl_canvas
            .texture_creator()
            .create_texture_streaming(SdlPixelFormat::RGB24, width, height)
            .unwrap();
        let sdl_game_controller = sdl_context.game_controller().unwrap();
        let mut sdl_game_controllers = Vec::new();
        for i in 0..sdl_game_controller.num_joysticks().unwrap_or(0) {
            if let Ok(gc) = sdl_game_controller.open(i) {
                sdl_game_controllers.push(gc);
            }
        }
        let sdl_audio = sdl_context.audio().unwrap();

        Self {
            sdl_context,
            sdl_event_pump,
            sdl_timer,
            sdl_canvas,
            sdl_texture,
            sdl_game_controllers,
            sdl_audio,
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

    fn set_icon(&mut self, image: &[Vec<Color>], colors: &[Rgb8], scale: u32) {
        let width = image[0].len() as u32;
        let height = image.len() as u32;
        let mut sdl_surface =
            SdlSurface::new(width * scale, height * scale, SdlPixelFormat::RGBA32).unwrap();
        let pitch = sdl_surface.pitch();
        sdl_surface.with_lock_mut(|buffer: &mut [u8]| {
            for y in 0..height * scale {
                for x in 0..width * scale {
                    let color = image[(y / scale) as usize][(x / scale) as usize];
                    let rgb = colors[color as usize];
                    let offset = (y * pitch + x * 4) as usize;
                    buffer[offset] = ((rgb >> 16) & 0xff) as u8;
                    buffer[offset + 1] = ((rgb >> 8) & 0xff) as u8;
                    buffer[offset + 2] = (rgb & 0xff) as u8;
                    buffer[offset + 3] = if color > 0 { 0xff } else { 0x00 };
                }
            }
        });
        self.sdl_canvas.window_mut().set_icon(&sdl_surface);
    }

    fn show_cursor(&self, show: bool) {
        self.sdl_context.mouse().show_cursor(show);
    }

    fn move_cursor(&self, x: i32, y: i32) {
        let (window_x, window_y) = self.sdl_canvas.window().position();
        let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();
        let mouse_x = x * screen_scale as i32 + window_x + screen_x as i32;
        let mouse_y = y * screen_scale as i32 + window_y + screen_y as i32;
        unsafe {
            sdl2::sys::SDL_WarpMouseGlobal(mouse_x, mouse_y);
        }
    }

    fn toggle_fullscreen(&mut self) {
        let window = self.sdl_canvas.window_mut();
        if window.fullscreen_state() == SdlFullscreenType::Off {
            let _ = window.set_fullscreen(SdlFullscreenType::Desktop);
        } else {
            let _ = window.set_fullscreen(SdlFullscreenType::Off);
        }
    }

    fn tick_count(&self) -> u32 {
        self.sdl_timer.ticks()
    }

    fn sleep(&mut self, ms: u32) {
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
                }
                return None;
            }
            let event = match sdl_event.unwrap() {
                // System events
                SdlEvent::Quit { .. } => Event::Quit,
                SdlEvent::DropFile { filename, .. } => Event::DropFile { filename },

                // Window events
                SdlEvent::Window { win_event, .. } => match win_event {
                    SdlWindowEvent::FocusGained => {
                        self.mouse_x = i32::MIN;
                        self.mouse_y = i32::MIN;
                        Event::FocusGained
                    }
                    SdlWindowEvent::FocusLost => Event::FocusLost,
                    SdlWindowEvent::Maximized => {
                        self.mouse_x = i32::MIN;
                        self.mouse_y = i32::MIN;
                        Event::Maximized
                    }
                    SdlWindowEvent::Minimized => Event::Minimized,
                    _ => continue,
                },

                // Key events
                SdlEvent::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => Event::KeyDown {
                    keycode: keycode as u32,
                },
                SdlEvent::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => Event::KeyUp {
                    keycode: keycode as u32,
                },
                SdlEvent::TextInput { text, .. } => Event::TextInput { text },

                // Mouse events
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
                SdlEvent::MouseWheel { x, y, .. } => Event::MouseWheel { x, y },

                // Controller events
                SdlEvent::ControllerAxisMotion {
                    which, axis, value, ..
                } => Event::ControllerAxisMotion {
                    which,
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
                        which,
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
                            SdlButton::Misc1 => ControllerButton::Misc1,
                            SdlButton::Paddle1 => ControllerButton::Paddle1,
                            SdlButton::Paddle2 => ControllerButton::Paddle2,
                            SdlButton::Paddle3 => ControllerButton::Paddle3,
                            SdlButton::Paddle4 => ControllerButton::Paddle4,
                            SdlButton::Touchpad => ControllerButton::Touchpad,
                        },
                    }
                }
                SdlEvent::ControllerButtonUp { which, button, .. } => Event::ControllerButtonUp {
                    which,
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
                        SdlButton::Misc1 => ControllerButton::Misc1,
                        SdlButton::Paddle1 => ControllerButton::Paddle1,
                        SdlButton::Paddle2 => ControllerButton::Paddle2,
                        SdlButton::Paddle3 => ControllerButton::Paddle3,
                        SdlButton::Paddle4 => ControllerButton::Paddle4,
                        SdlButton::Touchpad => ControllerButton::Touchpad,
                    },
                },

                // Others
                _ => continue,
            };
            return Some(event);
        }
    }

    fn render_screen(&mut self, image: &[Vec<Color>], colors: &[Rgb8], bg_color: Rgb8) {
        let width = image[0].len() as u32;
        let height = image.len() as u32;
        self.sdl_texture
            .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                for i in 0..height as usize {
                    for j in 0..width as usize {
                        let offset = i * pitch + j * 3;
                        let color = colors[image[i][j] as usize];
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

        // Instead of self.sdl_canvas.clear()
        {
            let display_size = self.sdl_canvas.output_size().unwrap();
            self.sdl_canvas
                .fill_rect(SdlRect::new(0, 0, display_size.0, display_size.1))
                .unwrap();
        }

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
        num_samples: u32,
        audio: shared_type!(dyn AudioCallback + Send),
    ) {
        let spec = SdlAudioSpecDesired {
            freq: Some(sample_rate as i32),
            channels: Some(1),
            samples: Some(num_samples as u16),
        };
        let sdl_audio_device = self
            .sdl_audio
            .open_playback(None, &spec, |_| AudioContextHolder { audio })
            .unwrap();
        sdl_audio_device.resume();
        self.sdl_audio_device = Some(sdl_audio_device);
    }

    fn pause_audio(&mut self) {
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.pause();
        }
    }

    fn resume_audio(&mut self) {
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.resume();
        }
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
        let (window_x, window_y) = self.sdl_canvas.window().position();
        let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();
        let mut mouse_x = 0;
        let mut mouse_y = 0;
        unsafe {
            sdl2::sys::SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y);
        }
        mouse_x = (mouse_x - window_x - screen_x as i32) / screen_scale as i32;
        mouse_y = (mouse_y - window_y - screen_y as i32) / screen_scale as i32;
        (mouse_x, mouse_y)
    }
}
