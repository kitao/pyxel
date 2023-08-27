use crate::sdl2_sys::*;
use std::ptr::null_mut;

static mut CONTROLLERS: [*mut SDL_GameController; 4] = [null_mut(); 4];

pub(crate) fn add_controller(device_index: i32) {
    for i in 0..4 {
        if unsafe { CONTROLLERS[i].is_null() } {
            let controller = unsafe { SDL_GameControllerOpen(device_index) };
            if !controller.is_null() {
                unsafe {
                    CONTROLLERS[i] = controller;
                }
                println!("Controller {} connected to slot {}", device_index, i);
            }
            break;
        }
    }
}

pub(crate) fn remove_controller(instance_id: SDL_JoystickID) {
    for i in 0..4 {
        if !unsafe { CONTROLLERS[i].is_null() }
            && unsafe { SDL_JoystickInstanceID(SDL_GameControllerGetJoystick(CONTROLLERS[i])) }
                == instance_id
        {
            println!("Controller in slot {} disconnected", i);
            unsafe {
                SDL_GameControllerClose(CONTROLLERS[i]);
                CONTROLLERS[i] = null_mut();
            }
            break;
        }
    }
}
