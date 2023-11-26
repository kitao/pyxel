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
