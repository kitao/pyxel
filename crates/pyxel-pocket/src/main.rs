use std::path::Path;
use std::{env, process};

fn print_usage() {
    println!("usage:");
    println!("    pyxel-pocket PYTHON_SCRIPT_FILE(.py)");
    println!("    pyxel-pocket PYXEL_APP_FILE(.pyxapp)");
}

fn print_help() {
    println!("pyxel-pocket {}, a standalone Pyxel Player", pyxel::VERSION);
    print_usage();
}

fn main() {
    let mut args = env::args_os();
    let _program = args.next();
    let Some(script) = args.next() else {
        print_help();
        return;
    };
    if args.next().is_some() {
        println!("invalid number of parameters");
        print_usage();
        process::exit(1);
    }

    if let Err(err) = pyxel_pocket::run_path(Path::new(&script)) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
