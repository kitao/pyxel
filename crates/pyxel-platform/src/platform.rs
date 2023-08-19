use std::process::exit;

use crate::sdl2_sys::*;
use crate::window::init_window;

pub fn init<'a, F: FnOnce() -> (&'a str, u32, u32)>(window_params: F) {
    if unsafe { SDL_Init(SDL_INIT_VIDEO) } < 0 {
        panic!("Failed to initialize SDL2");
    }
    let (title, width, height) = window_params();
    init_window(title, width, height);
}

pub fn run<F: FnMut()>(mut main_loop: F) {
    #[cfg(not(target_os = "emscripten"))]
    loop {
        let start_ms = elapsed_time() as f64;
        main_loop();
        let elapsed_ms = elapsed_time() as f64 - start_ms;
        let wait_ms = 1000.0 / 60.0 - elapsed_ms;
        if wait_ms > 0.0 {
            sleep((wait_ms / 2.0) as u32);
        }
    }

    #[cfg(target_os = "emscripten")]
    emscripten::set_main_loop(main_loop);
}

pub fn quit() {
    unsafe {
        SDL_Quit();
    }

    #[cfg(not(target_os = "emscripten"))]
    exit(0);

    #[cfg(target_os = "emscripten")]
    emscripten::force_exit(0);
}

pub fn elapsed_time() -> u32 {
    unsafe { SDL_GetTicks() }
}

pub fn sleep(ms: u32) {
    unsafe {
        SDL_Delay(ms);
    }
}

pub fn set_audio_enabled(enabled: bool) {
    if enabled {
        /*
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.resume();
        }
        */
    } else {
        /*
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.pause();
        }
        */
    }
}

pub fn set_mouse_visible(visible: bool) {
    let visible = if visible { SDL_ENABLE } else { SDL_DISABLE };
    unsafe {
        SDL_ShowCursor(visible as i32);
    }
}

pub fn set_mouse_pos(x: i32, y: i32) {
    unsafe {
        SDL_WarpMouseGlobal(x, y);
    }
    /*let (window_x, window_y) = self.sdl_canvas.window().position();
    let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();
    let mouse_x = x * screen_scale as i32 + window_x + screen_x as i32;
    let mouse_y = y * screen_scale as i32 + window_y + screen_y as i32;*/
}

/*
use std::env::var as envvar;
use std::fs::{read_to_string, write};

#[cfg(not(target_os = "emscripten"))]
use chrono::Local;
use sdl2::audio::{
    AudioCallback as SdlAudioCallback, AudioDevice as SdlAudioDevice,
    AudioSpecDesired as SdlAudioSpecDesired,
};
use sdl2::controller::{
    Axis as SdlAxis, Button as SdlButton, GameController as SdlGameControllerState,
};
use crate::settings::WATCH_INFO_FILE_ENVVAR;

#[derive(Default, PartialEq)]
struct WindowState {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

struct AudioContextHolder {
    audio: shared_type!(dyn AudioCallback + Send),
}

impl SdlAudioCallback for AudioContextHolder {
    type Channel = i16;

    fn callback(&mut self, out: &mut [i16]) {
        self.audio.lock().update(out);
    }
}

pub struct Platform {
    sdl_texture: SdlTexture,
    sdl_game_controller: Option<SdlGameController>,
    sdl_game_controller_states: Vec<SdlGameControllerState>,
    sdl_audio: Option<SdlAudio>,
    sdl_audio_device: Option<SdlAudioDevice<AudioContextHolder>>,
    mouse_x: i32,
    mouse_y: i32,
    watch_info_file: Option<String>,
    window_state: WindowState,
    #[cfg(target_os = "emscripten")]
    virtual_gamepad_states: [bool; 8],
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

    pub fn pause_audio(&mut self) {
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.pause();
        }
    }

    pub fn resume_audio(&mut self) {
        if let Some(audio_device) = &self.sdl_audio_device {
            audio_device.resume();
        }
    }

    #[cfg(target_os = "emscripten")]
    pub fn save_file_on_web_browser(filename: &str) {
        emscripten::run_script(&format!("_savePyxelFile('{filename}');"));
    }

    fn screen_pos_scale(&self) -> (u32, u32, u32) {
        let (window_width, window_height) = self.sdl_canvas.window().size();
        let screen_scale = min(
            window_width / self.screen_width,
            window_height / self.screen_height,
        );
        let screen_x = (window_width - self.screen_width * screen_scale) / 2;
        let screen_y = (window_height - self.screen_height * screen_scale) / 2;
        (screen_x, screen_y, screen_scale)
    }

    fn mouse_pos(&self) -> (i32, i32) {
        #[cfg(not(target_os = "emscripten"))]
        let (window_x, window_y) = self.sdl_canvas.window().position();
        #[cfg(target_os = "emscripten")]
        let (window_x, window_y) = (0, 0);
        let (screen_x, screen_y, screen_scale) = self.screen_pos_scale();
        let mut mouse_x = 0;
        let mut mouse_y = 0;
        unsafe {
            sdl2::sys::SDL_GetGlobalMouseState(&mut mouse_x, &mut mouse_y);
        }
        mouse_x = (mouse_x - window_x - screen_x as i32) / screen_scale as i32;
        mouse_y = (mouse_y - window_y - screen_y as i32) / screen_scale as i32;
        (mouse_x, mouse_y)
    }

    fn gamepad_index(&self, game_controller_id: u32) -> u32 {
        self.sdl_game_controller_states
            .iter()
            .position(|gc| gc.instance_id() == game_controller_id)
            .unwrap() as u32
    }

    fn watch_info_file() -> Option<String> {
        envvar(WATCH_INFO_FILE_ENVVAR).map_or(None, |watch_info_file| Some(watch_info_file))
    }

    fn load_watch_info(watch_info_file: &Option<String>) -> Option<WindowState> {
        if watch_info_file.is_none() {
            return None;
        }
        let watch_info_file = watch_info_file.as_ref().unwrap();
        let watch_info = read_to_string(watch_info_file).unwrap();
        let watch_info: Vec<&str> = watch_info.split(' ').collect();
        if watch_info.len() == 4 {
            Some(WindowState {
                x: watch_info[0].parse().unwrap(),
                y: watch_info[1].parse().unwrap(),
                width: watch_info[2].parse().unwrap(),
                height: watch_info[3].parse().unwrap(),
            })
        } else {
            None
        }
    }

    fn save_watch_info(&mut self) {
        if self.watch_info_file.is_none() || self.is_fullscreen() {
            return;
        }
        let (x, y) = self.sdl_canvas.window().position();
        let (width, height) = self.sdl_canvas.window().size();
        let window_state = WindowState {
            x,
            y,
            width,
            height,
        };
        if window_state == self.window_state {
            return;
        }
        self.window_state = window_state;
        let watch_info = format!("{x} {y} {width} {height}");
        write(self.watch_info_file.as_ref().unwrap(), watch_info).unwrap();
    }
}
*/

#[cfg(target_os = "emscripten")]
mod emscripten {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_void};

    #[allow(non_camel_case_types)]
    type em_arg_callback_func = unsafe extern "C" fn(*mut c_void);

    extern "C" {
        pub fn emscripten_set_main_loop_arg(
            func: em_arg_callback_func,
            arg: *mut c_void,
            fps: c_int,
            simulate_infinite_loop: c_int,
        );
        pub fn emscripten_force_exit(status: c_int);
        pub fn emscripten_run_script(script: *const c_char);
        pub fn emscripten_run_script_int(script: *const c_char) -> c_int;
        pub fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
    }

    unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
        (*arg.cast::<F>())();
    }

    pub fn set_main_loop<F: FnMut()>(callback: F) {
        unsafe {
            emscripten_set_main_loop_arg(
                callback_wrapper::<F>,
                Box::into_raw(Box::new(callback)).cast::<std::ffi::c_void>(),
                0,
                1,
            );
        }
    }

    pub fn force_exit(status: i32) {
        unsafe {
            emscripten_force_exit(status);
        }
    }

    pub fn run_script(script: &str) {
        let script = CString::new(script).unwrap();
        unsafe {
            emscripten_run_script(script.as_ptr());
        }
    }

    pub fn run_script_int(script: &str) -> i32 {
        let script = CString::new(script).unwrap();
        unsafe { emscripten_run_script_int(script.as_ptr()) }
    }

    pub fn run_script_string(script: &str) -> String {
        let script = CString::new(script).unwrap();
        unsafe {
            CStr::from_ptr(emscripten_run_script_string(script.as_ptr()))
                .to_str()
                .unwrap()
                .to_string()
        }
    }
}
