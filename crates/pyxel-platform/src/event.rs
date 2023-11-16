use std::ffi::CStr;
use std::mem::zeroed;
use std::ptr::{addr_of, addr_of_mut};
use std::str::from_utf8 as str_from_utf8;

use cfg_if::cfg_if;

#[cfg(target_os = "emscripten")]
use crate::emscripten::run_script_int;
#[cfg(target_os = "emscripten")]
use crate::gamepad::joystick_button_to_key;
use crate::gamepad::{
    add_gamepad, controller_axis_to_key, controller_button_to_key, gamepad_key_offset,
    remove_gamepad,
};
use crate::keys::*;
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
                if unsafe { sdl_event.key.repeat } == 0 {
                    let key = unsafe { sdl_event.key.keysym.sym } as Key;
                    pyxel_events.push(Event::KeyPressed { key });
                    if let Some(unified_key) = to_unified_key(key) {
                        pyxel_events.push(Event::KeyPressed { key: unified_key });
                    }
                }
            }
            SDL_KEYUP => {
                if unsafe { sdl_event.key.repeat } == 0 {
                    let key = unsafe { sdl_event.key.keysym.sym } as Key;
                    pyxel_events.push(Event::KeyReleased { key });
                    if let Some(unified_key) = to_unified_key(key) {
                        pyxel_events.push(Event::KeyReleased { key: unified_key });
                    }
                }
            }
            SDL_MOUSEBUTTONDOWN => {
                let key = match unsafe { sdl_event.button.button } as u32 {
                    SDL_BUTTON_LEFT => MOUSE_BUTTON_LEFT,
                    SDL_BUTTON_MIDDLE => MOUSE_BUTTON_MIDDLE,
                    SDL_BUTTON_RIGHT => MOUSE_BUTTON_RIGHT,
                    SDL_BUTTON_X1 => MOUSE_BUTTON_X1,
                    SDL_BUTTON_X2 => MOUSE_BUTTON_X2,
                    _ => KEY_UNKNOWN,
                };
                if key != KEY_UNKNOWN {
                    pyxel_events.push(Event::KeyPressed { key });
                }
            }
            SDL_MOUSEBUTTONUP => {
                let key = match unsafe { sdl_event.button.button } as u32 {
                    SDL_BUTTON_LEFT => MOUSE_BUTTON_LEFT,
                    SDL_BUTTON_MIDDLE => MOUSE_BUTTON_MIDDLE,
                    SDL_BUTTON_RIGHT => MOUSE_BUTTON_RIGHT,
                    SDL_BUTTON_X1 => MOUSE_BUTTON_X1,
                    SDL_BUTTON_X2 => MOUSE_BUTTON_X2,
                    _ => KEY_UNKNOWN,
                };
                if key != KEY_UNKNOWN {
                    pyxel_events.push(Event::KeyReleased { key });
                }
            }
            SDL_MOUSEWHEEL => {
                pyxel_events.push(Event::KeyValueChanged {
                    key: MOUSE_WHEEL_X,
                    value: unsafe { sdl_event.wheel.x },
                });
                pyxel_events.push(Event::KeyValueChanged {
                    key: MOUSE_WHEEL_Y,
                    value: unsafe { sdl_event.wheel.y },
                });
            }
            SDL_CONTROLLERDEVICEADDED => {
                add_gamepad(unsafe { sdl_event.cdevice.which });
            }
            SDL_CONTROLLERDEVICEREMOVED => {
                remove_gamepad(unsafe { sdl_event.cdevice.which });
            }
            SDL_CONTROLLERAXISMOTION => {
                if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.caxis.which }) {
                    let axis = unsafe { sdl_event.caxis.axis } as i32;
                    let key = controller_axis_to_key(axis);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyValueChanged {
                            key: key + key_offset,
                            value: unsafe { sdl_event.caxis.value } as i32,
                        });
                    }
                }
            }
            SDL_CONTROLLERBUTTONDOWN => {
                if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.cbutton.which }) {
                    let button = unsafe { sdl_event.cbutton.button } as i32;
                    let key = controller_button_to_key(button);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyPressed {
                            key: key + key_offset,
                        });
                    }
                }
            }
            SDL_CONTROLLERBUTTONUP => {
                if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.cbutton.which }) {
                    let button = unsafe { sdl_event.cbutton.button } as i32;
                    let key = controller_button_to_key(button);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyReleased {
                            key: key + key_offset,
                        });
                    }
                }
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONDOWN => {
                if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.jbutton.which }) {
                    let button = unsafe { sdl_event.jbutton.button } as i32;
                    let key = joystick_button_to_key(button);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyPressed {
                            key: key + key_offset,
                        });
                    }
                }
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONUP => {
                if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.jbutton.which }) {
                    let button = unsafe { sdl_event.jbutton.button } as i32;
                    let key = joystick_button_to_key(button);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyReleased {
                            key: key + key_offset,
                        });
                    }
                }
            }
            SDL_TEXTINPUT => {
                let text = unsafe {
                    let ptr = (addr_of!(sdl_event.text.text) as *const [i8]).cast::<u8>();
                    let slice = std::slice::from_raw_parts(ptr, sdl_event.text.text.len());
                    str_from_utf8(slice)
                };
                if let Ok(text) = text {
                    let text = text.to_string();
                    pyxel_events.push(Event::TextInput { text });
                }
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

    // Mouse cursor movement
    let mut mouse_x = 0;
    let mut mouse_y = 0;
    unsafe { SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y) };
    if mouse_x != platform().mouse_x || mouse_y != platform().mouse_y {
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                let (window_x, window_y) = (0, 0);
            } else {
                let (window_x, window_y) = crate::window_pos();
            }
        }
        pyxel_events.push(Event::KeyValueChanged {
            key: MOUSE_POS_X,
            value: mouse_x - window_x,
        });
        pyxel_events.push(Event::KeyValueChanged {
            key: MOUSE_POS_Y,
            value: mouse_y - window_y,
        });
    }

    // Virtual gamepad
    #[cfg(target_os = "emscripten")]
    {
        const INDEX_TO_BUTTON: [Key; 8] = [
            GAMEPAD1_BUTTON_DPAD_UP,
            GAMEPAD1_BUTTON_DPAD_DOWN,
            GAMEPAD1_BUTTON_DPAD_LEFT,
            GAMEPAD1_BUTTON_DPAD_RIGHT,
            GAMEPAD1_BUTTON_A,
            GAMEPAD1_BUTTON_B,
            GAMEPAD1_BUTTON_X,
            GAMEPAD1_BUTTON_Y,
        ];
        for (i, button) in INDEX_TO_BUTTON.iter().enumerate() {
            let button_state = run_script_int(&format!("_virtualGamepadStates[{i}];")) != 0;
            if button_state != platform().virtual_gamepad_states[i] {
                platform().virtual_gamepad_states[i] = button_state;
                if button_state {
                    pyxel_events.push(Event::KeyPressed { key: *button });
                } else {
                    pyxel_events.push(Event::KeyReleased { key: *button });
                };
            }
        }
    }

    pyxel_events
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
