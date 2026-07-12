use std::process::exit;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyxel::{Pyxel, PyxelCallback};

use crate::pyxel_singleton::pyxel;

// Lifecycle

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
    pyxel::validate_init_params(width, height, fps, display_scale, headless)
        .map_err(pyo3::exceptions::PyValueError::new_err)?;

    // Capture reset info before chdir
    let sys = py.import("sys")?;
    let os_mod = py.import("os")?;
    let exec_path: String = sys.getattr("executable")?.extract()?;
    let cwd: String = os_mod.call_method0("getcwd")?.extract()?;
    // Prefer Python's original argv so reset can restart the same command line.
    let orig_argv: Vec<String> = sys
        .getattr("orig_argv")
        .or_else(|_| sys.getattr("argv"))?
        .extract()?;

    // Change to script directory
    let locals = PyDict::new(py);
    locals.set_item("os", &os_mod)?;
    locals.set_item("inspect", py.import("inspect")?)?;
    py.run(
        c"os.chdir(os.path.dirname(inspect.stack()[1].filename) or \".\")",
        None,
        Some(&locals),
    )?;

    if !headless.unwrap_or(false) {
        let environ = os_mod.getattr("environ")?;
        let has_window_state: bool = environ
            .call_method1("__contains__", (pyxel::WINDOW_STATE_ENV,))?
            .extract()?;
        if !has_window_state {
            environ.set_item(pyxel::WINDOW_STATE_ENV, "")?;
        }
    }

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
    )
    .map_err(pyo3::exceptions::PyValueError::new_err)?;

    // Register reset callback
    *pyxel::reset_callback() = Some(Box::new(move |window_state| {
        Python::attach(|py| {
            let result: PyResult<()> = (|| {
                let locals = PyDict::new(py);
                locals.set_item("exec_path", &exec_path)?;
                locals.set_item("cwd", &cwd)?;
                locals.set_item("orig_argv", &orig_argv)?;
                locals.set_item("window_state", &window_state)?;
                locals.set_item("window_state_env", pyxel::WINDOW_STATE_ENV)?;
                py.run(
                    c"
import os, subprocess, sys
# 0x52 = WATCH_RESET_EXIT_CODE in settings.rs, checked by cli.py watch mode
if os.environ.get('PYXEL_WATCH_STATE_FILE'):
    os._exit(0x52)
if sys.platform == 'darwin':
    # Silence child stderr while the parent process is being replaced.
    try:
        f = open(os.devnull, 'wb')
        os.dup2(f.fileno(), 2)
        f.close()
    except OSError:
        pass
env = os.environ.copy()
if window_state is not None and window_state_env in env:
    env[window_state_env] = window_state
subprocess.Popen(
    [exec_path] + orig_argv[1:],
    cwd=cwd,
    env=env,
)
sys.exit(0)
",
                    None,
                    Some(&locals),
                )
            })();
            if let Err(err) = result {
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
        fn update(&mut self) {
            if let Err(err) = self.update.call0() {
                err.print(self.py);
                exit(1);
            }
        }

        fn draw(&mut self) {
            if let Err(err) = self.draw.call0() {
                err.print(self.py);
                exit(1);
            }
        }
    }

    Pyxel::run(PythonCallback { py, update, draw });
}

#[pyfunction]
fn show() {
    Pyxel::show_screen();
}

#[pyfunction]
fn flip() {
    Pyxel::flip_screen();
}

#[pyfunction]
fn quit() {
    Pyxel::quit();
}

#[pyfunction]
fn reset() {
    Pyxel::restart();
}

// Window settings

#[pyfunction]
fn title(title: &str) {
    pyxel().set_title(title);
}

#[pyfunction]
#[pyo3(signature = (data, scale, colkey=None))]
fn icon(data: Vec<String>, scale: u32, colkey: Option<pyxel::Color>) -> PyResult<()> {
    pyxel()
        .set_icon(&data, scale, colkey)
        .map_err(pyo3::exceptions::PyValueError::new_err)
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
fn resize(width: u32, height: u32) -> PyResult<()> {
    pyxel()
        .set_screen_size(width, height)
        .map_err(pyo3::exceptions::PyValueError::new_err)
}

// Internal helpers

#[cfg(target_os = "emscripten")]
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

// Module registration

pub fn add_system_functions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(flip, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;
    m.add_function(wrap_pyfunction!(reset, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(icon, m)?)?;
    m.add_function(wrap_pyfunction!(perf_monitor, m)?)?;
    m.add_function(wrap_pyfunction!(integer_scale, m)?)?;
    m.add_function(wrap_pyfunction!(screen_mode, m)?)?;
    m.add_function(wrap_pyfunction!(fullscreen, m)?)?;
    m.add_function(wrap_pyfunction!(resize, m)?)?;
    #[cfg(target_os = "emscripten")]
    m.add_function(wrap_pyfunction!(_reset_statics, m)?)?;
    #[cfg(not(target_os = "emscripten"))]
    m.add_function(wrap_pyfunction!(_pid_exists, m)?)?;
    Ok(())
}
