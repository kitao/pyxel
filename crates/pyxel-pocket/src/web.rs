use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Mutex;

use crate::Runtime;

static LAST_ERROR: Mutex<Option<CString>> = Mutex::new(None);

#[no_mangle]
pub unsafe extern "C" fn pyxel_pocket_run_script(
    source: *const c_char,
    filename: *const c_char,
) -> c_int {
    clear_last_error();

    let result = catch_unwind(AssertUnwindSafe(|| {
        let source = unsafe { read_required_c_str(source, "source") }?;
        let filename = unsafe { read_optional_c_str(filename, "<pyxel-pocket-web>") }?;
        // pyxel.run() unwinds into Emscripten after installing the browser main
        // loop, so the PocketPy runtime must live for the rest of the page.
        let runtime = Box::leak(Box::new(Runtime::new()));
        runtime.exec_source(source, filename)
    }));

    match result {
        Ok(Ok(())) => 0,
        Ok(Err(err)) => {
            set_last_error(err);
            1
        }
        Err(_) => {
            set_last_error("PocketPy runtime panicked");
            2
        }
    }
}

#[no_mangle]
pub extern "C" fn pyxel_pocket_last_error() -> *const c_char {
    LAST_ERROR
        .lock()
        .expect("PocketPy Web error lock poisoned")
        .as_ref()
        .map_or(std::ptr::null(), |message| message.as_ptr())
}

unsafe fn read_required_c_str<'a>(ptr: *const c_char, name: &str) -> Result<&'a str, String> {
    if ptr.is_null() {
        return Err(format!("{name} pointer is null"));
    }

    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| format!("{name} is not valid UTF-8"))
}

unsafe fn read_optional_c_str<'a>(
    ptr: *const c_char,
    fallback: &'a str,
) -> Result<&'a str, String> {
    if ptr.is_null() {
        return Ok(fallback);
    }

    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| "filename is not valid UTF-8".to_owned())
}

fn clear_last_error() {
    *LAST_ERROR.lock().expect("PocketPy Web error lock poisoned") = None;
}

fn set_last_error(message: impl Into<String>) {
    let message = CString::new(message.into()).unwrap_or_else(|_| {
        CString::new("PocketPy error message contains NUL byte").expect("static string is valid")
    });
    *LAST_ERROR.lock().expect("PocketPy Web error lock poisoned") = Some(message);
}

#[cfg(test)]
mod tests {
    use std::os::raw::{c_char, c_int};

    use super::{pyxel_pocket_last_error, pyxel_pocket_run_script};

    #[test]
    fn exports_use_c_abi() {
        let _run: unsafe extern "C" fn(*const c_char, *const c_char) -> c_int =
            pyxel_pocket_run_script;
        let _last_error: extern "C" fn() -> *const c_char = pyxel_pocket_last_error;
    }
}
