use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

extern "C" {
    pub fn js_wait_next_vsync_bridge();
}

pub fn wait_next_vsync() {
    unsafe {
        js_wait_next_vsync_bridge();
    }
}

macro_rules! run_js {
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
            String::new()
        } else {
            unsafe {
                CStr::from_ptr(ptr).to_string_lossy().into_owned();
            }
        }
    }};
}

unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
    (*arg.cast::<F>())();
}

#[allow(non_camel_case_types)]
type em_arg_callback_func = unsafe extern "C" fn(*mut c_void);

pub(crate) fn run<F: FnMut()>(callback: F) {
    extern "C" {
        fn emscripten_set_main_loop_arg(
            func: em_arg_callback_func,
            arg: *mut c_void,
            fps: c_int,
            simulate_infinite_loop: c_int,
        );
    }
    unsafe {
        emscripten_set_main_loop_arg(
            callback_wrapper::<F>,
            Box::into_raw(Box::new(callback)).cast::<c_void>(),
            0,
            1,
        );
    }
}
