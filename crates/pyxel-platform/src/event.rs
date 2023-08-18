use std::ffi::CStr;
use std::mem;
use std::str;

use crate::keys::*;
use crate::platform::window;
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

fn to_unified_key(key: Key) -> Option<Key> {
    match key {
        KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
        KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
        KEY_LALT | KEY_RALT => Some(KEY_ALT),
        KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
        _ => None,
    }
}

pub fn poll_events() -> Vec<Event> {
    let mut events = Vec::new();
    let mut sdl_event: SDL_Event = unsafe { mem::zeroed() };
    while unsafe { SDL_PollEvent(&mut sdl_event as *mut _) } != 0 {
        match unsafe { sdl_event.type_ } {
            SDL_WINDOWEVENT => match unsafe { sdl_event.window.event } as u32 {
                SDL_WINDOWEVENT_SHOWN | SDL_WINDOWEVENT_MAXIMIZED | SDL_WINDOWEVENT_RESTORED => {
                    events.push(Event::WindowShown);
                }
                SDL_WINDOWEVENT_HIDDEN | SDL_WINDOWEVENT_MINIMIZED => {
                    events.push(Event::WindowHidden);
                }
                _ => {}
            },
            SDL_KEYDOWN => {
                if unsafe { sdl_event.key.repeat } == 0 {
                    let key = unsafe { sdl_event.key.keysym.sym } as Key;
                    events.push(Event::KeyPressed { key });
                    if let Some(unified_key) = to_unified_key(key) {
                        events.push(Event::KeyPressed { key: unified_key });
                    }
                }
            }
            SDL_KEYUP => {
                if unsafe { sdl_event.key.repeat } == 0 {
                    let key = unsafe { sdl_event.key.keysym.sym } as Key;
                    events.push(Event::KeyReleased { key });
                    if let Some(unified_key) = to_unified_key(key) {
                        events.push(Event::KeyReleased { key: unified_key });
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
                    events.push(Event::KeyPressed { key });
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
                    events.push(Event::KeyReleased { key });
                }
            }
            SDL_MOUSEWHEEL => {
                events.push(Event::KeyValueChanged {
                    key: MOUSE_WHEEL_X,
                    value: unsafe { sdl_event.wheel.x },
                });
                events.push(Event::KeyValueChanged {
                    key: MOUSE_WHEEL_Y,
                    value: unsafe { sdl_event.wheel.y },
                });
            }
            SDL_CONTROLLERAXISMOTION => {
                /*
                SdlEvent::ControllerAxisMotion {
                which, axis, value, ..
                } => Event::ControllerAxisMotion {
                which: self.gamepad_index(which),
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
                */
            }
            SDL_CONTROLLERBUTTONDOWN => {
                /*
                SdlEvent::ControllerButtonDown { which, button, .. } => Event::ControllerButtonDown {
                which: self.gamepad_index(which),
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
                */
            }
            SDL_CONTROLLERBUTTONUP => {
                /*
                SdlEvent::ControllerButtonUp { which, button, .. } => Event::ControllerButtonUp {
                which: self.gamepad_index(which),
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
                */
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONDOWN => {
                /*
                SdlEvent::JoyButtonDown {
                timestamp: _,
                which,
                button_idx,
                } => Event::ControllerButtonDown {
                which: self.gamepad_index(which),
                button: match button_idx {
                12 => ControllerButton::DPadUp,
                13 => ControllerButton::DPadDown,
                14 => ControllerButton::DPadLeft,
                15 => ControllerButton::DPadRight,
                _ => continue,
                */
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYBUTTONUP => {
                /*
                SdlEvent::JoyButtonUp {
                timestamp: _,
                which,
                button_idx,
                } => Event::ControllerButtonUp {
                which: self.gamepad_index(which),
                button: match button_idx {
                12 => ControllerButton::DPadUp,
                13 => ControllerButton::DPadDown,
                14 => ControllerButton::DPadLeft,
                15 => ControllerButton::DPadRight,
                _ => {
                continue;
                }
                */
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYDEVICEADDED => {
                /*
                SdlEvent::JoyDeviceAdded {
                timestamp: _,
                which,
                } => {
                if self.sdl_game_controller.is_some() {
                if let Ok(gc) = self.sdl_game_controller.as_mut().unwrap().open(which) {
                self.sdl_game_controller_states.push(gc);
                }
                }
                continue;
                    */
            }
            #[cfg(target_os = "emscripten")]
            SDL_JOYDEVICEREMOVED => {
                /*
                SdlEvent::JoyDeviceRemoved { .. } => {
                self.sdl_game_controller_states
                .retain(SdlGameControllerState::attached);
                continue;
                }
                    */
            }
            SDL_TEXTINPUT => {
                let text = unsafe {
                    str::from_utf8(&*(&sdl_event.text.text as *const [i8] as *const [u8]))
                };
                if let Ok(text) = text {
                    let text = text.to_string();
                    events.push(Event::TextInput { text });
                }
            }
            SDL_DROPFILE => {
                unsafe {
                    SDL_RaiseWindow(window());
                }
                let filename = unsafe { CStr::from_ptr(sdl_event.drop.file) };
                let filename = filename.to_string_lossy().into_owned();
                events.push(Event::FileDropped { filename });
                unsafe {
                    SDL_free(sdl_event.drop.file as *mut _);
                }
            }
            SDL_QUIT => {
                events.push(Event::Quit);
            }
            _ => {}
        }
    }

    #[cfg(target_os = "emscripten")]
    {
        const INDEX_TO_BUTTON: [ControllerButton; 8] = [
            ControllerButton::DPadUp,
            ControllerButton::DPadDown,
            ControllerButton::DPadLeft,
            ControllerButton::DPadRight,
            ControllerButton::A,
            ControllerButton::B,
            ControllerButton::X,
            ControllerButton::Y,
        ];
        for (i, button) in INDEX_TO_BUTTON.iter().enumerate() {
            let button_state =
                emscripten::run_script_int(&format!("_virtualGamepadStates[{i}];")) != 0;
            if button_state != self.virtual_gamepad_states[i] {
                self.virtual_gamepad_states[i] = button_state;
                return if button_state {
                    Some(Event::ControllerButtonDown {
                        which: 0,
                        button: *button,
                    })
                } else {
                    Some(Event::ControllerButtonUp {
                        which: 0,
                        button: *button,
                    })
                };
            }
        }
    }

    events
}
