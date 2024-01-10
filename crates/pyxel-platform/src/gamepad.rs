#[cfg(target_os = "emscripten")]
use crate::emscripten::run_script_int;
use crate::event::Event;
use crate::keys::*;
use crate::platform::platform;
use crate::sdl2_sys::*;

pub enum Gamepad {
    Unused,
    Controller(i32, *mut SDL_GameController),
}

pub fn init_gamepads() -> Vec<Gamepad> {
    let mut gamepads = Vec::new();
    let num_joysticks = unsafe { SDL_NumJoysticks() };
    gamepads.extend((0..num_joysticks).filter_map(open_gamepad));
    gamepads
}

pub fn handle_controller_device_added(sdl_event: SDL_Event) {
    let device_index = unsafe { sdl_event.cdevice.which };
    if let Some(gamepad) = open_gamepad(device_index) {
        let unused_gamepad = platform()
            .gamepads
            .iter_mut()
            .find(|gamepad| matches!(gamepad, Gamepad::Unused));
        match unused_gamepad {
            Some(unused_gamepad) => {
                *unused_gamepad = gamepad;
            }
            None => {
                platform().gamepads.push(gamepad);
            }
        }
    }
}

pub fn handle_controller_device_removed(sdl_event: SDL_Event) {
    let instance_id = unsafe { sdl_event.cdevice.which };
    if let Some(gamepad) = platform()
        .gamepads
        .iter_mut()
        .find(|g| matches!(g, Gamepad::Controller(id, _) if *id == instance_id))
    {
        if let Gamepad::Controller(_, controller) = gamepad {
            unsafe {
                SDL_GameControllerClose(*controller);
            }
            *gamepad = Gamepad::Unused;
        }
    }
}

pub fn handle_controller_axis_motion(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.caxis.which }) {
        let axis = unsafe { sdl_event.caxis.axis } as i32;
        let key = controller_axis_to_key(axis);
        if key != KEY_UNKNOWN {
            events.push(Event::KeyValueChanged {
                key: key + key_offset,
                value: unsafe { sdl_event.caxis.value } as i32,
            });
        }
    }
    events
}

pub fn handle_controller_button_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.cbutton.which }) {
        let button = unsafe { sdl_event.cbutton.button } as i32;
        let key = controller_button_to_key(button);
        if key != KEY_UNKNOWN {
            events.push(Event::KeyPressed {
                key: key + key_offset,
            });
        }
    }
    events
}

pub fn handle_controller_button_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.cbutton.which }) {
        let button = unsafe { sdl_event.cbutton.button } as i32;
        let key = controller_button_to_key(button);
        if key != KEY_UNKNOWN {
            events.push(Event::KeyReleased {
                key: key + key_offset,
            });
        }
    }
    events
}

#[cfg(target_os = "emscripten")]
pub fn handle_joy_button_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.jbutton.which }) {
        let button = unsafe { sdl_event.jbutton.button } as i32;
        let key = joystick_button_to_key(button);
        if key != KEY_UNKNOWN {
            events.push(Event::KeyPressed {
                key: key + key_offset,
            });
        }
    }
    events
}

#[cfg(target_os = "emscripten")]
pub fn handle_joy_button_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if let Some(key_offset) = gamepad_key_offset(unsafe { sdl_event.jbutton.which }) {
        let button = unsafe { sdl_event.jbutton.button } as i32;
        let key = joystick_button_to_key(button);
        if key != KEY_UNKNOWN {
            events.push(Event::KeyReleased {
                key: key + key_offset,
            });
        }
    }
    events
}

#[cfg(target_os = "emscripten")]
pub fn handle_virtual_gamepad_inputs() -> Vec<Event> {
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
        let button_state = run_script_int(&format!("_virtualGamepadStates[{i}];")) != 0;
        if button_state != platform().virtual_gamepad_states[i] {
            platform().virtual_gamepad_states[i] = button_state;
            if button_state {
                events.push(Event::KeyPressed { key: *button });
            } else {
                events.push(Event::KeyReleased { key: *button });
            };
        }
    }
    events
}

fn open_gamepad(device_index: i32) -> Option<Gamepad> {
    let controller = unsafe { SDL_GameControllerOpen(device_index) };
    if controller.is_null() {
        None
    } else {
        let instance_id = unsafe { SDL_JoystickGetDeviceInstanceID(device_index) };
        Some(Gamepad::Controller(instance_id, controller))
    }
}

fn gamepad_key_offset(instance_id: i32) -> Option<Key> {
    platform()
        .gamepads
        .iter()
        .enumerate()
        .find_map(|(index, slot)| match slot {
            Gamepad::Controller(id, _) if *id == instance_id => Some(index as Key),
            _ => None,
        })
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
