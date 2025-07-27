use std::ptr::addr_of_mut;

use glow::Context;

use crate::event::Event;
use crate::platform::GlProfile;
use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

impl PlatformSdl2 {
    fn start_loop<F: FnMut()>(&mut self, mut callback: F) {
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
        //
    }

    fn poll_events(&mut self) -> Vec<Event> {
        Vec::new() // TODO
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
}

/*
pub fn handle_window_event(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    match unsafe { sdl_event.window.event } as SDL_WindowEventID {
        SDL_WINDOWEVENT_SHOWN | SDL_WINDOWEVENT_MAXIMIZED | SDL_WINDOWEVENT_RESTORED => {
            events.push(Event::WindowShown);
        }
        SDL_WINDOWEVENT_HIDDEN | SDL_WINDOWEVENT_MINIMIZED => {
            events.push(Event::WindowHidden);
        }
        _ => {}
    }
    events
}

pub fn handle_drop_file(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    unsafe {
        SDL_RaiseWindow(platform().window);
    }

    let filename = unsafe { CStr::from_ptr(sdl_event.drop.file) };
    let filename = filename.to_string_lossy().into_owned();
    events.push(Event::FileDropped { filename });

    unsafe {
        SDL_free(sdl_event.drop.file.cast());
    }
    events
}

pub fn handle_quit() -> Vec<Event> {
    vec![Event::Quit]
}
*/

/*
use std::mem::zeroed;
use std::ptr::addr_of_mut;

use crate::gamepad::{
    handle_controller_axis_motion, handle_controller_button_down, handle_controller_button_up,
    handle_controller_device_added, handle_controller_device_removed,
};
#[cfg(target_os = "emscripten")]
use crate::gamepad::{handle_joy_button_down, handle_joy_button_up, handle_virtual_gamepad_inputs};
use crate::keyboard::{handle_key_down, handle_key_up, handle_text_input};
use crate::keys::{Key, KeyValue};
use crate::mouse::{
    handle_mouse_button_down, handle_mouse_button_up, handle_mouse_motion, handle_mouse_wheel,
};
use crate::sdl2_sys::*;
use crate::window::{handle_drop_file, handle_quit, handle_window_event};

#[derive(Clone)]
pub enum Event {
    WindowShown,
    WindowHidden,
    KeyPressed { key: Key },
    KeyReleased { key: Key },
    KeyValueChanged { key: Key, value: KeyValue },
    TextInput { text: String },
    FileDropped { filename: String },
    Quit,
}

pub fn poll_events() -> Vec<Event> {
    let mut pyxel_events = Vec::new();
    let mut sdl_event: SDL_Event = unsafe { zeroed() };

    while unsafe { SDL_PollEvent(addr_of_mut!(sdl_event)) } != 0 {
        match unsafe { sdl_event.type_ as SDL_EventType } {
            // Window
            SDL_WINDOWEVENT => {
                pyxel_events.extend(handle_window_event(sdl_event));
            }
            SDL_DROPFILE => {
                pyxel_events.extend(handle_drop_file(sdl_event));
            }
            SDL_QUIT => {
                pyxel_events.extend(handle_quit());
            }

            // Keyboard
            SDL_KEYDOWN => {
                pyxel_events.extend(handle_key_down(sdl_event));
            }
            SDL_KEYUP => {
                pyxel_events.extend(handle_key_up(sdl_event));
            }
            SDL_TEXTINPUT => {
                pyxel_events.extend(handle_text_input(sdl_event));
            }

            // Mouse
            SDL_MOUSEBUTTONDOWN => {
                pyxel_events.extend(handle_mouse_button_down(sdl_event));
            }
            SDL_MOUSEBUTTONUP => {
                pyxel_events.extend(handle_mouse_button_up(sdl_event));
            }
            SDL_MOUSEWHEEL => {
                pyxel_events.extend(handle_mouse_wheel(sdl_event));
            }

            // Gamepad
            SDL_CONTROLLERDEVICEADDED => {
                handle_controller_device_added(sdl_event);
            }
            SDL_CONTROLLERDEVICEREMOVED => {
                handle_controller_device_removed(sdl_event);
            }
            SDL_CONTROLLERAXISMOTION => {
                pyxel_events.extend(handle_controller_axis_motion(sdl_event));
            }
            SDL_CONTROLLERBUTTONDOWN => {
                pyxel_events.extend(handle_controller_button_down(sdl_event));
            }
            SDL_CONTROLLERBUTTONUP => {
                pyxel_events.extend(handle_controller_button_up(sdl_event));
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONDOWN => {
                pyxel_events.extend(handle_joy_button_down(sdl_event));
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONUP => {
                pyxel_events.extend(handle_joy_button_up(sdl_event));
            }
            _ => {}
        }
    }

    pyxel_events.extend(handle_mouse_motion());
    #[cfg(target_os = "emscripten")]
    pyxel_events.extend(handle_virtual_gamepad_inputs());

    pyxel_events
}
*/

/*
*/
