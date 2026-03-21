use std::path::Path;
use std::{env, fs};

fn find_pyxel_python_dir() -> Option<String> {
    // Look for python/pyxel/ relative to the binary
    let exe = env::current_exe().ok()?;
    let mut dir = exe.parent()?;
    // Walk up from binary location to find repo root with python/pyxel/
    for _ in 0..10 {
        let candidate = dir.join("python").join("pyxel");
        if candidate.join("cli.py").exists() {
            return Some(candidate.to_string_lossy().into_owned());
        }
        dir = dir.parent()?;
    }
    None
}

fn main() {
    let interp = pyxel_embed::create_interpreter();
    let args: Vec<String> = env::args().collect();

    // Set pyxel.__path__ so "import pyxel.cli" works, and patch runpy workaround
    if let Some(pyxel_dir) = find_pyxel_python_dir() {
        let setup = format!(
            r#"
import pyxel
pyxel.__path__ = [r"{}"]

# Patch play_pyxel_app to use exec instead of runpy (RustPython compatibility)
def _patch_cli():
    try:
        import pyxel.cli as cli
        import os, sys
        _orig_play = cli.play_pyxel_app
        def _play_pyxel_app(pyxel_app_file):
            file_ext = os.path.splitext(pyxel_app_file)[1].lower()
            if file_ext != ".zip":
                pyxel_app_file = cli._complete_extension(pyxel_app_file, "play", pyxel.APP_FILE_EXTENSION)
            cli._check_file_exists(pyxel_app_file)
            cli.print_pyxel_app_metadata(pyxel_app_file)
            startup_script_file = cli._extract_pyxel_app(pyxel_app_file)
            if startup_script_file:
                sys.path.insert(0, os.path.abspath(os.path.dirname(startup_script_file)))
                os.chdir(os.path.dirname(startup_script_file))
                with open(startup_script_file) as f:
                    code = f.read()
                exec(compile(code, startup_script_file, "exec"), {{"__name__": "__main__", "__file__": startup_script_file}})
                return
            print(f"file not found: '{{pyxel.APP_STARTUP_SCRIPT_FILE}}'")
            sys.exit(1)
        cli.play_pyxel_app = _play_pyxel_app
    except Exception:
        pass
_patch_cli()
del _patch_cli
"#,
            pyxel_dir.replace('\\', "\\\\")
        );
        pyxel_embed::exec_source(&interp, &setup);
    }

    if args.len() < 2 {
        pyxel_embed::exec_source(&interp, "import pyxel; pyxel.init(160, 120, 'Test'); pyxel.cls(1); pyxel.text(10, 10, 'Hello!', 7)");
        pyxel_embed::exec_source(&interp, "import pyxel; pyxel.show()");
    } else {
        let script_path = Path::new(&args[1]);
        let script = fs::read_to_string(script_path).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {e}", args[1]);
            std::process::exit(1);
        });

        // Change to script directory for relative resource paths
        if let Some(dir) = script_path
            .canonicalize()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        {
            let _ = env::set_current_dir(&dir);
        }

        let abs_path = script_path
            .canonicalize()
            .unwrap_or_else(|_| script_path.to_path_buf());
        pyxel_embed::exec_source_with_file(
            &interp,
            &script,
            Some(abs_path.to_str().unwrap_or("<script>")),
        );
    }
}
