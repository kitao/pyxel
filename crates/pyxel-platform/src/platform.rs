use std::mem::transmute;
use std::process::exit;
use std::ptr::null_mut;

use cfg_if::cfg_if;
use glow::Context as GlowContext;

use crate::audio::init_audio;
use crate::sdl2_sys::*;
use crate::window::init_window;

pub struct Platform {
    pub window: *mut SDL_Window,
    pub glow_context: *mut GlowContext,
    pub controllers: Vec<*mut SDL_GameController>,
    pub audio_device_id: SDL_AudioDeviceID,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

static mut PLATFORM: *mut Platform = null_mut();

pub fn platform() -> &'static mut Platform {
    unsafe { &mut *PLATFORM }
}

pub fn init<'a, F: FnOnce(u32, u32) -> (&'a str, u32, u32)>(window_params: F) {
    if unsafe { SDL_Init(SDL_INIT_VIDEO | SDL_INIT_GAMECONTROLLER | SDL_INIT_AUDIO) } < 0 {
        panic!("Failed to initialize SDL2");
    }
    cfg_if! {
        if #[cfg(target_os = "emscripten")] {
            unsafe {
                SDL_GL_SetAttribute(
                    SDL_GL_CONTEXT_PROFILE_MASK,
                    SDL_GL_CONTEXT_PROFILE_ES as i32,
                );
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 0);
            }
        } else {
            unsafe {
                SDL_GL_SetAttribute(
                    SDL_GL_CONTEXT_PROFILE_MASK,
                    SDL_GL_CONTEXT_PROFILE_CORE as i32,
                );
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 3);
                SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 3);
            }
        }
    }
    let mut display_mode = SDL_DisplayMode {
        format: 0,
        w: 0,
        h: 0,
        refresh_rate: 0,
        driverdata: null_mut(),
    };
    if unsafe { SDL_GetCurrentDisplayMode(0, &mut display_mode as *mut SDL_DisplayMode) } != 0 {
        panic!("Failed to get display size");
    }
    let (title, width, height) = window_params(display_mode.w as u32, display_mode.h as u32);
    let (window, glow_context) = init_window(title, width, height);
    let controllers = Vec::new();
    let audio_device_id = init_audio(0, 0, 0);
    unsafe {
        PLATFORM = transmute(Box::new(Platform {
            window,
            glow_context,
            controllers,
            audio_device_id,
            mouse_x: i32::MAX,
            mouse_y: i32::MAX,
        }));
    }
}

pub fn run<F: FnMut()>(mut main_loop: F) {
    cfg_if! {
        if #[cfg(target_os = "emscripten")] {
            emscripten::set_main_loop(main_loop);
        } else {
            loop {
                let start_ms = elapsed_time() as f64;
                main_loop();
                let elapsed_ms = elapsed_time() as f64 - start_ms;
                let wait_ms = 1000.0 / 60.0 - elapsed_ms;
                if wait_ms > 0.0 {
                    sleep((wait_ms / 2.0) as u32);
                }
            }
        }
    }
}

pub fn quit() {
    unsafe {
        SDL_Quit();
    }

    cfg_if! {
        if #[cfg(target_os = "emscripten")] {
            emscripten::force_exit(0);
        } else {
            exit(0);
        }
    }
}

pub fn elapsed_time() -> u32 {
    unsafe { SDL_GetTicks() }
}

pub fn sleep(ms: u32) {
    unsafe {
        SDL_Delay(ms);
    }
}

/*
#[cfg(not(target_os = "emscripten"))]
use chrono::Local;
use sdl2::controller::{
    Axis as SdlAxis, Button as SdlButton, GameController as SdlGameControllerState,
};

pub struct Platform {
    sdl_game_controller: Option<SdlGameController>,
    sdl_game_controller_states: Vec<SdlGameControllerState>,
    #[cfg(target_os = "emscripten")]
    virtual_gamepad_states: [bool; 8],
}

impl Platform {
    #[cfg(target_os = "emscripten")]
    pub fn save_file_on_web_browser(filename: &str) {
        emscripten::run_script(&format!("_savePyxelFile('{filename}');"));
    }

    fn gamepad_index(&self, game_controller_id: u32) -> u32 {
        self.sdl_game_controller_states
            .iter()
            .position(|gc| gc.instance_id() == game_controller_id)
            .unwrap() as u32
    }
}
*/

#[cfg(target_os = "emscripten")]
mod emscripten {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_void};

    #[allow(non_camel_case_types)]
    type em_arg_callback_func = unsafe extern "C" fn(*mut c_void);

    extern "C" {
        pub fn emscripten_set_main_loop_arg(
            func: em_arg_callback_func,
            arg: *mut c_void,
            fps: c_int,
            simulate_infinite_loop: c_int,
        );
        pub fn emscripten_force_exit(status: c_int);
        pub fn emscripten_run_script(script: *const c_char);
        pub fn emscripten_run_script_int(script: *const c_char) -> c_int;
        pub fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
    }

    unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
        (*arg.cast::<F>())();
    }

    pub fn set_main_loop<F: FnMut()>(callback: F) {
        unsafe {
            emscripten_set_main_loop_arg(
                callback_wrapper::<F>,
                Box::into_raw(Box::new(callback)).cast::<std::ffi::c_void>(),
                0,
                1,
            );
        }
    }

    pub fn force_exit(status: i32) {
        unsafe {
            emscripten_force_exit(status);
        }
    }

    pub fn run_script(script: &str) {
        let script = CString::new(script).unwrap();
        unsafe {
            emscripten_run_script(script.as_ptr());
        }
    }

    pub fn run_script_int(script: &str) -> i32 {
        let script = CString::new(script).unwrap();
        unsafe { emscripten_run_script_int(script.as_ptr()) }
    }

    pub fn run_script_string(script: &str) -> String {
        let script = CString::new(script).unwrap();
        unsafe {
            CStr::from_ptr(emscripten_run_script_string(script.as_ptr()))
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
