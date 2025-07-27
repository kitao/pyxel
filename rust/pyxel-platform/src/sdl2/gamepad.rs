use crate::event::Event;
use crate::key::*;
use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

pub enum Gamepad {
    Unused,
    Controller(i32, *mut SDL_GameController),
}

impl PlatformSdl2 {
    pub fn init_gamepads(&mut self) -> Vec<Gamepad> {
        let mut gamepads = Vec::new();
        let num_joysticks = unsafe { SDL_NumJoysticks() };
        gamepads.extend((0..num_joysticks).filter_map(open_gamepad));
        gamepads
    }

    pub fn handle_controller_device_added(&mut self, sdl_event: SDL_Event) {
        let device_index = unsafe { sdl_event.cdevice.which };
        if let Some(gamepad) = open_gamepad(device_index) {
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

    pub fn handle_controller_device_removed(&mut self, sdl_event: SDL_Event) {
        let instance_id = unsafe { sdl_event.cdevice.which };
        if let Some(gamepad) = self
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
            Gamepad::Controller(id, _) if *id == instance_id => {
                Some(GAMEPAD_KEY_INDEX_INTERVAL * index as Key)
            }
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
