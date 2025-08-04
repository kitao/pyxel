use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

macro_rules! em_js {
    ($script:expr) => {{
        let c_script = CString::new($script).unwrap();
        extern "C" {
            fn emscripten_run_script(script: *const c_char);
        }
        unsafe { emscripten_run_script(c_script.as_ptr()) }
    }};

    (int $script:expr) => {{
        let c_script = CString::new($script).unwrap();
        extern "C" {
            fn emscripten_run_script_int(script: *const c_char) -> c_int;
        }
        unsafe { emscripten_run_script_int(c_script.as_ptr()) }
    }};

    (string $script:expr) => {{
        extern "C" {
            fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
        }
        let c_script = CString::new($script).unwrap();
        let ptr = unsafe { emscripten_run_script_string(c_script.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() })
        }
    }};
}

unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
    (*arg.cast::<F>())();
}

pub struct PlatformWeb {
    // TODO
}

impl PlatformWeb {
    pub fn new() -> Self {
        Self {
            // TODO
        }
    }

    //
    // Core
    //
    pub fn init(&mut self) {
        //
    }

    pub fn quit(&mut self) {
        extern "C" {
            fn emscripten_force_exit(status: c_int);
        }
        unsafe {
            emscripten_force_exit(0);
        }
    }

    pub fn ticks(&self) -> u32 {
        extern "C" {
            fn emscripten_get_now() -> f64;
        }
        unsafe { (emscripten_get_now() * 1000.0) as u32 }
    }

    //
    // Window
    //
    pub fn init_window(&mut self, title: &str, width: u32, height: u32) {
        em_js!(
            r#"
                alert('!!!{}');
            "#
        );
        //
    }

    pub fn window_pos(&mut self) -> (i32, i32) {
        //
    }

    pub fn set_window_pos(&mut self, x: i32, y: i32) {
        //
    }

    pub fn window_size(&mut self) -> (u32, u32) {
        //
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        //
    }

    pub fn set_window_title(&mut self, title: &str) {
        //
    }

    pub fn set_window_icon(&mut self, width: u32, height: u32, rgba: &[u8]) {
        //
    }

    pub fn is_fullscreen(&mut self) -> bool {
        //
    }

    pub fn set_fullscreen(&mut self, enabled: bool) {
        //
    }

    pub fn set_mouse_pos(&mut self, x: i32, y: i32) {
        //
    }

    pub fn set_mouse_visible(&self, visible: bool) {
        //
    }

    pub fn display_size(&self) -> (u32, u32) {
        //
    }

    //
    // Audio
    //
    pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
        &mut self,
        sample_rate: u32,
        buffer_size: u32,
        callback: F,
    ) {
        //
    }

    pub fn pause_audio(&mut self, paused: bool) {
        //
    }

    //
    // Frame
    //
    pub fn run_frame_loop<F: FnMut(f32)>(&mut self, fps: u32, mut callback: F) {
        #[allow(non_camel_case_types)]
        type em_arg_callback_func = unsafe extern "C" fn(*mut c_void);

        extern "C" {
            fn emscripten_set_main_loop_arg(
                func: em_arg_callback_func,
                arg: *mut c_void,
                fps: c_int,
                simulate_infinite_loop: c_int,
            );
        }
        //
    }

    pub fn step_frame(&mut self, fps: u32) {
        //
    }

    pub fn poll_events(&mut self) -> Vec<Event> {
        //
    }

    pub fn gl_profile(&self) -> GLProfile {
        //
    }

    pub fn gl_context(&mut self) -> &'static mut Context {
        //
    }
}
