use std::ffi::CString;
use std::sync::{Mutex, MutexGuard};

use crate::{ffi, module};

static RUNTIME_LOCK: Mutex<()> = Mutex::new(());

pub struct Runtime {
    _guard: MutexGuard<'static, ()>,
}

impl Runtime {
    pub fn new() -> Self {
        let guard = RUNTIME_LOCK.lock().expect("PocketPy runtime lock poisoned");
        unsafe {
            ffi::py_initialize();
        }
        module::register();
        Self { _guard: guard }
    }

    pub fn exec_source(&self, source: &str, filename: &str) -> Result<(), String> {
        let source = CString::new(source).map_err(|_| "source contains NUL byte".to_owned())?;
        let filename =
            CString::new(filename).map_err(|_| "filename contains NUL byte".to_owned())?;
        let ok = unsafe {
            ffi::py_exec(
                source.as_ptr(),
                filename.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                std::ptr::null_mut(),
            )
        };
        if ok {
            Ok(())
        } else {
            unsafe {
                ffi::py_printexc();
            }
            Err(format!("PocketPy failed to execute {filename:?}"))
        }
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Runtime {
    fn drop(&mut self) {
        unsafe {
            ffi::py_finalize();
        }
    }
}
