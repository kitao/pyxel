mod ffi;
#[macro_use]
mod helpers;
mod audio_wrapper;
mod channel_wrapper;
mod constant_wrapper;
mod font_wrapper;
mod graphics_wrapper;
mod image_wrapper;
mod input_wrapper;
mod math_wrapper;
mod music_wrapper;
mod resource_wrapper;
mod sound_wrapper;
mod system_wrapper;
mod tilemap_wrapper;
mod tone_wrapper;
mod variable_wrapper;

use std::ffi::CString;

fn register_pyxel_module() {
    unsafe {
        let m = ffi::py_newmodule(c"pyxel".as_ptr());

        // Register types (must be before functions that reference them)
        font_wrapper::add_font_class(m);
        image_wrapper::add_image_class(m);
        tilemap_wrapper::add_tilemap_class(m);
        channel_wrapper::add_channel_class(m);
        tone_wrapper::add_tone_class(m);
        sound_wrapper::add_sound_class(m);
        music_wrapper::add_music_class(m);

        // Register module variables and constants
        constant_wrapper::add_module_constants(m);
        variable_wrapper::add_module_variables(m);

        // Register module functions
        system_wrapper::add_system_functions(m);
        resource_wrapper::add_resource_functions(m);
        input_wrapper::add_input_functions(m);
        graphics_wrapper::add_graphics_functions(m);
        audio_wrapper::add_audio_functions(m);
        math_wrapper::add_math_functions(m);
    }
}

/// Initialize PocketPy VM and register the `pyxel` module.
pub fn initialize() {
    unsafe {
        ffi::py_initialize();
    }
    register_pyxel_module();
}

/// Finalize PocketPy VM.
pub fn finalize() {
    unsafe {
        ffi::py_finalize();
    }
}

/// Execute a Python source string.
pub fn exec(source: &str, filename: &str) -> bool {
    let source = CString::new(source).unwrap();
    let filename = CString::new(filename).unwrap();
    unsafe {
        let ok = ffi::py_exec(
            source.as_ptr(),
            filename.as_ptr(),
            ffi::py_CompileMode_EXEC_MODE,
            std::ptr::null_mut(),
        );
        if !ok {
            ffi::py_printexc();
        }
        ok
    }
}
