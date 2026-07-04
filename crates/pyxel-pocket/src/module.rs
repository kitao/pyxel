use crate::{ffi, wrappers};

pub fn register() {
    unsafe {
        let module = ffi::py_newmodule(c"pyxel".as_ptr());
        wrappers::variables::add_constants(module);
        wrappers::system::add_functions(module);
        wrappers::variables::sync(module);
    }
}

pub fn sync_variables() {
    unsafe {
        let module = ffi::py_getmodule(c"pyxel".as_ptr());
        wrappers::variables::sync(module);
    }
}
