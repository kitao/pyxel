use crate::{ffi, wrappers};

pub fn register() {
    unsafe {
        let module = ffi::py_newmodule(c"pyxel".as_ptr());
        wrappers::variables::add_constants(module);
        wrappers::objects::register(module);
        wrappers::system::add_functions(module);
        wrappers::resource::add_functions(module);
        wrappers::graphics::add_functions(module);
        wrappers::audio::add_functions(module);
        wrappers::input::add_functions(module);
        wrappers::math::add_functions(module);
        wrappers::variables::sync(module);
    }
}

pub fn sync_variables() {
    unsafe {
        let module = ffi::py_getmodule(c"pyxel".as_ptr());
        wrappers::variables::sync(module);
    }
}
