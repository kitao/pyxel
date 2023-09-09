use std::ptr::null_mut;

use crate::platform::platform;
use crate::sdl2_sys::*;

pub fn init_audio(freqency: u32, channels: u8, samples: u16) -> SDL_AudioDeviceID {
    let desired = SDL_AudioSpec {
        freq: freqency as i32,
        format: AUDIO_S16 as u16,
        channels: channels,
        silence: 0,
        samples: samples,
        padding: 0,
        size: 0,
        callback: None,
        userdata: null_mut(),
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
    //unsafe { SDL_OpenAudioDevice(device, iscapture, desired, obtained, allowed_changes) }
    0
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

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

/*
use sdl2_sys::*;
use std::os::raw::c_void;
use std::ptr::null_mut;

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

struct AudioManager<'a> {
    callback: Box<dyn AudioCallback + 'a>,
}

extern "C" fn bridge_callback(userdata: *mut c_void, stream: *mut u8, len: i32) {
    let callback: &mut Box<dyn AudioCallback> = unsafe { &mut *(userdata as *mut _) };
    let slice = unsafe { std::slice::from_raw_parts_mut(stream as *mut i16, (len / 2) as usize) };
    callback.update(slice);
}

impl<'a> AudioManager<'a> {
    fn new<T: AudioCallback + 'a>(callback: T) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }

    fn start(&mut self) {
        unsafe {
            let desired_spec = SDL_AudioSpec {
                freq: 44100,
                format: AUDIO_S16LSB as u16,
                channels: 2,
                silence: 0,
                samples: 512,
                size: 0,
                callback: Some(bridge_callback),
                userdata: &mut *self.callback as *mut _ as *mut c_void,
            };

            let device_id = SDL_OpenAudioDevice(null_mut(), 0, &desired_spec, null_mut(), 0);
            if device_id == 0 {
                println!("SDL_OpenAudioDevice Error: {}", std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
                return;
            }

            SDL_PauseAudioDevice(device_id, 0);


            SDL_CloseAudioDevice(device_id);
        }
    }
}

// AudioCallback traitの実装例
struct MyAudioCallback;

impl AudioCallback for MyAudioCallback {
    fn update(&mut self, out: &mut [i16]) {
        for sample in out.iter_mut() {
            *sample = 0;  // この例では、静音を出力するだけです
        }
    }
}

fn main() {
    unsafe {
        if SDL_Init(SDL_INIT_AUDIO) != 0 {
            println!("SDL_Init Error: {}", std::ffi::CStr::from_ptr(SDL_GetError()).to_string_lossy());
            return;
        }

        let mut manager = AudioManager::new(MyAudioCallback);
        manager.start();

        SDL_Quit();
    }
}

*/

/*
use sdl2::audio::{
    AudioCallback as SdlAudioCallback, AudioDevice as SdlAudioDevice,
    AudioSpecDesired as SdlAudioSpecDesired,
};

struct AudioContextHolder {
    audio: shared_type!(dyn AudioCallback + Send),
}

impl SdlAudioCallback for AudioContextHolder {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        self.audio.lock().update(out);
    }
}

impl Platform {
    pub fn start_audio(
        &mut self,
        sample_rate: u32,
        num_samples: u32,
        audio: shared_type!(dyn AudioCallback + Send),
    ) {
        let spec = SdlAudioSpecDesired {
            freq: Some(sample_rate as i32),
            channels: Some(1),
            samples: Some(num_samples as u16),
        };
        self.sdl_audio_device = self.sdl_audio.as_ref().and_then(|sdl_audio| {
            sdl_audio
                .open_playback(None, &spec, |_| AudioContextHolder { audio })
                .map_or_else(
                    |_| {
                        println!("Unable to open a new audio device");
                        None
                    },
                    |sdl_audio_device| {
                        sdl_audio_device.resume();
                        Some(sdl_audio_device)
                    },
                )
        });
    }
*/
