use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

#[allow(non_camel_case_types)]
type em_arg_callback_func = unsafe extern "C" fn(*mut c_void);

extern "C" {
    fn emscripten_set_main_loop_arg(
        func: em_arg_callback_func,
        arg: *mut c_void,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    fn emscripten_force_exit(status: c_int);
    fn emscripten_run_script(script: *const c_char);
    fn emscripten_run_script_int(script: *const c_char) -> c_int;
    fn emscripten_run_script_string(script: *const c_char) -> *const c_char;
}

unsafe extern "C" fn callback_wrapper<F: FnMut()>(arg: *mut c_void) {
    (*arg.cast::<F>())();
}

pub(crate) fn run<F: FnMut()>(callback: F) {
    unsafe {
        emscripten_set_main_loop_arg(
            callback_wrapper::<F>,
            Box::into_raw(Box::new(callback)).cast::<std::ffi::c_void>(),
            0,
            1,
        );
    }
}

pub(crate) fn exit(status: i32) {
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

pub fn save_file(filename: &str) {
    run_script(&format!("_savePyxelFile('{filename}');"));
}

pub fn datetime_string() -> String {
    let script = "
        let now = new Date();
        let year = now.getFullYear();
        let month = now.getMonth() + 1;
        let date = now.getDate();
        let hour = now.getHours();
        let min = now.getMinutes();
        let sec = now.getSeconds();
        `${year}${month}${date}-${hour}${min}${sec}`
    ";
    run_script_string(script)
}
