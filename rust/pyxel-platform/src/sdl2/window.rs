use std::ffi::CString;
use std::mem::transmute;
use std::ptr::{addr_of_mut, copy_nonoverlapping, null_mut};

use glow::Context;

use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

impl PlatformSdl2 {
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

            self.gl_context = transmute(Box::new(Context::from_loader_function(|s| {
                SDL_GL_GetProcAddress(s.as_ptr().cast()).cast_const()
            })));
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

            let dst_pixels = (*surface).pixels as *mut u32;
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
}

/*
pub fn swap_window() {
    unsafe {
        SDL_GL_SwapWindow(platform().window);
    }
}
*/

/*
use cfg_if::cfg_if;

use crate::event::Event;
use crate::keys::{
    KEY_UNKNOWN, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_X1,
    MOUSE_BUTTON_X2, MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
};
use crate::platform::platform;
use crate::sdl2_sys::*;

pub fn handle_mouse_button_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    let key = match unsafe { sdl_event.button.button } as u32 {
        SDL_BUTTON_LEFT => MOUSE_BUTTON_LEFT,
        SDL_BUTTON_MIDDLE => MOUSE_BUTTON_MIDDLE,
        SDL_BUTTON_RIGHT => MOUSE_BUTTON_RIGHT,
        SDL_BUTTON_X1 => MOUSE_BUTTON_X1,
        SDL_BUTTON_X2 => MOUSE_BUTTON_X2,
        _ => KEY_UNKNOWN,
    };

    if key != KEY_UNKNOWN {
        events.push(Event::KeyPressed { key });
    }

    events
}

pub fn handle_mouse_button_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    let key = match unsafe { sdl_event.button.button } as u32 {
        SDL_BUTTON_LEFT => MOUSE_BUTTON_LEFT,
        SDL_BUTTON_MIDDLE => MOUSE_BUTTON_MIDDLE,
        SDL_BUTTON_RIGHT => MOUSE_BUTTON_RIGHT,
        SDL_BUTTON_X1 => MOUSE_BUTTON_X1,
        SDL_BUTTON_X2 => MOUSE_BUTTON_X2,
        _ => KEY_UNKNOWN,
    };

    if key != KEY_UNKNOWN {
        events.push(Event::KeyReleased { key });
    }

    events
}

pub fn handle_mouse_wheel(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    events.push(Event::KeyValueChanged {
        key: MOUSE_WHEEL_X,
        value: unsafe { sdl_event.wheel.x },
    });
    events.push(Event::KeyValueChanged {
        key: MOUSE_WHEEL_Y,
        value: unsafe { sdl_event.wheel.y },
    });

    events
}

pub fn handle_mouse_motion() -> Vec<Event> {
    let mut events = Vec::new();
    let mut mouse_x = i32::MIN;
    let mut mouse_y = i32::MIN;

    if unsafe { SDL_GetWindowFlags(platform().window) } & SDL_WINDOW_INPUT_FOCUS as Uint32 != 0 {
        unsafe {
            SDL_GetGlobalMouseState(&raw mut mouse_x, &raw mut mouse_y);
        }
    }

    if mouse_x != platform().mouse_x || mouse_y != platform().mouse_y {
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                let (window_x, window_y) = (0, 0);
            } else {
                let (window_x, window_y) = crate::window_pos();
            }
        }

        events.push(Event::KeyValueChanged {
            key: MOUSE_POS_X,
            value: mouse_x - window_x,
        });
        events.push(Event::KeyValueChanged {
            key: MOUSE_POS_Y,
            value: mouse_y - window_y,
        });
    }

    events
}
*/

/*
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::event::Event;
use crate::keys::{
    Key, KEY_ALT, KEY_CTRL, KEY_GUI, KEY_LALT, KEY_LCTRL, KEY_LGUI, KEY_LSHIFT, KEY_RALT,
    KEY_RCTRL, KEY_RGUI, KEY_RSHIFT, KEY_SHIFT,
};
use crate::sdl2_sys::*;

pub fn handle_key_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    if unsafe { sdl_event.key.repeat } == 0 {
        let key = unsafe { sdl_event.key.keysym.sym } as Key;
        events.push(Event::KeyPressed { key });

        if let Some(unified_key) = to_unified_key(key) {
            events.push(Event::KeyPressed { key: unified_key });
        }
    }

    events
}

pub fn handle_key_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    if unsafe { sdl_event.key.repeat } == 0 {
        let key = unsafe { sdl_event.key.keysym.sym } as Key;
        events.push(Event::KeyReleased { key });

        if let Some(unified_key) = to_unified_key(key) {
            events.push(Event::KeyReleased { key: unified_key });
        }
    }

    events
}

pub fn handle_text_input(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();

    unsafe {
        let c_str = CStr::from_ptr(sdl_event.text.text.as_ptr().cast::<c_char>());
        if let Ok(text) = c_str.to_str() {
            let text = text.to_string();
            events.push(Event::TextInput { text });
        }
    }

    events
}

fn to_unified_key(key: Key) -> Option<Key> {
    match key {
        KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
        KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
        KEY_LALT | KEY_RALT => Some(KEY_ALT),
        KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
        _ => None,
    }
}
*/
