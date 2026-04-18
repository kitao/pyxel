use std::ffi::{c_char, CStr};
use std::mem::zeroed;
#[cfg(target_os = "emscripten")]
use std::sync::OnceLock;

use super::super::event::Event;
use super::super::key::{
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
use super::platform_sdl2::PlatformSdl2;
#[allow(clippy::wildcard_imports)]
use super::sdl2_sys::*;

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_run_script_int(script: *const c_char) -> std::os::raw::c_int;
}

#[cfg(target_os = "emscripten")]
static GAMEPAD_SCRIPTS: OnceLock<[std::ffi::CString; 10]> = OnceLock::new();

#[cfg(target_os = "emscripten")]
fn gamepad_scripts() -> &'static [std::ffi::CString; 10] {
    GAMEPAD_SCRIPTS.get_or_init(|| {
        std::array::from_fn(|i| {
            std::ffi::CString::new(format!("_virtualGamepadStates[{i}];")).unwrap()
        })
    })
}

pub type GamepadSlot = Option<(i32, *mut SDL_GameController)>;

pub fn open_gamepad(device_index: i32) -> Option<(i32, *mut SDL_GameController)> {
    let controller = unsafe { SDL_GameControllerOpen(device_index) };
    if controller.is_null() {
        return None;
    }
    let instance_id = unsafe { SDL_JoystickGetDeviceInstanceID(device_index) };
    Some((instance_id, controller))
}

fn close_gamepad(gamepad: &mut GamepadSlot) {
    if let Some((_, controller)) = gamepad {
        unsafe { SDL_GameControllerClose(*controller) };
        *gamepad = None;
    }
}

impl PlatformSdl2 {
    pub fn poll_events(&mut self, pyxel_events: &mut Vec<Event>) {
        let mut sdl_event: SDL_Event = unsafe { zeroed() };

        while unsafe { SDL_PollEvent(&raw mut sdl_event) } != 0 {
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
                    unsafe { SDL_RaiseWindow(self.window) };
                    let filename = unsafe { CStr::from_ptr(sdl_event.drop.file) };
                    let filename = filename.to_string_lossy().into_owned();
                    pyxel_events.push(Event::FileDropped { filename });
                    unsafe { SDL_free(sdl_event.drop.file.cast()) };
                }

                SDL_QUIT => {
                    pyxel_events.push(Event::Quit);
                }

                //
                // Keyboard
                //
                SDL_KEYDOWN | SDL_KEYUP => {
                    let key = unsafe { sdl_event.key.keysym.sym } as Key;

                    #[cfg(target_os = "emscripten")]
                    let key = correct_emscripten_key(key, unsafe { sdl_event.key.keysym.scancode }
                        as u32);

                    if unsafe { sdl_event.key.repeat } == 0 {
                        let pressed = unsafe { sdl_event.type_ } as SDL_EventType == SDL_KEYDOWN;
                        push_key_event(pyxel_events, key, pressed);
                        if let Some(unified_key) = key_to_virtual_key(key) {
                            push_key_event(pyxel_events, unified_key, pressed);
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
                    let key = Self::sdl_button_to_key(unsafe { sdl_event.button.button } as u32);
                    if key != KEY_UNKNOWN {
                        pyxel_events.push(Event::KeyPressed { key });
                    }
                }

                SDL_MOUSEBUTTONUP => {
                    let key = Self::sdl_button_to_key(unsafe { sdl_event.button.button } as u32);
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
                    if let Some(gamepad) = open_gamepad(device_index) {
                        match self.gamepads.iter_mut().find(|g| g.is_none()) {
                            Some(slot) => *slot = Some(gamepad),
                            None => self.gamepads.push(Some(gamepad)),
                        }
                    }
                }

                SDL_CONTROLLERDEVICEREMOVED => {
                    let instance_id = unsafe { sdl_event.cdevice.which };
                    if let Some(slot) = self
                        .gamepads
                        .iter_mut()
                        .find(|g| matches!(g, Some((id, _)) if *id == instance_id))
                    {
                        close_gamepad(slot);
                    }
                }

                SDL_CONTROLLERAXISMOTION => {
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.caxis.which })
                    {
                        let key = controller_axis_to_key(unsafe { sdl_event.caxis.axis } as i32);
                        if key != KEY_UNKNOWN {
                            pyxel_events.push(Event::KeyValueChanged {
                                key: key + key_offset,
                                value: unsafe { sdl_event.caxis.value } as i32,
                            });
                        }
                    }
                }

                SDL_CONTROLLERBUTTONDOWN | SDL_CONTROLLERBUTTONUP => {
                    if let Some(key_offset) =
                        self.gamepad_key_offset(unsafe { sdl_event.cbutton.which })
                    {
                        let key =
                            controller_button_to_key(unsafe { sdl_event.cbutton.button } as i32);
                        if key != KEY_UNKNOWN {
                            let pressed = unsafe { sdl_event.type_ } as SDL_EventType
                                == SDL_CONTROLLERBUTTONDOWN;
                            push_key_event(pyxel_events, key + key_offset, pressed);
                        }
                    }
                }

                _ => {}
            }
        }

        //
        // Mouse Motion (polling)
        //
        let (mouse_x, mouse_y) = if self.is_wayland || cfg!(target_os = "emscripten") {
            let (mut x, mut y) = (0, 0);
            unsafe { SDL_GetMouseState(&raw mut x, &raw mut y) };
            (x, y)
        } else {
            let (mut gx, mut gy) = (0, 0);
            unsafe { SDL_GetGlobalMouseState(&raw mut gx, &raw mut gy) };
            let (wx, wy) = self.window_pos();
            (gx - wx, gy - wy)
        };

        if mouse_x != self.mouse_x || mouse_y != self.mouse_y {
            self.mouse_x = mouse_x;
            self.mouse_y = mouse_y;
            pyxel_events.push(Event::KeyValueChanged {
                key: MOUSE_POS_X,
                value: mouse_x,
            });
            pyxel_events.push(Event::KeyValueChanged {
                key: MOUSE_POS_Y,
                value: mouse_y,
            });
        }

        //
        // Virtual Gamepad (Emscripten)
        //
        #[cfg(target_os = "emscripten")]
        {
            const INDEX_TO_BUTTON: [Key; 10] = [
                GAMEPAD1_BUTTON_DPAD_UP,
                GAMEPAD1_BUTTON_DPAD_DOWN,
                GAMEPAD1_BUTTON_DPAD_LEFT,
                GAMEPAD1_BUTTON_DPAD_RIGHT,
                GAMEPAD1_BUTTON_A,
                GAMEPAD1_BUTTON_B,
                GAMEPAD1_BUTTON_X,
                GAMEPAD1_BUTTON_Y,
                GAMEPAD1_BUTTON_START,
                GAMEPAD1_BUTTON_BACK,
            ];

            for (i, &button) in INDEX_TO_BUTTON.iter().enumerate() {
                let pressed =
                    unsafe { emscripten_run_script_int(gamepad_scripts()[i].as_ptr()) != 0 };
                if pressed != self.virtual_gamepad_states[i] {
                    self.virtual_gamepad_states[i] = pressed;
                    let event = if pressed {
                        Event::KeyPressed { key: button }
                    } else {
                        Event::KeyReleased { key: button }
                    };
                    pyxel_events.push(event);
                }
            }
        }
    }

    fn sdl_button_to_key(button: u32) -> Key {
        match button {
            SDL_BUTTON_LEFT => MOUSE_BUTTON_LEFT,
            SDL_BUTTON_MIDDLE => MOUSE_BUTTON_MIDDLE,
            SDL_BUTTON_RIGHT => MOUSE_BUTTON_RIGHT,
            SDL_BUTTON_X1 => MOUSE_BUTTON_X1,
            SDL_BUTTON_X2 => MOUSE_BUTTON_X2,
            _ => KEY_UNKNOWN,
        }
    }

    fn gamepad_key_offset(&self, instance_id: i32) -> Option<Key> {
        self.gamepads
            .iter()
            .enumerate()
            .find_map(|(index, slot)| match slot {
                Some((id, _)) if *id == instance_id => {
                    Some(GAMEPAD_KEY_INDEX_INTERVAL * index as Key)
                }
                _ => None,
            })
    }
}

// Correct SDL2 keycode on Emscripten for non-US keyboard layouts.
// Emscripten's SDL2 maps physical keys through a US-layout table, producing
// incorrect keycodes for JIS and other non-US keyboards. This function looks
// up the actual character from a persistent per-scancode correction map
// (populated by keydown listeners in pyxel.js).
//
// Using a persistent map keyed by SDL scancode (physical key) instead of a
// per-event queue ensures that keydown and keyup for the same physical key
// always receive the same correction, eliminating key sticking caused by
// queue desynchronization.
#[cfg(target_os = "emscripten")]
fn correct_emscripten_key(sdl_key: Key, scancode: u32) -> Key {
    let js_key = unsafe {
        let script = std::ffi::CString::new(format!("_scanCorrection[{scancode}]||0;")).unwrap();
        emscripten_run_script_int(script.as_ptr())
    } as u32;

    // Only correct printable ASCII keys (0x20 space .. 0x7E tilde)
    if !(0x20..=0x7E).contains(&sdl_key) || !(0x20..=0x7E).contains(&js_key) {
        return sdl_key;
    }

    // Convert uppercase letters to lowercase (Pyxel uses lowercase key constants)
    if (0x41..=0x5A).contains(&js_key) {
        return js_key + 0x20;
    }

    js_key
}

fn push_key_event(events: &mut Vec<Event>, key: Key, pressed: bool) {
    events.push(if pressed {
        Event::KeyPressed { key }
    } else {
        Event::KeyReleased { key }
    });
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
