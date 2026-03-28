use std::{env, fs, path::Path, process};

fn print_usage() {
    eprintln!("Pyxel Pocket - A portable Pyxel runtime");
    eprintln!();
    eprintln!("Usage: pyxel-pocket <file>");
    eprintln!();
    eprintln!("  .py      Run a Python script");
    eprintln!("  .pyxapp  Play a Pyxel app");
    eprintln!("  .pyxres  Edit a Pyxel resource");
}

fn run_script(path: &Path) {
    let source = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error: cannot read '{}': {e}", path.display());
        process::exit(1);
    });

    if let Some(dir) = path.parent() {
        let _ = env::set_current_dir(dir);
    }

    // Set __file__ global
    let file_path = path.to_string_lossy();
    let setup = format!("__file__ = '{file_path}'");
    pyxel_pocket::exec(&setup, "<setup>");

    let filename = path
        .file_name()
        .map(|f| f.to_string_lossy().into_owned())
        .unwrap_or_default();
    if !pyxel_pocket::exec(&source, &filename) {
        process::exit(1);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let file_arg = &args[1];
    let path = Path::new(file_arg);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    pyxel_pocket::initialize();

    match ext.as_str() {
        "py" => {
            let path = fs::canonicalize(path).unwrap_or_else(|e| {
                eprintln!("Error: cannot resolve '{}': {e}", file_arg);
                process::exit(1);
            });
            run_script(&path);
        }
        "pyxapp" | "zip" => {
            let path = fs::canonicalize(path).unwrap_or_else(|e| {
                eprintln!("Error: cannot resolve '{}': {e}", file_arg);
                process::exit(1);
            });
            pyxel_pocket::play_app(&path);
        }
        "pyxres" => {
            // .pyxres need not exist yet (new file)
            let path = if path.exists() {
                fs::canonicalize(path).unwrap_or_else(|e| {
                    eprintln!("Error: cannot resolve '{}': {e}", file_arg);
                    process::exit(1);
                })
            } else {
                env::current_dir().unwrap().join(file_arg)
            };
            pyxel_pocket::edit_resource(&path);
        }
        _ => {
            eprintln!("Error: unsupported file type '.{ext}'");
            print_usage();
            process::exit(1);
        }
    }

    pyxel_pocket::finalize();
}
