use std::ffi::CString;
use std::fs::File;
use std::path::Path;

use crate::{ffi, runner, value};

fn pyxel_app_metadata(filename: &str) -> Result<Vec<(String, String)>, String> {
    let file = File::open(filename).map_err(|_| format!("no such file: '{filename}'"))?;
    let archive =
        zip::ZipArchive::new(file).map_err(|_| format!("failed to parse file: '{filename}'"))?;
    let comment = String::from_utf8_lossy(archive.comment());
    let mut metadata = Vec::new();

    for line in comment.lines() {
        if line.starts_with('-') {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            metadata.push((key.trim().to_owned(), value.trim().to_owned()));
        }
    }

    Ok(metadata)
}

unsafe extern "C" fn get_pyxel_app_metadata(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = value::str_arg(argv, 0);
    let metadata = match pyxel_app_metadata(&filename) {
        Ok(metadata) => metadata,
        Err(err) => return value::raise_exception(&err),
    };

    let dict = ffi::py_pushtmp();
    ffi::py_newdict(dict);
    for (key, value) in metadata {
        let key_ref = ffi::py_pushtmp();
        let key = CString::new(key).unwrap();
        ffi::py_newstr(key_ref, key.as_ptr());
        let value_ref = ffi::py_pushtmp();
        let value = CString::new(value).unwrap();
        ffi::py_newstr(value_ref, value.as_ptr());
        if !ffi::py_dict_setitem(dict, key_ref, value_ref) {
            ffi::py_pop();
            ffi::py_pop();
            ffi::py_pop();
            return false;
        }
        ffi::py_pop();
        ffi::py_pop();
    }
    ffi::py_assign(ffi::py_retval(), dict);
    ffi::py_pop();
    true
}

unsafe extern "C" fn play_pyxel_app(_argc: i32, argv: ffi::py_StackRef) -> bool {
    let filename = value::str_arg(argv, 0);
    match runner::play_pyxapp_in_current_runtime(Path::new(&filename)) {
        Ok(()) => {
            value::return_none();
            true
        }
        Err(err) => value::raise_exception(&err),
    }
}

pub unsafe fn register(parent_module: ffi::py_GlobalRef) {
    let module = ffi::py_newmodule(c"pyxel.cli".as_ptr());
    ffi::py_bind(
        module,
        c"get_pyxel_app_metadata(pyxel_app_file)".as_ptr(),
        Some(get_pyxel_app_metadata),
    );
    ffi::py_bind(
        module,
        c"play_pyxel_app(pyxel_app_file)".as_ptr(),
        Some(play_pyxel_app),
    );
    ffi::py_setattr(parent_module, ffi::py_name(c"cli".as_ptr()), module);
}
