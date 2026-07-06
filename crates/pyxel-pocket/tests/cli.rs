use std::process::Command;

fn run_pyxel_pocket(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_pyxel-pocket"))
        .args(args)
        .output()
        .expect("failed to run pyxel-pocket")
}

#[test]
fn no_args_prints_pyxel_style_usage() {
    let output = run_pyxel_pocket(&[]);

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("pyxel-pocket "));
    assert!(stdout.contains(", a standalone Pyxel Player\n"));
    assert!(stdout.contains("usage:\n"));
    assert!(stdout.contains("    pyxel-pocket PYTHON_SCRIPT_FILE(.py)\n"));
    assert!(stdout.contains("    pyxel-pocket PYXEL_APP_FILE(.pyxapp)\n"));
    assert!(!stdout.contains("Usage:"));
}

#[test]
fn too_many_args_prints_error_and_usage() {
    let output = run_pyxel_pocket(&["a.py", "b.py"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).expect("stdout should be utf-8");
    assert!(stdout.contains("invalid number of parameters\n"));
    assert!(stdout.contains("usage:\n"));
    assert!(stdout.contains("    pyxel-pocket PYTHON_SCRIPT_FILE(.py)\n"));
    assert!(stdout.contains("    pyxel-pocket PYXEL_APP_FILE(.pyxapp)\n"));
}
