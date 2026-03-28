mod editor_modules;
mod ffi;
#[macro_use]
mod helpers;
mod pyxapp;

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

/// Register os.path and os shims for PocketPy compatibility.
fn register_os_compat() {
    exec(
        r#"
import os as _os

def _join(*parts):
    return "/".join(str(p).rstrip("/") for p in parts if str(p))

def _dirname(p):
    p = str(p)
    parts = p.split("/")
    if len(parts) <= 1:
        return ""
    return "/".join(parts[:-1])

def _basename(p):
    p = str(p)
    parts = p.split("/")
    return parts[-1]

def _splitext(p):
    p = str(p)
    b = _basename(p)
    parts = b.split(".")
    if len(parts) > 1:
        ext = "." + parts[-1]
        return (p[:len(p)-len(ext)], ext)
    return (p, "")

def _isfile(p):
    try:
        with open(p, "r"):
            return True
    except:
        return False

def _isdir(p):
    try:
        _os.listdir(p)
        return True
    except:
        return False

def _abspath(p):
    p = str(p)
    if not p.startswith("/"):
        p = _os.getcwd() + "/" + p
    # Resolve . and ..
    parts = []
    for part in p.split("/"):
        if part == "..":
            if parts:
                parts.pop()
        elif part and part != ".":
            parts.append(part)
    return "/" + "/".join(parts)

def _makedirs(p, exist_ok=False):
    # PocketPy has no mkdir; this is a no-op shim for editor compat
    pass

_os.path.join = _join
_os.path.dirname = _dirname
_os.path.basename = _basename
_os.path.splitext = _splitext
_os.path.isfile = _isfile
_os.path.isdir = _isdir
_os.path.abspath = _abspath
_os.makedirs = _makedirs
if not hasattr(_os, 'listdir'):
    _os.listdir = lambda p: []

del _os, _join, _dirname, _basename, _splitext, _isfile, _isdir, _abspath, _makedirs
"#,
        "<os_compat>",
    );
}

/// Register sys shims for PocketPy compatibility.
fn register_sys_compat() {
    exec(
        r#"
import sys as _sys

class _SystemExit(Exception):
    pass

def _exit(code=0):
    raise _SystemExit(str(code))

_sys.exit = _exit
_sys.modules = {}
_sys.path = []

del _sys, _exit
"#,
        "<sys_compat>",
    );
}

// FFI for pk_newmodule (exported via patch 19)
extern "C" {
    fn pk_newmodule(
        path: *const core::ffi::c_char,
        is_init: bool,
    ) -> ffi::py_GlobalRef;
}

/// Register embedded editor modules so `import pyxel.editor.*` works.
/// Two-pass: register all modules first, then execute sources in their
/// own module scope (so relative imports resolve correctly).
fn register_editor_modules() {
    // Pass 1: register all modules (no source execution yet)
    for (mod_name, _source) in editor_modules::EDITOR_MODULES {
        let c_name = CString::new(*mod_name).unwrap();
        let is_init = *mod_name == "pyxel.editor" || *mod_name == "pyxel.editor.widgets";
        unsafe {
            pk_newmodule(c_name.as_ptr(), is_init);
        }
    }
    // Pass 2: execute module sources in their own module scope
    for (mod_name, source) in editor_modules::EDITOR_MODULES {
        let c_name = CString::new(*mod_name).unwrap();
        let c_source = CString::new(*source).unwrap();
        unsafe {
            let module = ffi::py_getmodule(c_name.as_ptr());
            // Set __file__ for os.path.dirname(__file__) usage
            let file_setup = CString::new(format!("__file__ = '{mod_name}'")).unwrap();
            let ok = ffi::py_exec(
                file_setup.as_ptr(),
                c_name.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                module,
            );
            if !ok {
                ffi::py_printexc();
            }
            // Execute module source in its own scope
            let ok = ffi::py_exec(
                c_source.as_ptr(),
                c_name.as_ptr(),
                ffi::py_CompileMode_EXEC_MODE,
                module,
            );
            if !ok {
                ffi::py_printexc();
            }
        }
    }
}

/// Register compatibility modules missing from PocketPy's standard library.
fn register_compat_modules() {
    register_os_compat();
    register_sys_compat();

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
        parts = n.split(".")
        return ".".join(parts[:-1]) if len(parts) > 1 else n
    @property
    def suffix(self):
        n = self.name
        parts = n.split(".")
        return "." + parts[-1] if len(parts) > 1 else ""
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

/// Play a .pyxapp archive.
pub fn play_app(path: &std::path::Path) {
    let (_temp_dir, script_path) = match pyxapp::extract_and_find_startup(path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    };

    let source = std::fs::read_to_string(&script_path).unwrap_or_else(|e| {
        eprintln!("Error: cannot read '{}': {e}", script_path.display());
        std::process::exit(1);
    });

    if let Some(dir) = script_path.parent() {
        let _ = std::env::set_current_dir(dir);
    }

    let file_path = script_path.to_string_lossy();
    let setup = format!("__file__ = '{file_path}'");
    exec(&setup, "<setup>");

    let filename = script_path
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ok = exec(&source, &filename);

    pyxapp::cleanup();

    if !ok {
        std::process::exit(1);
    }
}

/// Edit a .pyxres resource file.
pub fn edit_resource(path: &std::path::Path) {
    register_editor_modules();
    let file_str = path.to_string_lossy();
    let code = format!(
        "import pyxel.editor\npyxel.editor.App(\"{}\", \"image\")\n",
        file_str.replace('\\', "/").replace('"', "\\\"")
    );
    if !exec(&code, "<editor>") {
        std::process::exit(1);
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
