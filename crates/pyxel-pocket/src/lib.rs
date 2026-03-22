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
    register_compat_modules();
}

/// Register compatibility modules missing from PocketPy's standard library.
fn register_compat_modules() {
    // Register pathlib as a native module so `import pathlib` works
    unsafe {
        ffi::py_newmodule(c"pathlib".as_ptr());
    }
    exec(
        r#"
import os as _os

class Path:
    def __init__(self, *parts):
        self._path = "/".join(str(p) for p in parts)
    @property
    def parent(self):
        return Path(_os.path.dirname(self._path))
    @property
    def name(self):
        return _os.path.basename(self._path)
    @property
    def stem(self):
        n = self.name
        i = n.rfind(".")
        return n[:i] if i > 0 else n
    @property
    def suffix(self):
        n = self.name
        i = n.rfind(".")
        return n[i:] if i > 0 else ""
    def __truediv__(self, other):
        return Path(self._path + "/" + str(other))
    def __rtruediv__(self, other):
        return Path(str(other) + "/" + self._path)
    def resolve(self):
        return Path(_os.path.abspath(self._path))
    def exists(self):
        return _os.path.exists(self._path)
    def is_file(self):
        return _os.path.isfile(self._path)
    def is_dir(self):
        return _os.path.isdir(self._path)
    def iterdir(self):
        return [Path(self._path + "/" + n) for n in _os.listdir(self._path)]
    def glob(self, pattern):
        results = []
        for p in self.iterdir():
            if p.name.endswith(pattern.replace("*", "")):
                results.append(p)
        return results
    def __str__(self):
        return self._path
    def __repr__(self):
        return "Path('" + self._path + "')"
    def __eq__(self, other):
        return str(self) == str(other)
    def __ne__(self, other):
        return str(self) != str(other)
    def __hash__(self):
        return hash(self._path)

import pathlib as _pathlib
_pathlib.Path = Path
del _os, _pathlib, Path
"#,
        "<pathlib_compat>",
    );
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
