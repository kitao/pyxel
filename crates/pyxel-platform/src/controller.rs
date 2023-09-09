#[cfg(target_os = "emscripten")]
mod controller {
    use std::ptr;

    use crate::sdl2_sys::*;

    static mut CONTROLLERS: [*mut SDL_GameController; 4] = [null_mut(); 4];

    pub fn add_controller(device_index: i32) {
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

    pub fn remove_controller(instance_id: SDL_JoystickID) {
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
}

/*
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
*/
