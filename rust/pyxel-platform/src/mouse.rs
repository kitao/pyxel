use cfg_if::cfg_if;

use crate::event::Event;
use crate::keys::{
    KEY_UNKNOWN, MOUSE_BUTTON_LEFT, MOUSE_BUTTON_MIDDLE, MOUSE_BUTTON_RIGHT, MOUSE_BUTTON_X1,
    MOUSE_BUTTON_X2, MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
};
use crate::platform::platform;
use crate::sdl2_sys::*;

pub fn handle_mouse_button_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
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
    events
}

pub fn handle_mouse_button_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
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
    events
}

pub fn handle_mouse_wheel(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    events.push(Event::KeyValueChanged {
        key: MOUSE_WHEEL_X,
        value: unsafe { sdl_event.wheel.x },
    });
    events.push(Event::KeyValueChanged {
        key: MOUSE_WHEEL_Y,
        value: unsafe { sdl_event.wheel.y },
    });
    events
}

pub fn handle_mouse_motion() -> Vec<Event> {
    let mut events = Vec::new();
    let mut mouse_x = i32::MIN;
    let mut mouse_y = i32::MIN;
    if unsafe { SDL_GetWindowFlags(platform().window) } & SDL_WINDOW_INPUT_FOCUS as Uint32 != 0 {
        unsafe {
            SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y);
        }
    }
    if mouse_x != platform().mouse_x || mouse_y != platform().mouse_y {
        cfg_if! {
            if #[cfg(target_os = "emscripten")] {
                let (window_x, window_y) = (0, 0);
            } else {
                let (window_x, window_y) = crate::window_pos();
            }
        }
        events.push(Event::KeyValueChanged {
            key: MOUSE_POS_X,
            value: mouse_x - window_x,
        });
        events.push(Event::KeyValueChanged {
            key: MOUSE_POS_Y,
            value: mouse_y - window_y,
        });
    }
    events
}
