use std::mem::MaybeUninit;
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
    let audio_callback = unsafe { &*userdata.cast::<Arc<Mutex<dyn AudioCallback>>>() };
    let stream: &mut [i16] =
        unsafe { slice::from_raw_parts_mut(stream.cast::<i16>(), len as usize / 2) };
    audio_callback.lock().update(stream);
}

pub fn start_audio(
    sample_rate: u32,
    num_channels: u8,
    num_samples: u16,
    audio_callback: Arc<Mutex<dyn AudioCallback>>,
) {
    let userdata = Box::into_raw(Box::new(audio_callback)).cast();
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
    let mut obtained = MaybeUninit::uninit();
    platform().audio_device_id =
        unsafe { SDL_OpenAudioDevice(null_mut(), 0, &desired, obtained.as_mut_ptr(), 0) };
    if platform().audio_device_id == 0 {
        println!("PyxelWarning: Failed to initialize audio device");
    }
    set_audio_enabled(true);
}

pub fn set_audio_enabled(enabled: bool) {
    let pause_on = i32::from(!enabled);
    let audio_device_id = platform().audio_device_id;
    if audio_device_id != 0 {
        unsafe {
            SDL_PauseAudioDevice(audio_device_id, pause_on);
        }
    }
}
