use cpython::Python;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;
use tempfile::tempdir;

fn main() {
    let (pyxel_ver, app_file_ext, res_file_ext) = pyxel_settings();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        print_usage(&pyxel_ver);
        return;
    }

    let filename = &args[1];
    let file_ext: &str = &file_extension(filename);

    if file_ext == "py" {
        execute_python_file(filename);
    } else if file_ext == app_file_ext {
        execute_pyxapp_file(filename);
    } else if file_ext == res_file_ext {
        edit_pyxres_file(filename);
    } else if file_ext.is_empty() {
        make_pyxapp_file(filename);
    } else {
        print_error("invalid file type");
    }
}

fn init_import_path(py: Python) {
    let args: Vec<String> = env::args().collect();
    let code = format!(
        r#"
def init_import_path():
    import os
    import sys

    dirname = os.path.dirname("{}")

    sys.path.insert(1, os.path.join(dirname, "../../.."))
    sys.path.insert(1, os.path.join(dirname, "../.."))
    sys.path.insert(1, os.path.join(dirname, ".."))
    sys.path.insert(1, os.path.join(dirname, "."))

init_import_path()
del init_import_path
        "#,
        args[0]
    );

    py.run(&code, None, None).unwrap();
}

fn pyxel_settings() -> (String, String, String) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    init_import_path(py);
    py.run("from pyxel import *", None, None).unwrap();
    py.eval(
        "(PYXEL_VERSION, APPLICATION_FILE_EXTENSION, RESOURCE_FILE_EXTENSION)",
        None,
        None,
    )
    .unwrap()
    .extract(py)
    .unwrap()
}

fn file_extension(filename: &str) -> String {
    Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_lowercase()
}

fn print_usage(version: &str) {
    println!("pyxel {}, a retro game engine for Python", version);
}

fn print_error(msg: &str) {
    println!("pyxel: {}", msg);
    exit(1);
}

fn execute_python_file(filename: &str) {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let code = format!("__file__ = '{}'; exec(open(__file__).read())", filename);

    init_import_path(py);
    py.run(&code, None, None).unwrap();
}

fn execute_pyxapp_file(filename: &str) {
    let gil = Python::acquire_gil();
    let python = gil.python();

    init_import_path(python);

    let dir = tempdir().unwrap();

    dir.close().unwrap();

    /*
    use std::io::{self, Write};

    let file_path = dir.path().join("my-temporary-note.txt");
    let mut file = File::create(file_path)?;
    writeln!(file, "Brian was here. Briefly.")?;

    drop(file);
    dir.close()?;
    */
}

fn edit_pyxres_file(filename: &str) {
    let gil = Python::acquire_gil();
    let py = gil.python();

    init_import_path(py);

    // TODO
}

fn make_pyxapp_file(dirname: &str) {
    let _ = dirname;

    // TODO
}
