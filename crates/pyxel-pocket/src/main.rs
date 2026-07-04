use std::path::Path;
use std::{env, fs, process};

fn print_usage() {
    eprintln!("Usage: pyxel-pocket SCRIPT.py");
}

fn run_script(path: &Path) -> Result<(), String> {
    let path = fs::canonicalize(path)
        .map_err(|err| format!("cannot resolve '{}': {err}", path.display()))?;
    let source = fs::read_to_string(&path)
        .map_err(|err| format!("cannot read '{}': {err}", path.display()))?;

    if let Some(parent) = path.parent() {
        env::set_current_dir(parent)
            .map_err(|err| format!("cannot enter '{}': {err}", parent.display()))?;
    }

    let runtime = pyxel_pocket::Runtime::new();
    let file_path = path
        .to_string_lossy()
        .replace('\\', "\\\\")
        .replace('\'', "\\'");
    runtime.exec_source(&format!("__file__ = '{file_path}'"), "<setup>")?;
    runtime.exec_source(
        &source,
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("<script>"),
    )
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

    if let Err(err) = run_script(Path::new(&script)) {
        eprintln!("Error: {err}");
        process::exit(1);
    }
}
