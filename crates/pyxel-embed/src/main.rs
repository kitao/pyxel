use std::env;

fn find_pyxel_python_dir() -> Option<String> {
    let exe = env::current_exe().ok()?;
    let mut dir = exe.parent()?;
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
    pyxel_embed::save_original_cwd();
    let interp = pyxel_embed::create_interpreter();
    let args: Vec<String> = env::args().collect();

    let pyxel_dir = find_pyxel_python_dir().unwrap_or_else(|| {
        eprintln!("Error: cannot find python/pyxel/ directory");
        std::process::exit(1);
    });

    // Build sys.argv as if invoked via `pyxel <subcommand> ...`
    let sys_argv: Vec<String> = {
        let mut v = vec!["pyxel".to_string()];
        v.extend_from_slice(&args[1..]);
        v
    };

    // Set up pyxel module path and patch runpy for RustPython compatibility
    let setup = format!(
        r#"
import sys
sys.argv = {sys_argv:?}

import pyxel
pyxel.__path__ = [r"{pyxel_dir}"]

# Patch run_python_script / play_pyxel_app to use exec instead of runpy
import pyxel.cli as _cli
import os as _os

def _run_python_script(python_script_file):
    python_script_file = _cli._complete_extension(python_script_file, "run", ".py")
    _cli._check_file_exists(python_script_file)
    python_script_file = _os.path.abspath(python_script_file)
    sys.path.insert(0, _os.path.dirname(python_script_file))
    _os.chdir(_os.path.dirname(python_script_file) or ".")
    with open(python_script_file) as _f:
        _code = _f.read()
    exec(compile(_code, python_script_file, "exec"), {{"__name__": "__main__", "__file__": python_script_file}})

def _play_pyxel_app(pyxel_app_file):
    file_ext = _os.path.splitext(pyxel_app_file)[1].lower()
    if file_ext != ".zip":
        pyxel_app_file = _cli._complete_extension(pyxel_app_file, "play", pyxel.APP_FILE_EXTENSION)
    _cli._check_file_exists(pyxel_app_file)
    _cli.print_pyxel_app_metadata(pyxel_app_file)
    startup_script_file = _cli._extract_pyxel_app(pyxel_app_file)
    if startup_script_file:
        sys.path.insert(0, _os.path.abspath(_os.path.dirname(startup_script_file)))
        _os.chdir(_os.path.dirname(startup_script_file))
        with open(startup_script_file) as _f:
            _code = _f.read()
        exec(compile(_code, startup_script_file, "exec"), {{"__name__": "__main__", "__file__": startup_script_file}})
        return
    print(f"file not found: '{{pyxel.APP_STARTUP_SCRIPT_FILE}}'")
    sys.exit(1)

_cli.run_python_script = _run_python_script
_cli.play_pyxel_app = _play_pyxel_app
"#,
        sys_argv = sys_argv,
        pyxel_dir = pyxel_dir.replace('\\', "\\\\"),
    );
    pyxel_embed::exec_source(&interp, &setup);

    // Dispatch via cli
    pyxel_embed::exec_source(
        &interp,
        "import sys; import pyxel.cli; pyxel.cli.cli(); sys.stdout.flush()",
    );
}
