use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::slice::from_raw_parts_mut;
use std::sync::atomic::{AtomicU32, Ordering};

use glow::Context;

use super::super::facade::GLProfile;
use super::poll_events::{open_gamepad, GamepadSlot};
#[allow(clippy::wildcard_imports)]
use super::sdl2_sys::*;

static AUDIO_DEVICE_ID: AtomicU32 = AtomicU32::new(0);

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_run_script(script: *const std::os::raw::c_char);
    fn emscripten_set_main_loop_arg(
        func: unsafe extern "C" fn(*mut c_void),
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    fn emscripten_cancel_main_loop();
}

#[cfg(target_os = "emscripten")]
unsafe extern "C" fn main_loop_callback<F: FnMut(f32)>(arg: *mut c_void) {
    (*arg.cast::<F>())(10.0);
}

extern "C" fn audio_callback(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let callback = unsafe { &mut *userdata.cast::<Box<dyn FnMut(&mut [i16])>>() };
    let stream = unsafe { from_raw_parts_mut(stream.cast::<i16>(), len as usize / 2) };
    (*callback)(stream);
}

pub struct PlatformSdl2 {
    pub window: *mut SDL_Window,
    pub gl_context: *mut Context,
    pub audio_device_id: SDL_AudioDeviceID,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub is_wayland: bool,
    pub gamepads: Vec<GamepadSlot>,
    #[cfg(target_os = "emscripten")]
    pub virtual_gamepad_states: [bool; 10],
    #[cfg(not(target_os = "emscripten"))]
    pub next_update_ms: Option<f32>,
}

impl PlatformSdl2 {
    pub fn new() -> Self {
        Self {
            window: null_mut(),
            gl_context: null_mut(),
            audio_device_id: 0,
            mouse_x: i32::MIN,
            mouse_y: i32::MIN,
            is_wayland: false,
            gamepads: Vec::new(),
            #[cfg(target_os = "emscripten")]
            virtual_gamepad_states: [false; 10],
            #[cfg(not(target_os = "emscripten"))]
            next_update_ms: None,
        }
    }

    //
    // Core
    //
    pub fn init(&mut self, headless: bool) {
        if headless {
            unsafe { SDL_Init(0) };
            return;
        }

        let sdl_flags = SDL_INIT_VIDEO | SDL_INIT_GAMECONTROLLER;

        // Prefer Wayland driver on Wayland sessions (workaround for bundled
        // SDL2 failing to auto-detect Wayland). Falls back to auto-detection.
        let initialized = if std::env::var("XDG_SESSION_TYPE").is_ok_and(|v| v == "wayland")
            && std::env::var("SDL_VIDEODRIVER").is_err()
        {
            std::env::set_var("SDL_VIDEODRIVER", "wayland");
            let ok = unsafe { SDL_Init(sdl_flags) } >= 0;
            if !ok {
                std::env::remove_var("SDL_VIDEODRIVER");
            }
            ok
        } else {
            false
        };

        if !initialized {
            assert!(
                unsafe { SDL_Init(sdl_flags) } >= 0,
                "Failed to initialize SDL2: {}",
                unsafe { CStr::from_ptr(SDL_GetError()) }.to_string_lossy()
            );
        }

        let driver = unsafe { SDL_GetCurrentVideoDriver() };
        self.is_wayland =
            !driver.is_null() && unsafe { CStr::from_ptr(driver) }.to_bytes() == b"wayland";

        self.gamepads.clear();
        let num_joysticks = unsafe { SDL_NumJoysticks() };
        self.gamepads.extend((0..num_joysticks).map(open_gamepad));
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn quit(&mut self) {
        unsafe { SDL_Quit() };
        std::process::exit(0);
    }

    #[cfg(target_os = "emscripten")]
    pub fn quit(&mut self) {
        unsafe { emscripten_cancel_main_loop() };
        self.pause_audio(true);
    }

    pub fn ticks(&self) -> u32 {
        unsafe { SDL_GetTicks() }
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn export_browser_file(&self, _filename: &str) {}

    #[cfg(target_os = "emscripten")]
    pub fn export_browser_file(&self, filename: &str) {
        let script = CString::new(format!("_savePyxelFile('{filename}');")).unwrap();
        unsafe { emscripten_run_script(script.as_ptr()) };
    }

    //
    // Window
    //
    pub fn init_window(&mut self, title: &str, width: u32, height: u32) {
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
            assert!(
                !self.window.is_null(),
                "Failed to create window: {}",
                CStr::from_ptr(SDL_GetError()).to_string_lossy()
            );

            let hint_value = CString::new("1").unwrap();
            SDL_SetHint(
                SDL_HINT_MOUSE_FOCUS_CLICKTHROUGH.as_ptr().cast(),
                hint_value.as_ptr(),
            );

            // Try OpenGL 2.1, fall back to OpenGL ES 2.0
            SDL_GL_SetAttribute(
                SDL_GL_CONTEXT_PROFILE_MASK,
                SDL_GL_CONTEXT_PROFILE_CORE as i32,
            );
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
            SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 1);

            if SDL_GL_CreateContext(self.window).is_null() {
                SDL_GL_SetAttribute(
                    SDL_GL_CONTEXT_PROFILE_MASK,
                    SDL_GL_CONTEXT_PROFILE_ES as i32,
                );
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 2);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
                assert!(
                    !SDL_GL_CreateContext(self.window).is_null(),
                    "Failed to create OpenGL context: {}",
                    CStr::from_ptr(SDL_GetError()).to_string_lossy()
                );
            }

            self.gl_context = Box::into_raw(Box::new(Context::from_loader_function(|s| {
                SDL_GL_GetProcAddress(s.as_ptr().cast()).cast_const()
            })));
        }
    }

    pub fn window_pos(&self) -> (i32, i32) {
        let (mut x, mut y) = (0, 0);
        unsafe { SDL_GetWindowPosition(self.window, &raw mut x, &raw mut y) };
        (x, y)
    }

    pub fn set_window_pos(&mut self, x: i32, y: i32) {
        unsafe { SDL_SetWindowPosition(self.window, x, y) };
    }

    pub fn window_size(&self) -> (u32, u32) {
        let (mut w, mut h) = (0i32, 0i32);
        unsafe { SDL_GetWindowSize(self.window, &raw mut w, &raw mut h) };
        (w as u32, h as u32)
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        unsafe { SDL_SetWindowSize(self.window, width as i32, height as i32) };
    }

    pub fn set_window_title(&mut self, title: &str) {
        let title = CString::new(title).unwrap();
        unsafe { SDL_SetWindowTitle(self.window, title.as_ptr()) };
    }

    pub fn set_window_icon(&mut self, width: u32, height: u32, rgba: &[u8]) {
        unsafe {
            let surface = SDL_CreateRGBSurfaceWithFormat(
                0,
                width as i32,
                height as i32,
                32,
                SDL_PIXELFORMAT_RGBA32 as Uint32,
            );
            assert!(!surface.is_null(), "Failed to create icon surface");

            let pixels = (*surface).pixels.cast::<u8>();
            let size = (height * (*surface).pitch as u32) as usize;
            assert!(rgba.len() >= size, "RGBA buffer too small for icon");
            copy_nonoverlapping(rgba.as_ptr(), pixels, size);

            SDL_SetWindowIcon(self.window, surface);
            SDL_FreeSurface(surface);
        }
    }

    pub fn is_fullscreen(&self) -> bool {
        let flags = unsafe { SDL_GetWindowFlags(self.window) };
        flags & SDL_WINDOW_FULLSCREEN as Uint32 != 0
    }

    pub fn set_fullscreen(&mut self, enabled: bool) {
        let flag = if enabled {
            SDL_WINDOW_FULLSCREEN_DESKTOP as Uint32
        } else {
            0
        };
        unsafe { SDL_SetWindowFullscreen(self.window, flag) };
    }

    pub fn set_mouse_pos(&mut self, x: i32, y: i32) {
        unsafe { SDL_WarpMouseInWindow(self.window, x, y) };
    }

    pub fn set_mouse_visible(&self, visible: bool) {
        let toggle = if visible { SDL_ENABLE } else { SDL_DISABLE } as i32;
        unsafe { SDL_ShowCursor(toggle) };
    }

    pub fn display_size(&self) -> (u32, u32) {
        let mut mode = MaybeUninit::<SDL_DisplayMode>::uninit();
        assert!(
            unsafe { SDL_GetCurrentDisplayMode(0, mode.as_mut_ptr()) } == 0,
            "Failed to get display size"
        );
        let mode = unsafe { mode.assume_init() };
        (mode.w as u32, mode.h as u32)
    }

    //
    // Audio
    //
    pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
        &mut self,
        sample_rate: u32,
        buffer_size: u32,
        callback: F,
    ) {
        unsafe { SDL_InitSubSystem(SDL_INIT_AUDIO) };

        // Reuse the audio device across re-initialization
        let saved_id = AUDIO_DEVICE_ID.load(Ordering::Relaxed);
        if saved_id != 0 {
            self.audio_device_id = saved_id;
            self.pause_audio(false);
            return;
        }

        let userdata = Box::into_raw(Box::new(Box::new(callback) as Box<dyn FnMut(&mut [i16])>))
            .cast::<c_void>();
        let desired = SDL_AudioSpec {
            freq: sample_rate as i32,
            format: AUDIO_S16 as u16,
            channels: 1,
            silence: 0,
            samples: buffer_size as u16,
            padding: 0,
            size: 0,
            callback: Some(audio_callback),
            userdata,
        };

        let mut obtained = MaybeUninit::uninit();
        self.audio_device_id = unsafe {
            SDL_OpenAudioDevice(null_mut(), 0, &raw const desired, obtained.as_mut_ptr(), 0)
        };
        if self.audio_device_id == 0 {
            println!("Failed to initialize audio device");
        }

        AUDIO_DEVICE_ID.store(self.audio_device_id, Ordering::Relaxed);
        self.pause_audio(false);
    }

    pub fn pause_audio(&mut self, paused: bool) {
        if self.audio_device_id != 0 {
            unsafe { SDL_PauseAudioDevice(self.audio_device_id, paused as i32) };
        }
    }

    pub fn lock_audio(&self) {
        if self.audio_device_id != 0 {
            unsafe { SDL_LockAudioDevice(self.audio_device_id) };
        }
    }

    pub fn unlock_audio(&self) {
        if self.audio_device_id != 0 {
            unsafe { SDL_UnlockAudioDevice(self.audio_device_id) };
        }
    }

    //
    // Frame
    //
    #[cfg(not(target_os = "emscripten"))]
    pub fn run_frame_loop<F: FnMut(f32)>(&mut self, fps: u32, mut callback: F) {
        let frame_ms = 1000.0 / fps as f32;
        let mut next_update_ms = self.ticks() as f32;
        let mut last_update_ms = next_update_ms;

        loop {
            // Busy-wait with short sleeps until the next frame time
            loop {
                let remaining_ms = next_update_ms - self.ticks() as f32;
                if remaining_ms <= 0.0 {
                    break;
                }
                unsafe { SDL_Delay((remaining_ms as u32 / 2).max(1)) };
            }

            callback(next_update_ms - last_update_ms);
            if !self.window.is_null() {
                unsafe { SDL_GL_SwapWindow(self.window) };
            }
            last_update_ms = next_update_ms;

            // Catch up if frames were missed
            let ticks = self.ticks() as f32;
            while next_update_ms <= ticks {
                next_update_ms += frame_ms;
            }
        }
    }

    #[cfg(target_os = "emscripten")]
    pub fn run_frame_loop<F: FnMut(f32)>(&mut self, fps: u32, callback: F) {
        unsafe {
            emscripten_set_main_loop_arg(
                main_loop_callback::<F>,
                Box::into_raw(Box::new(callback)).cast::<c_void>(),
                fps as c_int,
                1,
            );
        }
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn step_frame(&mut self, fps: u32) {
        let frame_ms = 1000.0 / fps as f32;
        let mut next_update_ms = self.next_update_ms.unwrap_or(self.ticks() as f32);

        // Busy-wait with short sleeps until the next frame time
        loop {
            let remaining_ms = next_update_ms - self.ticks() as f32;
            if remaining_ms <= 0.0 {
                break;
            }
            unsafe { SDL_Delay((remaining_ms as u32 / 2).max(1)) };
        }

        if !self.window.is_null() {
            unsafe { SDL_GL_SwapWindow(self.window) };
        }

        // Catch up if frames were missed
        let ticks = self.ticks() as f32;
        while next_update_ms <= ticks {
            next_update_ms += frame_ms;
        }
        self.next_update_ms = Some(next_update_ms);
    }

    #[cfg(target_os = "emscripten")]
    pub fn step_frame(&mut self, _fps: u32) {
        panic!("flip is not supported for Web");
    }

    // poll_events is implemented in poll_events.rs

    pub fn gl_profile(&self) -> GLProfile {
        let mut value = 0i32;
        unsafe { SDL_GL_GetAttribute(SDL_GL_CONTEXT_PROFILE_MASK, &raw mut value) };

        if value & SDL_GL_CONTEXT_PROFILE_CORE as i32 != 0 {
            GLProfile::Gl
        } else if value & SDL_GL_CONTEXT_PROFILE_ES as i32 != 0 {
            GLProfile::Gles
        } else {
            GLProfile::None
        }
    }

    pub fn gl_context(&mut self) -> &'static mut Context {
        unsafe { &mut *self.gl_context }
    }
}
