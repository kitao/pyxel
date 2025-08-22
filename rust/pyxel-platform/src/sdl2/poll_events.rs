use std::ffi::{c_char, CStr};
use std::mem::zeroed;
use std::ptr::addr_of_mut;

use crate::event::Event;
use crate::key::{
    Key, GAMEPAD1_AXIS_LEFTX, GAMEPAD1_AXIS_LEFTY, GAMEPAD1_AXIS_RIGHTX, GAMEPAD1_AXIS_RIGHTY,
    GAMEPAD1_AXIS_TRIGGERLEFT, GAMEPAD1_AXIS_TRIGGERRIGHT, GAMEPAD1_BUTTON_A, GAMEPAD1_BUTTON_B,
    GAMEPAD1_BUTTON_BACK, GAMEPAD1_BUTTON_DPAD_DOWN, GAMEPAD1_BUTTON_DPAD_LEFT,
    GAMEPAD1_BUTTON_DPAD_RIGHT, GAMEPAD1_BUTTON_DPAD_UP, GAMEPAD1_BUTTON_GUIDE,
    GAMEPAD1_BUTTON_LEFTSHOULDER, GAMEPAD1_BUTTON_LEFTSTICK, GAMEPAD1_BUTTON_RIGHTSHOULDER,
    GAMEPAD1_BUTTON_RIGHTSTICK, GAMEPAD1_BUTTON_START, GAMEPAD1_BUTTON_X, GAMEPAD1_BUTTON_Y,
    GAMEPAD_KEY_INDEX_INTERVAL, KEY_ALT, KEY_CTRL, KEY_GUI, KEY_LALT, KEY_LCTRL, KEY_LGUI,
    KEY_LSHIFT, KEY_RALT, KEY_RCTRL, KEY_RGUI, KEY_RSHIFT, KEY_SHIFT, KEY_UNKNOWN,
    MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_X1, MOUSE_BUTTON_X2,
    MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
};
use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_run_script_int(script: *const c_char) -> std::os::raw::c_int;
}

pub enum Gamepad {
    Unused,
    Controller(i32, *mut SDL_GameController),
}

impl Gamepad {
    pub fn open(device_index: i32) -> Option<Gamepad> {
        let controller = unsafe { SDL_GameControllerOpen(device_index) };
        if controller.is_null() {
            None
        } else {
            let instance_id = unsafe { SDL_JoystickGetDeviceInstanceID(device_index) };
            Some(Gamepad::Controller(instance_id, controller))
        }
    }

    pub fn close(&mut self) {
        if let Gamepad::Controller(_, controller) = self {
            unsafe {
                SDL_GameControllerClose(*controller);
            }
            *self = Gamepad::Unused;
        }
    }
}

impl PlatformSdl2 {
    pub fn poll_events(&mut self) -> Vec<Event> {
        let mut pyxel_events = Vec::new();
        let mut sdl_event: SDL_Event = unsafe { zeroed() };

        while unsafe { SDL_PollEvent(addr_of_mut!(sdl_event)) } != 0 {
            match unsafe { sdl_event.type_ as SDL_EventType } {
                //
                // Window
                //
                SDL_WINDOWEVENT => match unsafe { sdl_event.window.event } as SDL_WindowEventID {
                    SDL_WINDOWEVENT_SHOWN
                    | SDL_WINDOWEVENT_MAXIMIZED
                    | SDL_WINDOWEVENT_RESTORED => {
                        pyxel_events.push(Event::WindowShown);
                    }
                    SDL_WINDOWEVENT_HIDDEN | SDL_WINDOWEVENT_MINIMIZED => {
                        pyxel_events.push(Event::WindowHidden);
                    }
                    _ => {}
                },

                SDL_DROPFILE => {
                    unsafe {
                        SDL_RaiseWindow(self.window);
                    }

                    let filename = unsafe { CStr::from_ptr(sdl_event.drop.file) };
                    let filename = filename.to_string_lossy().into_owned();
                    pyxel_events.push(Event::FileDropped { filename });

                    unsafe {
                        SDL_free(sdl_event.drop.file.cast());
                    }
                }

                SDL_QUIT => {
                    pyxel_events.extend(vec![Event::Quit]);
                }

                //
                // Keyboard
                //
                SDL_KEYDOWN => {
                    if unsafe { sdl_event.key.repeat } == 0 {
                        let key = unsafe { sdl_event.key.keysym.sym } as Key;
                        pyxel_events.push(Event::KeyPressed { key });

                        if let Some(unified_key) = key_to_virtual_key(key) {
                            pyxel_events.push(Event::KeyPressed { key: unified_key });
                        }
                    }
                }

                SDL_KEYUP => {
                    if unsafe { sdl_event.key.repeat } == 0 {
                        let key = unsafe { sdl_event.key.keysym.sym } as Key;
                        pyxel_events.push(Event::KeyReleased { key });

                        if let Some(unified_key) = key_to_virtual_key(key) {
                            pyxel_events.push(Event::KeyReleased { key: unified_key });
                        }
                    }
                }

                SDL_TEXTINPUT => unsafe {
                    let c_str = CStr::from_ptr(sdl_event.text.text.as_ptr().cast::<c_char>());
                    if let Ok(text) = c_str.to_str() {
                        let text = text.to_string();
                        pyxel_events.push(Event::TextInput { text });
                    }
                },

                //
                // Mouse Button
                //
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

                //
                // Gamepad
                //
                SDL_CONTROLLERDEVICEADDED => {
                    let device_index = unsafe { sdl_event.cdevice.which };
                    if let Some(gamepad) = Gamepad::open(device_index) {
                        let unused_gamepad = self
                            .gamepads
                            .iter_mut()
                            .find(|gamepad| matches!(gamepad, Gamepad::Unused));

                        match unused_gamepad {
                            Some(unused_gamepad) => {
                                *unused_gamepad = gamepad;
                            }
                            None => {
                                self.gamepads.push(gamepad);
                            }
                        }
                    }
                }

                SDL_CONTROLLERDEVICEREMOVED => {
                    let instance_id = unsafe { sdl_event.cdevice.which };
                    if let Some(gamepad) = self
                        .gamepads
                        .iter_mut()
                        .find(|g| matches!(g, Gamepad::Controller(id, _) if *id == instance_id))
                    {
                        gamepad.close();
                    }
                }

                SDL_CONTROLLERAXISMOTION => {
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.caxis.which })
                    {
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
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.cbutton.which })
                    {
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
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.cbutton.which })
                    {
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
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.jbutton.which })
                    {
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
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.jbutton.which })
                    {
                        let button = unsafe { sdl_event.jbutton.button } as i32;
                        let key = joystick_button_to_key(button);
                        if key != KEY_UNKNOWN {
                            pyxel_events.push(Event::KeyReleased {
                                key: key + key_offset,
                            });
                        }
                    }
                }

                _ => {}
            }
        }

        //
        // Mouse Motion
        //
        {
            let mut mouse_x = i32::MIN;
            let mut mouse_y = i32::MIN;

            if unsafe { SDL_GetWindowFlags(self.window) } & SDL_WINDOW_INPUT_FOCUS as Uint32 != 0 {
                unsafe {
                    SDL_GetGlobalMouseState(&raw mut mouse_x, &raw mut mouse_y);
                }
            }

            if mouse_x != self.mouse_x || mouse_y != self.mouse_y {
                let (window_x, window_y) = crate::window_pos();

                pyxel_events.push(Event::KeyValueChanged {
                    key: MOUSE_POS_X,
                    value: mouse_x - window_x,
                });
                pyxel_events.push(Event::KeyValueChanged {
                    key: MOUSE_POS_Y,
                    value: mouse_y - window_y,
                });
            }
        }

        #[cfg(target_os = "emscripten")]
        pyxel_events.extend({
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

            let mut events = Vec::new();

            for (i, button) in INDEX_TO_BUTTON.iter().enumerate() {
                let button_state = unsafe {
                    let script =
                        std::ffi::CString::new(format!("_virtualGamepadStates[{i}];")).unwrap();
                    emscripten_run_script_int(script.as_ptr()) != 0
                };
                if button_state != self.virtual_gamepad_states[i] {
                    self.virtual_gamepad_states[i] = button_state;
                    if button_state {
                        events.push(Event::KeyPressed { key: *button });
                    } else {
                        events.push(Event::KeyReleased { key: *button });
                    }
                }
            }

            events
        });

        pyxel_events
    }

    fn gamepad_key_offset(&self, instance_id: i32) -> Option<Key> {
        self.gamepads
            .iter()
            .enumerate()
            .find_map(|(index, slot)| match slot {
                Gamepad::Controller(id, _) if *id == instance_id => {
                    Some(GAMEPAD_KEY_INDEX_INTERVAL * index as Key)
                }
                _ => None,
            })
    }
}

fn key_to_virtual_key(key: Key) -> Option<Key> {
    match key {
        KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
        KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
        KEY_LALT | KEY_RALT => Some(KEY_ALT),
        KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
        _ => None,
    }
}

fn controller_axis_to_key(axis: i32) -> Key {
    match axis {
        SDL_CONTROLLER_AXIS_LEFTX => GAMEPAD1_AXIS_LEFTX,
        SDL_CONTROLLER_AXIS_LEFTY => GAMEPAD1_AXIS_LEFTY,
        SDL_CONTROLLER_AXIS_RIGHTX => GAMEPAD1_AXIS_RIGHTX,
        SDL_CONTROLLER_AXIS_RIGHTY => GAMEPAD1_AXIS_RIGHTY,
        SDL_CONTROLLER_AXIS_TRIGGERLEFT => GAMEPAD1_AXIS_TRIGGERLEFT,
        SDL_CONTROLLER_AXIS_TRIGGERRIGHT => GAMEPAD1_AXIS_TRIGGERRIGHT,
        _ => KEY_UNKNOWN,
    }
}

fn controller_button_to_key(button: i32) -> Key {
    match button {
        SDL_CONTROLLER_BUTTON_A => GAMEPAD1_BUTTON_A,
        SDL_CONTROLLER_BUTTON_B => GAMEPAD1_BUTTON_B,
        SDL_CONTROLLER_BUTTON_X => GAMEPAD1_BUTTON_X,
        SDL_CONTROLLER_BUTTON_Y => GAMEPAD1_BUTTON_Y,
        SDL_CONTROLLER_BUTTON_BACK => GAMEPAD1_BUTTON_BACK,
        SDL_CONTROLLER_BUTTON_GUIDE => GAMEPAD1_BUTTON_GUIDE,
        SDL_CONTROLLER_BUTTON_START => GAMEPAD1_BUTTON_START,
        SDL_CONTROLLER_BUTTON_LEFTSTICK => GAMEPAD1_BUTTON_LEFTSTICK,
        SDL_CONTROLLER_BUTTON_RIGHTSTICK => GAMEPAD1_BUTTON_RIGHTSTICK,
        SDL_CONTROLLER_BUTTON_LEFTSHOULDER => GAMEPAD1_BUTTON_LEFTSHOULDER,
        SDL_CONTROLLER_BUTTON_RIGHTSHOULDER => GAMEPAD1_BUTTON_RIGHTSHOULDER,
        SDL_CONTROLLER_BUTTON_DPAD_UP => GAMEPAD1_BUTTON_DPAD_UP,
        SDL_CONTROLLER_BUTTON_DPAD_DOWN => GAMEPAD1_BUTTON_DPAD_DOWN,
        SDL_CONTROLLER_BUTTON_DPAD_LEFT => GAMEPAD1_BUTTON_DPAD_LEFT,
        SDL_CONTROLLER_BUTTON_DPAD_RIGHT => GAMEPAD1_BUTTON_DPAD_RIGHT,
        _ => KEY_UNKNOWN,
    }
}

#[cfg(target_os = "emscripten")]
fn joystick_button_to_key(button: i32) -> Key {
    match button {
        12 => GAMEPAD1_BUTTON_DPAD_UP,
        13 => GAMEPAD1_BUTTON_DPAD_DOWN,
        14 => GAMEPAD1_BUTTON_DPAD_LEFT,
        15 => GAMEPAD1_BUTTON_DPAD_RIGHT,
        _ => KEY_UNKNOWN,
    }
}
