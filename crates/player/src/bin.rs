mod interpreter;
mod utils;

const PYXEL_VERSION: &str = "1.5.0";

use std::process::exit;
use tempfile::tempdir;

use crate::interpreter::Interpreter;
use crate::utils::{command_args, file_extension};

fn main() {
    let args = command_args();

    if args.len() != 2 {
        print_usage();
        return;
    }

    let filename = &args[1];
    let file_ext: &str = &file_extension(filename);

    if file_ext == "py" {
        execute_python_file(filename);
    } else if file_ext == "pyxapp" {
        execute_pyxapp_file(filename);
    } else if file_ext == "pyxres" {
        edit_pyxres_file(filename);
    } else if file_ext.is_empty() {
        make_pyxapp_file(filename);
    } else {
        print_error("invalid file type");
    }
}

fn print_usage() {
    println!("pyxel {}, a retro game engine for Python", PYXEL_VERSION);
}

fn print_error(msg: &str) {
    println!("pyxel: {}", msg);
    exit(1);
}

fn execute_python_file(filename: &str) {
    let interpreter = Interpreter::new();

    interpreter.run_file(filename);
}

fn execute_pyxapp_file(filename: &str) {
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

fn edit_pyxres_file(filename: &str) {
    let interpreter = Interpreter::new();

    // TODO
}

fn make_pyxapp_file(dirname: &str) {
    let _ = dirname;

    // TODO
}
