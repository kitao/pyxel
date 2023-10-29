use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::slice;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::platform::platform;
use crate::sdl2_sys::*;

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

extern "C" fn c_audio_callback(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let audio_callback = unsafe { &*(userdata as *mut Arc<Mutex<dyn AudioCallback>>) };
    let stream: &mut [i16] =
        unsafe { slice::from_raw_parts_mut(stream as *mut i16, len as usize / 2) };
    audio_callback.lock().update(stream);
}

pub fn start_audio(
    sample_rate: u32,
    num_channels: u8,
    num_samples: u16,
    audio_callback: Arc<Mutex<dyn AudioCallback>>,
) {
    let userdata = Box::into_raw(Box::new(audio_callback)) as *mut _;
    let desired = SDL_AudioSpec {
        freq: sample_rate as i32,
        format: AUDIO_S16 as u16,
        channels: num_channels,
        silence: 0,
        samples: num_samples,
        padding: 0,
        size: 0,
        callback: Some(c_audio_callback),
        userdata,
    };
    let mut obtained = SDL_AudioSpec {
        freq: 0,
        format: 0,
        channels: 0,
        silence: 0,
        samples: 0,
        padding: 0,
        size: 0,
        callback: None,
        userdata: null_mut(),
    };
    platform().audio_device_id =
        unsafe { SDL_OpenAudioDevice(null_mut(), 0, &desired, &mut obtained, 0) };
    set_audio_enabled(true);
}

pub fn set_audio_enabled(enabled: bool) {
    let pause_on = if enabled { 0 } else { 1 };
    let audio_device_id = platform().audio_device_id;
    if audio_device_id != 0 {
        unsafe {
            SDL_PauseAudioDevice(audio_device_id, pause_on);
        }
    }
}
