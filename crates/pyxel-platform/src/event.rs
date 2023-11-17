use std::ffi::CStr;
use std::mem::zeroed;
use std::ptr::addr_of_mut;

use crate::gamepad::{
    controller_axis_motion, controller_button_down, controller_button_up, controller_device_added,
    controller_device_removed,
};
#[cfg(target_os = "emscripten")]
use crate::gamepad::{handle_virtual_gamepad, joy_button_down, joy_button_up};
use crate::keyboard::{key_down, key_up, text_input};
use crate::keys::*;
use crate::mouse::{handle_mouse_motion, mouse_button_down, mouse_button_up, mouse_wheel};
use crate::platform::platform;
use crate::sdl2_sys::*;

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
        match unsafe { sdl_event.type_ } {
            SDL_WINDOWEVENT => match unsafe { sdl_event.window.event } as u32 {
                SDL_WINDOWEVENT_SHOWN | SDL_WINDOWEVENT_MAXIMIZED | SDL_WINDOWEVENT_RESTORED => {
                    pyxel_events.push(Event::WindowShown);
                }
                SDL_WINDOWEVENT_HIDDEN | SDL_WINDOWEVENT_MINIMIZED => {
                    pyxel_events.push(Event::WindowHidden);
                }
                _ => {}
            },
            SDL_KEYDOWN => {
                pyxel_events.extend(key_down(sdl_event));
            }
            SDL_KEYUP => {
                pyxel_events.extend(key_up(sdl_event));
            }
            SDL_MOUSEBUTTONDOWN => {
                pyxel_events.extend(mouse_button_down(sdl_event));
            }
            SDL_MOUSEBUTTONUP => {
                pyxel_events.extend(mouse_button_up(sdl_event));
            }
            SDL_MOUSEWHEEL => {
                pyxel_events.extend(mouse_wheel(sdl_event));
            }
            SDL_CONTROLLERDEVICEADDED => {
                controller_device_added(sdl_event);
            }
            SDL_CONTROLLERDEVICEREMOVED => {
                controller_device_removed(sdl_event);
            }
            SDL_CONTROLLERAXISMOTION => {
                pyxel_events.extend(controller_axis_motion(sdl_event));
            }
            SDL_CONTROLLERBUTTONDOWN => {
                pyxel_events.extend(controller_button_down(sdl_event));
            }
            SDL_CONTROLLERBUTTONUP => {
                pyxel_events.extend(controller_button_up(sdl_event));
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONDOWN => {
                pyxel_events.extend(joy_button_down(sdl_event));
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONUP => {
                pyxel_events.extend(joy_button_up(sdl_event));
            }
            SDL_TEXTINPUT => {
                pyxel_events.extend(text_input(sdl_event));
            }
            SDL_DROPFILE => {
                unsafe {
                    SDL_RaiseWindow(platform().window);
                }
                let filename = unsafe { CStr::from_ptr(sdl_event.drop.file) };
                let filename = filename.to_string_lossy().into_owned();
                pyxel_events.push(Event::FileDropped { filename });
                unsafe {
                    SDL_free(sdl_event.drop.file.cast());
                }
            }
            SDL_QUIT => {
                pyxel_events.push(Event::Quit);
            }
            _ => {}
        }
    }
    pyxel_events.extend(handle_mouse_motion());
    #[cfg(target_os = "emscripten")]
    pyxel_events.extend(handle_virtual_gamepad());

    pyxel_events
}
