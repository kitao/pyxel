use std::str::from_utf8 as str_from_utf8;

use crate::event::Event;
use crate::keys::{
    Key, KEY_ALT, KEY_CTRL, KEY_GUI, KEY_LALT, KEY_LCTRL, KEY_LGUI, KEY_LSHIFT, KEY_RALT,
    KEY_RCTRL, KEY_RGUI, KEY_RSHIFT, KEY_SHIFT,
};
use crate::sdl2_sys::*;

pub fn handle_key_down(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if unsafe { sdl_event.key.repeat } == 0 {
        let key = unsafe { sdl_event.key.keysym.sym } as Key;
        events.push(Event::KeyPressed { key });
        if let Some(unified_key) = to_unified_key(key) {
            events.push(Event::KeyPressed { key: unified_key });
        }
    }
    events
}

pub fn handle_key_up(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    if unsafe { sdl_event.key.repeat } == 0 {
        let key = unsafe { sdl_event.key.keysym.sym } as Key;
        events.push(Event::KeyReleased { key });
        if let Some(unified_key) = to_unified_key(key) {
            events.push(Event::KeyReleased { key: unified_key });
        }
    }
    events
}

pub fn handle_text_input(sdl_event: SDL_Event) -> Vec<Event> {
    let mut events = Vec::new();
    let text = unsafe {
        let ptr = sdl_event.text.text.as_ptr().cast::<u8>();
        let slice = std::slice::from_raw_parts(ptr, sdl_event.text.text.len());
        str_from_utf8(slice)
    };
    if let Ok(text) = text {
        let text = text.to_string();
        events.push(Event::TextInput { text });
    }
    events
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
