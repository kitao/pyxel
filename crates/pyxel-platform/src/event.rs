use std::mem;

use crate::keys::*;
use crate::sdl2_sys::*;

#[derive(Clone)]
pub enum Event {
    Quit,
    Shown,
    Hidden,
    KeyDown { key: Key },
    KeyUp { key: Key },
    KeyValueChange { key: Key, value: KeyValue },
    TextInput { text: String },
    DropFile { filename: String },
}

pub fn poll_events() -> Vec<Event> {
    let mut events = Vec::new();

    loop {
        let mut sdl_event: SDL_Event = unsafe { mem::zeroed() };
        if unsafe { SDL_PollEvent(&mut sdl_event as *mut _) } == 0 {
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
            return None;
        }
        let event = match unsafe { sdl_event.type_ } {
            // System events
            SDL_QUIT => Event::Quit,
            /*SDL_EventType_SDL_DROPFILE => {
                SDL_RaiseWindow(window());
                Event::DropFile { filename }

                let file_path_cstr = std::ffi::CStr::from_ptr(event.drop.file);
                if let Ok(file_path_str) = file_path_cstr.to_str() {
                    println!("Dropped file path: {}", file_path_str);
                }
                bindings::SDL_free(event.drop.file as *mut _);
            }
            */

            /*
            // Window events
            SdlEvent::Window { win_event, .. } => match win_event {
                SdlWindowEvent::Shown | SdlWindowEvent::Maximized | SdlWindowEvent::Restored => {
                    Event::Shown
                }
                SdlWindowEvent::Hidden | SdlWindowEvent::Minimized => Event::Hidden,
                _ => continue,
            },

            // Key events
            SdlEvent::KeyDown {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => Event::KeyDown {
                keycode: keycode as u32,
            },
            SdlEvent::KeyUp {
                keycode: Some(keycode),
                repeat: false,
                ..
            } => Event::KeyUp {
                keycode: keycode as u32,
            },
            SdlEvent::TextInput { text, .. } => Event::TextInput { text },

            // Mouse events
            SdlEvent::MouseButtonDown { mouse_btn, .. } => Event::MouseButtonDown {
                button: match mouse_btn {
                    SdlMouseButton::Left => MouseButton::Left,
                    SdlMouseButton::Middle => MouseButton::Middle,
                    SdlMouseButton::Right => MouseButton::Right,
                    SdlMouseButton::X1 => MouseButton::X1,
                    SdlMouseButton::X2 => MouseButton::X2,
                    SdlMouseButton::Unknown => MouseButton::Unknown,
                },
            },
            SdlEvent::MouseButtonUp { mouse_btn, .. } => Event::MouseButtonUp {
                button: match mouse_btn {
                    SdlMouseButton::Left => MouseButton::Left,
                    SdlMouseButton::Middle => MouseButton::Middle,
                    SdlMouseButton::Right => MouseButton::Right,
                    SdlMouseButton::X1 => MouseButton::X1,
                    SdlMouseButton::X2 => MouseButton::X2,
                    SdlMouseButton::Unknown => MouseButton::Unknown,
                },
            },
            SdlEvent::MouseWheel { x, y, .. } => Event::MouseWheel { x, y },

            // Controller events
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
            }
            SdlEvent::JoyDeviceRemoved { .. } => {
                self.sdl_game_controller_states
                    .retain(SdlGameControllerState::attached);
                continue;
            }
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
                },
            },
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
            },
            #[cfg(target_os = "emscripten")]
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
                },
            },
            #[cfg(target_os = "emscripten")]
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
                },
            },
            */
            // Others
            _ => continue,
        };
        events
    }
}
