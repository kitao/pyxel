#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(
    clippy::cargo_common_metadata,
    clippy::missing_const_for_fn,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies
)]

mod interpreter;
mod utils;

use std::process::exit;

use tempfile::tempdir;

use crate::interpreter::Interpreter;
use crate::utils::{command_args, file_extension};

const PYXEL_IMPORT_PATHS: [&str; 4] = [".", "..", "../..", "../../.."];

fn main() {
    let interpreter = Interpreter::new();
    interpreter.add_import_paths(&PYXEL_IMPORT_PATHS);
    interpreter.import("pyxel");
    let (pyxel_ver, app_ext, res_ext) = interpreter.eval::<(String, String, String)>(
        "(pyxel.PYXEL_VERSION, pyxel.APPLICATION_FILE_EXTENSION, pyxel.RESOURCE_FILE_EXTENSION)",
    );
    let args = command_args();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        print_usage(&pyxel_ver);
        return;
    }
    if args[1] == "-e" || args[1] == "--examples" {
        copy_examples();
        return;
    }
    let file_ext: &str = &file_extension(&args[1]);
    if file_ext == "py" {
        run_script_file(&args[1]);
    } else if file_ext == app_ext {
        run_application_file(&args[1]);
    } else if file_ext == res_ext {
        edit_resource_file(&args[1]);
    } else if file_ext.is_empty() {
        make_application_file(&args[1]);
    } else {
        print_error("invalid file type");
    }
}

fn print_usage(version: &str) {
    println!("pyxel {}, a retro game engine for Python", version);
    /*
    if arg == "-v" or arg == "--version":
        print("Pyxel Editor {}".format(pyxel.VERSION))
    else:
        print("Usage: pyxeleditor [option] [pyxel_resource_file]")
        print("Options:")
        print(" -h, --help     This help text")
        print(" -v, --version  Show version number and quit")
    */
}

fn print_error(msg: &str) {
    println!("pyxel: {}", msg);
    exit(1);
}

fn run_script_file(filename: &str) {
    let interpreter = Interpreter::new();
    interpreter.add_import_paths(&PYXEL_IMPORT_PATHS);
    interpreter.run_file(filename);
}

fn run_application_file(filename: &str) {
    let interpreter = Interpreter::new();
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

fn edit_resource_file(filename: &str) {
    let interpreter = Interpreter::new();
    let code = format!("pyxel.editor.run({})", filename);
    interpreter.add_import_paths(&PYXEL_IMPORT_PATHS);
    interpreter.import("pyxel.editor");
    interpreter.run_code(&code);
}

fn make_application_file(dirname: &str) {
    let _ = dirname;
    // TODO
}

fn copy_examples() {
    //
}
