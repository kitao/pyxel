use std::ffi::CString;
use std::mem::{transmute, MaybeUninit};
use std::os::raw::{c_int, c_void};
use std::ptr::{addr_of_mut, copy_nonoverlapping, null_mut};
use std::slice;

use glow::Context;
use parking_lot::Mutex;

use crate::event::Event;
use crate::platform::{AudioCallback, GlProfile, LoopCallback, Platform};
use crate::sdl2::sdl2_sys::*;

extern "C" fn c_audio_callback(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let callback = unsafe { &mut *userdata.cast::<Mutex<AudioCallback>>() };
    let stream: &mut [i16] =
        unsafe { slice::from_raw_parts_mut(stream.cast::<i16>(), len as usize / 2) };
    let mut guard = callback.lock();
    (*guard)(stream);
}

pub enum Gamepad {
    Unused,
    Controller(i32, *mut SDL_GameController),
}

pub(crate) struct PlatformSdl2 {
    pub window: *mut SDL_Window,
    pub gl_context: *mut Context,
    pub audio_device_id: SDL_AudioDeviceID,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub gamepads: Vec<Gamepad>,
}

impl PlatformSdl2 {
    pub fn new() -> Self {
        Self {
            window: null_mut(),
            gl_context: null_mut(),
            audio_device_id: 0,
            mouse_x: i32::MIN,
            mouse_y: i32::MIN,
            gamepads: Vec::new(),
        }
    }
}

impl Platform for PlatformSdl2 {
    //
    // Core
    //
    fn init(&mut self) {
        assert!(
            unsafe { SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMECONTROLLER,) } >= 0,
            "Failed to initialize SDL2"
        );

        self.gamepads.clear();
        let num_joysticks = unsafe { SDL_NumJoysticks() };
        self.gamepads
            .extend((0..num_joysticks).filter_map(open_gamepad));
    }

    fn quit(&mut self) {
        unsafe {
            SDL_Quit();
        }
    }

    fn ticks(&mut self) -> u32 {
        unsafe { SDL_GetTicks() }
    }

    fn delay(&mut self, ms: u32) {
        unsafe {
            SDL_Delay(ms);
        }
    }

    //
    // Window
    //
    fn init_window(&mut self, title: &str, width: u32, height: u32) {
        let title = CString::new(title).unwrap();
        unsafe {
            self.window = SDL_CreateWindow(
                title.as_ptr(),
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                SDL_WINDOWPOS_UNDEFINED_MASK as i32,
                width as i32,
                height as i32,
                (SDL_WINDOW_OPENGL as Uint32) | (SDL_WINDOW_RESIZABLE as Uint32),
            );
            assert!(!self.window.is_null(), "Failed to create window");

            let name = CString::new("SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH").unwrap();
            let value = CString::new("1").unwrap();
            SDL_SetHint(name.as_ptr(), value.as_ptr());

            // Try to initialize OpenGL 2.1
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE as i32,
            );
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);

            if SDL_GL_CreateContext(self.window).is_null() {
                // Try to initialize OpenGL ES 2.0
                SDL_GL_SetAttribute(
                    SDL_GL_CONTEXT_PROFILE_MASK,
                    SDL_GL_CONTEXT_PROFILE_ES as i32,
                );
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
                assert!(
                    !SDL_GL_CreateContext(self.window).is_null(),
                    "Failed to create OpenGL context"
                );
            }

            self.gl_context =
                transmute::<Box<Context>, *mut Context>(Box::new(Context::from_loader_function(
                    |s| SDL_GL_GetProcAddress(s.as_ptr().cast()).cast_const(),
                )));
        }
    }

    fn window_pos(&mut self) -> (i32, i32) {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        unsafe {
            SDL_GetWindowPosition(self.window, addr_of_mut!(x), addr_of_mut!(y));
        }
        (x, y)
    }

    fn set_window_pos(&mut self, x: i32, y: i32) {
        unsafe {
            SDL_SetWindowPosition(self.window, x, y);
        }
    }

    fn window_size(&mut self) -> (u32, u32) {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        unsafe {
            SDL_GetWindowSize(self.window, addr_of_mut!(width), addr_of_mut!(height));
        }
        (width as u32, height as u32)
    }

    fn set_window_size(&mut self, width: u32, height: u32) {
        unsafe {
            SDL_SetWindowSize(self.window, width as i32, height as i32);
        }
    }

    fn set_window_title(&mut self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe {
            SDL_SetWindowTitle(self.window, title.as_ptr());
        }
    }

    fn set_window_icon(&mut self, width: u32, height: u32, pixels: &[u32]) {
        unsafe {
            let surface = SDL_CreateRGBSurfaceWithFormat(
                0,
                width as i32,
                height as i32,
                32,
                SDL_PIXELFORMAT_RGBA32 as Uint32,
            );

            let dst_pixels = (*surface).pixels.cast::<u32>();
            let dst_pitch = (*surface).pitch / 4;

            for y in 0..height {
                copy_nonoverlapping(
                    pixels.as_ptr().add(width as usize * y as usize),
                    dst_pixels.add(dst_pitch as usize * y as usize),
                    width as usize,
                );
            }

            SDL_SetWindowIcon(self.window, surface);
            SDL_FreeSurface(surface);
        }
    }

    fn is_fullscreen(&mut self) -> bool {
        (unsafe { SDL_GetWindowFlags(self.window) }) & SDL_WINDOW_FULLSCREEN as Uint32 != 0
    }

    fn set_fullscreen(&mut self, enabled: bool) {
        let enabled = if enabled {
            SDL_WINDOW_FULLSCREEN_DESKTOP as Uint32
        } else {
            0
        };
        unsafe {
            SDL_SetWindowFullscreen(self.window, enabled);
        }
    }

    fn set_mouse_pos(&mut self, x: i32, y: i32) {
        let (window_x, window_y) = self.window_pos();
        unsafe {
            SDL_WarpMouseGlobal(window_x + x, window_y + y);
        }
    }

    fn set_mouse_visible(&mut self, visible: bool) {
        let visible = if visible {
            SDL_ENABLE as i32
        } else {
            SDL_DISABLE as i32
        };
        unsafe {
            SDL_ShowCursor(visible);
        }
    }

    fn display_size(&mut self) -> (u32, u32) {
        let mut display_mode = SDL_DisplayMode {
            format: 0,
            w: 0,
            h: 0,
            refresh_rate: 0,
            driverdata: null_mut(),
        };
        assert!(
            unsafe { SDL_GetCurrentDisplayMode(0, addr_of_mut!(display_mode)) } == 0,
            "Failed to get display size"
        );

        (display_mode.w as u32, display_mode.h as u32)
    }

    //
    // Audio
    //
    fn init_audio(&mut self, sample_rate: u32, buffer_size: u32, callback: AudioCallback) {
        let userdata = Box::into_raw(Box::new(Mutex::new(callback))).cast::<c_void>();
        let desired = SDL_AudioSpec {
            freq: sample_rate as i32,
            format: AUDIO_S16 as u16,
            channels: 1,
            silence: 0,
            samples: buffer_size as u16,
            padding: 0,
            size: 0,
            callback: Some(c_audio_callback),
            userdata,
        };

        let mut obtained = MaybeUninit::uninit();
        self.audio_device_id = unsafe {
            SDL_OpenAudioDevice(null_mut(), 0, &raw const desired, obtained.as_mut_ptr(), 0)
        };

        if self.audio_device_id == 0 {
            println!("Failed to initialize audio device");
        }

        self.pause_audio(false);
    }

    fn pause_audio(&mut self, paused: bool) {
        if self.audio_device_id != 0 {
            unsafe {
                SDL_PauseAudioDevice(self.audio_device_id, i32::from(paused));
            }
        }
    }

    //
    // Frame
    //
    fn start_loop(&mut self, mut callback: LoopCallback) {
        loop {
            let start_ms = self.ticks() as f64;
            callback();
            let elapsed_ms = self.ticks() as f64 - start_ms;
            let wait_ms = 1000.0 / 60.0 - elapsed_ms;
            if wait_ms > 0.0 {
                self.delay((wait_ms / 2.0) as u32);
            }
        }
    }

    fn step_loop(&mut self) {
        // TODO
    }

    fn poll_events(&mut self) -> Vec<Event> {
        self.poll_events_impl()
    }

    fn gl_profile(&mut self) -> GlProfile {
        let value = {
            let mut value: i32 = 0;
            unsafe {
                SDL_GL_GetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, addr_of_mut!(value));
            }
            value
        };
        if value & (SDL_GL_CONTEXT_PROFILE_ES as i32) != 0 {
            GlProfile::GLES
        } else {
            GlProfile::GL
        }
    }

    fn gl_context(&mut self) -> &'static mut Context {
        unsafe { &mut *self.gl_context }
    }

    fn gl_swap_buffers(&mut self) {
        unsafe {
            SDL_GL_SwapWindow(self.window);
        }
    }
}

pub fn open_gamepad(device_index: i32) -> Option<Gamepad> {
    let controller = unsafe { SDL_GameControllerOpen(device_index) };
    if controller.is_null() {
        None
    } else {
        let instance_id = unsafe { SDL_JoystickGetDeviceInstanceID(device_index) };
        Some(Gamepad::Controller(instance_id, controller))
    }
}
