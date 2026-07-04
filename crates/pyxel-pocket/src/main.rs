use std::path::Path;
use std::{env, process};

fn print_usage() {
    eprintln!("Usage: pyxel-pocket SCRIPT.py|APP.pyxapp");
}

fn main() {
    let mut args = env::args_os();
    let _program = args.next();
    let Some(script) = args.next() else {
        print_usage();
        process::exit(1);
    };
    if args.next().is_some() {
        print_usage();
        process::exit(1);
    }

    if let Err(err) = pyxel_pocket::run_path(Path::new(&script)) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
