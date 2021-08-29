use std::process::exit;

use pyo3::prelude::*;
use pyo3::types::PyAny;

use pyxel::{Pyxel, PyxelCallback};

use crate::{instance, set_instance};

#[pyfunction]
fn init<'a>(
    width: u32,
    height: u32,
    title: Option<&str>,
    fps: Option<u32>,
    quit_key: Option<pyxel::Key>,
    capture_sec: Option<u32>,
) -> PyResult<()> {
    set_instance(Pyxel::new(width, height, title, fps, quit_key, capture_sec));

    Ok(())
}

#[pyfunction]
fn title(title: &str) -> PyResult<()> {
    instance().title(title);

    Ok(())
}

#[pyfunction]
fn icon(data: Vec<&str>, scale: u32) -> PyResult<()> {
    instance().icon(&data, scale);

    Ok(())
}

#[pyfunction]
fn fullscreen() -> PyResult<()> {
    instance().fullscreen();

    Ok(())
}

#[pyfunction]
fn run(py: Python, update: &PyAny, draw: &PyAny) -> PyResult<()> {
    struct PythonCallback<'a> {
        py: Python<'a>,
        update: &'a PyAny,
        draw: &'a PyAny,
    }

    impl<'a> PyxelCallback for PythonCallback<'a> {
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

    instance().run(&mut PythonCallback {
        py: py,
        update: update,
        draw: draw,
    });

    Ok(())
}

#[pyfunction]
fn show() -> PyResult<()> {
    instance().show();

    Ok(())
}

#[pyfunction]
fn flip() -> PyResult<()> {
    instance().flip();

    Ok(())
}

#[pyfunction]
fn quit() -> PyResult<()> {
    instance().quit();

    Ok(())
}

pub fn add_system_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(icon, m)?)?;
    m.add_function(wrap_pyfunction!(fullscreen, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(flip, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;

    Ok(())
}
