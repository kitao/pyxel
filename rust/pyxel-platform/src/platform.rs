use std::mem::transmute;
use std::ptr::{addr_of_mut, null_mut};

use cfg_if::cfg_if;
use glow::Context as GlowContext;

use crate::gamepad::{init_gamepads, Gamepad};
use crate::sdl2_sys::*;
use crate::window::{init_glow, init_window};

pub struct Platform {
    pub window: *mut SDL_Window,
    pub glow_context: *mut GlowContext,
    pub audio_device_id: SDL_AudioDeviceID,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub gamepads: Vec<Gamepad>,
    #[cfg(target_os = "emscripten")]
    pub virtual_gamepad_states: [bool; 8],
}

static mut PLATFORM: *mut Platform = null_mut();

pub fn platform() -> &'static mut Platform {
    unsafe { &mut *PLATFORM }
}

pub fn init<'a, F: FnOnce(u32, u32) -> (&'a str, u32, u32)>(window_params: F) {
    assert!(
        unsafe { SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMECONTROLLER,) } >= 0,
        "Failed to initialize SDL2"
    );
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
    let (title, width, height) = window_params(display_mode.w as u32, display_mode.h as u32);
    let window = init_window(title, width, height);
    let glow_context = init_glow(window);
    let gamepads = init_gamepads();
    unsafe {
        PLATFORM = transmute::<Box<Platform>, *mut Platform>(Box::new(Platform {
            window,
            glow_context,
            audio_device_id: 0,
            mouse_x: i32::MIN,
            mouse_y: i32::MIN,
            gamepads,
            #[cfg(target_os = "emscripten")]
            virtual_gamepad_states: [false; 8],
        }));
    }
}

#[allow(unused_mut)]
pub fn run<F: FnMut()>(mut main_loop: F) {
    cfg_if! {
        if #[cfg(target_os = "emscripten")] {
            crate::emscripten::run(main_loop);
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
            crate::emscripten::exit(0);
        } else {
            std::process::exit(0);
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
