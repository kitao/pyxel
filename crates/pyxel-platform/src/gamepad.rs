use crate::keys::*;
use crate::platform::platform;
use crate::sdl2_sys::*;

pub enum Gamepad {
    Unused,
    Controller(i32, *mut SDL_GameController),
}

fn open_gamepad(device_index: i32) -> Option<Gamepad> {
    let instance_id = unsafe { SDL_JoystickGetDeviceInstanceID(device_index) };
    if unsafe { SDL_IsGameController(device_index) } != 0 {
        let controller = unsafe { SDL_GameControllerOpen(device_index) };
        Some(Gamepad::Controller(instance_id, controller))
    } else {
        None
    }
}

pub fn init_gamepads() -> Vec<Gamepad> {
    let mut gamepads = Vec::new();
    let num_joysticks = unsafe { SDL_NumJoysticks() };
    gamepads.extend((0..num_joysticks).filter_map(|i| open_gamepad(i)));
    gamepads
}

pub fn add_gamepad(device_index: i32) {
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

pub fn remove_gamepad(instance_id: i32) {
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

pub fn gamepad_key_offset(instance_id: i32) -> Option<u32> {
    platform()
        .gamepads
        .iter()
        .enumerate()
        .find_map(|(index, slot)| match slot {
            Gamepad::Controller(id, _) if *id == instance_id => Some(index as u32),
            _ => None,
        })
}

pub fn controller_axis_to_key(axis: i32) -> Key {
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

pub fn controller_button_to_key(button: i32) -> Key {
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
        SDL_CONTROLLER_BUTTON_MISC1 => GAMEPAD1_BUTTON_MISC1,
        SDL_CONTROLLER_BUTTON_PADDLE1 => GAMEPAD1_BUTTON_PADDLE1,
        SDL_CONTROLLER_BUTTON_PADDLE2 => GAMEPAD1_BUTTON_PADDLE2,
        SDL_CONTROLLER_BUTTON_PADDLE3 => GAMEPAD1_BUTTON_PADDLE3,
        SDL_CONTROLLER_BUTTON_PADDLE4 => GAMEPAD1_BUTTON_PADDLE4,
        SDL_CONTROLLER_BUTTON_TOUCHPAD => GAMEPAD1_BUTTON_TOUCHPAD,
        _ => KEY_UNKNOWN,
    }
}

#[cfg(target_os = "emscripten")]
pub fn joystick_button_to_key(button: i32) -> Key {
    match button {
        12 => GAMEPAD1_BUTTON_DPAD_UP,
        13 => GAMEPAD1_BUTTON_DPAD_DOWN,
        14 => GAMEPAD1_BUTTON_DPAD_LEFT,
        15 => GAMEPAD1_BUTTON_DPAD_RIGHT,
        _ => KEY_UNKNOWN,
    }
}
