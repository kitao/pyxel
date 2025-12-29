use std::ffi::CString;
use std::process::exit;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyxel::{Pyxel, PyxelCallback};

use crate::pyxel_singleton::{pyxel, set_pyxel_instance};

#[pyfunction]
#[pyo3(
    signature = (width, height, title=None, fps=None, quit_key=None, display_scale=None, capture_scale=None, capture_sec=None)
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
) -> PyResult<()> {
    let locals = PyDict::new(py);
    locals.set_item("os", py.import("os")?)?;
    locals.set_item("inspect", py.import("inspect")?)?;
    let script =
        CString::new(r#"os.chdir(os.path.dirname(inspect.stack()[1].filename) or ".")"#).unwrap();

    py.run(script.as_c_str(), None, Some(&locals))?;

    set_pyxel_instance(pyxel::init(
        width,
        height,
        title,
        fps,
        quit_key,
        display_scale,
        capture_scale,
        capture_sec,
    ));

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
    pyxel().show();
}

#[pyfunction]
fn flip() {
    pyxel().flip();
}

#[pyfunction]
fn reset() {
    pyxel().reset();
}

#[pyfunction]
fn quit() {
    pyxel().quit();
}

#[pyfunction]
fn title(title: &str) {
    pyxel().title(title);
}

#[pyfunction]
#[pyo3(signature = (data, scale, colkey=None))]
fn icon(data: Vec<String>, scale: u32, colkey: Option<pyxel::Color>) {
    let data_refs: Vec<_> = data.iter().map(String::as_str).collect();
    pyxel().icon(&data_refs, scale, colkey);
}

#[pyfunction]
fn perf_monitor(enabled: bool) {
    pyxel().perf_monitor(enabled);
}

#[pyfunction]
fn integer_scale(enabled: bool) {
    pyxel().integer_scale(enabled);
}

#[pyfunction]
fn screen_mode(scr: u32) {
    pyxel().screen_mode(scr);
}

#[pyfunction]
fn fullscreen(enabled: bool) {
    pyxel().fullscreen(enabled);
}

#[pyfunction]
fn _set_reset_func(func: Bound<'_, PyAny>) {
    let func = func.unbind();
    *pyxel::RESET_FUNC.lock() = Some(Box::new(move || {
        Python::attach(|py| {
            if let Err(err) = func.call0(py) {
                err.print(py);
                exit(1);
            }
        });
    }));
}

#[pyfunction]
fn _reset_statics() {
    pyxel::reset_statics();
}

#[cfg(not(target_os = "emscripten"))]
#[pyfunction]
fn _pid_exists(pid: u32) -> bool {
    {
        let system = sysinfo::System::new_all();
        system.process(sysinfo::Pid::from_u32(pid)).is_some()
    }
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
    m.add_function(wrap_pyfunction!(_set_reset_func, m)?)?;
    m.add_function(wrap_pyfunction!(_reset_statics, m)?)?;
    m.add_function(wrap_pyfunction!(_pid_exists, m)?)?;
    Ok(())
}
