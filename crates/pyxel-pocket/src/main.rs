use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: pyxel-pocket <script.py>");
        std::process::exit(1);
    }

    let script_path = fs::canonicalize(&args[1]).unwrap_or_else(|e| {
        eprintln!("Error: cannot resolve '{}': {e}", args[1]);
        std::process::exit(1);
    });

    let source = fs::read_to_string(&script_path).unwrap_or_else(|e| {
        eprintln!("Error: cannot read '{}': {e}", script_path.display());
        std::process::exit(1);
    });

    // Change to script directory
    if let Some(dir) = script_path.parent() {
        let _ = env::set_current_dir(dir);
    }

    pyxel_pocket::initialize();

    // Set __file__ global
    let file_path = script_path.to_string_lossy();
    let setup = format!("__file__ = '{file_path}'");
    pyxel_pocket::exec(&setup, "<setup>");

    let filename = script_path
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_else(|| args[1].clone());
    let ok = pyxel_pocket::exec(&source, &filename);
    pyxel_pocket::finalize();

    if !ok {
        std::process::exit(1);
    }
}
