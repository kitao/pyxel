use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_void};
use std::ptr::null_mut;
use std::slice;

use parking_lot::Mutex;

use crate::sdl2::platform_sdl2::PlatformSdl2;
use crate::sdl2::sdl2_sys::*;

extern "C" fn c_audio_callback(userdata: *mut c_void, stream: *mut u8, len: c_int) {
    let callback = unsafe { &mut *(userdata as *mut Mutex<Box<dyn FnMut(&mut [i16]) + Send>>) };
    let stream: &mut [i16] =
        unsafe { slice::from_raw_parts_mut(stream.cast::<i16>(), len as usize / 2) };
    let mut guard = callback.lock();
    (*guard)(stream);
}

impl PlatformSdl2 {
    fn init_audio<F: FnMut(&mut [i16])>(
        &mut self,
        sample_rate: u32,
        buffer_size: u32,
        callback: Box<dyn FnMut(&mut [i16]) + Send>,
    ) {
        let userdata = Box::into_raw(Box::new(Mutex::new(callback))) as *mut c_void;
        let desired = SDL_AudioSpec {
            freq: sample_rate as i32,
            format: AUDIO_S16 as u16,
            channels: 1,
            silence: 0,
            samples: buffer_size as u16,
            padding: 0,
            size: 0,
            callback: Some(c_audio_callback),
            userdata,
        };

        let mut obtained = MaybeUninit::uninit();
        self.audio_device_id = unsafe {
            SDL_OpenAudioDevice(null_mut(), 0, &raw const desired, obtained.as_mut_ptr(), 0)
        };

        if self.audio_device_id == 0 {
            println!("Failed to initialize audio device");
        }

        self.pause_audio(false);
    }

    fn pause_audio(&mut self, paused: bool) {
        if self.audio_device_id != 0 {
            unsafe {
                SDL_PauseAudioDevice(self.audio_device_id, i32::from(paused));
            }
        }
    }
}
