use std::ffi::CString;
use std::process::exit;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyxel::{Pyxel, PyxelCallback};

use crate::pyxel_singleton::pyxel;

#[pyfunction]
#[pyo3(
    signature = (width, height, title=None, fps=None, quit_key=None, display_scale=None, capture_scale=None, capture_sec=None, headless=None)
)]
fn init(
    py: Python,
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<pyxel::Key>,
    display_scale: Option<u32>,
    capture_scale: Option<u32>,
    capture_sec: Option<u32>,
    headless: Option<bool>,
) -> PyResult<()> {
    // Capture reset info before chdir
    let sys = py.import("sys")?;
    let os_mod = py.import("os")?;
    let exec_path: String = sys.getattr("executable")?.extract()?;
    let cwd: String = os_mod.call_method0("getcwd")?.extract()?;
    let orig_argv: Vec<String> = sys
        .getattr("orig_argv")
        .or_else(|_| sys.getattr("argv"))?
        .extract()?;

    // Change to script directory
    let locals = PyDict::new(py);
    locals.set_item("os", os_mod)?;
    locals.set_item("inspect", py.import("inspect")?)?;
    let script =
        CString::new(r#"os.chdir(os.path.dirname(inspect.stack()[1].filename) or ".")"#).unwrap();
    py.run(script.as_c_str(), None, Some(&locals))?;

    pyxel::init(
        width,
        height,
        title,
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
        headless,
    );

    // Register reset callback
    *pyxel::reset_callback() = Some(Box::new(move || {
        Python::attach(|py| {
            let locals = PyDict::new(py);
            locals.set_item("exec_path", &exec_path).unwrap();
            locals.set_item("cwd", &cwd).unwrap();
            locals.set_item("orig_argv", &orig_argv).unwrap();
            let script = CString::new(
                r"
import os, subprocess, sys
if os.environ.get('PYXEL_WATCH_STATE_FILE'):
    os._exit(0x52)
if sys.platform == 'darwin':
    try:
        f = open(os.devnull, 'wb')
        os.dup2(f.fileno(), 2)
        f.close()
    except OSError:
        pass
subprocess.Popen(
    [exec_path] + orig_argv[1:],
    cwd=cwd,
    env=os.environ.copy(),
)
sys.exit(0)
",
            )
            .unwrap();
            if let Err(err) = py.run(script.as_c_str(), None, Some(&locals)) {
                err.print(py);
                exit(1);
            }
        });
    }));

    // Register quit callback to run Python atexit handlers
    *pyxel::quit_callback() = Some(Box::new(|| {
        Python::attach(|py| {
            let _ = py.run(c"import atexit; atexit._run_exitfuncs()", None, None);
        });
    }));

    Ok(())
}

#[pyfunction]
fn run<'py>(py: Python, update: Bound<'py, PyAny>, draw: Bound<'py, PyAny>) {
    struct PythonCallback<'a> {
        py: Python<'a>,
        update: Bound<'a, PyAny>,
        draw: Bound<'a, PyAny>,
    }

    impl PyxelCallback for PythonCallback<'_> {
        fn update(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.update.call0() {
                err.print(self.py);
                exit(1);
            }
        }

        fn draw(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.draw.call0() {
                err.print(self.py);
                exit(1);
            }
        }
    }

    pyxel().run(PythonCallback { py, update, draw });
}

#[pyfunction]
fn show() {
    pyxel().show_screen();
}

#[pyfunction]
fn flip() {
    pyxel().flip_screen();
}

#[pyfunction]
fn reset() {
    pyxel().restart();
}

#[pyfunction]
fn quit() {
    pyxel().quit();
}

#[pyfunction]
fn title(title: &str) {
    pyxel().set_title(title);
}

#[pyfunction]
#[pyo3(signature = (data, scale, colkey=None))]
fn icon(data: Vec<String>, scale: u32, colkey: Option<pyxel::Color>) {
    let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
    pyxel().set_icon(&data_refs, scale, colkey);
}

#[pyfunction]
fn perf_monitor(enabled: bool) {
    pyxel().set_perf_monitor(enabled);
}

#[pyfunction]
fn integer_scale(enabled: bool) {
    pyxel().set_integer_scale(enabled);
}

#[pyfunction]
fn screen_mode(scr: u32) {
    pyxel().set_screen_mode(scr);
}

#[pyfunction]
fn fullscreen(enabled: bool) {
    pyxel().set_fullscreen(enabled);
}

#[pyfunction]
fn _reset_statics() {
    pyxel::reset_statics();
}

#[cfg(not(target_os = "emscripten"))]
#[pyfunction]
fn _pid_exists(pid: u32) -> bool {
    let system = sysinfo::System::new_all();
    system.process(sysinfo::Pid::from_u32(pid)).is_some()
}

#[cfg(target_os = "emscripten")]
#[pyfunction]
fn _pid_exists(_pid: u32) -> bool {
    false
}

pub fn add_system_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(flip, m)?)?;
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(icon, m)?)?;
    m.add_function(wrap_pyfunction!(perf_monitor, m)?)?;
    m.add_function(wrap_pyfunction!(integer_scale, m)?)?;
    m.add_function(wrap_pyfunction!(screen_mode, m)?)?;
    m.add_function(wrap_pyfunction!(fullscreen, m)?)?;
    m.add_function(wrap_pyfunction!(_reset_statics, m)?)?;
    m.add_function(wrap_pyfunction!(_pid_exists, m)?)?;
    Ok(())
}
