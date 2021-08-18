use pyo3::prelude::*;
use pyo3::types::PyAny;

use pyxel::{Pyxel, PyxelCallback};

use crate::{i32_to_u32, instance, set_instance};

#[pyfunction]
fn init<'a>(
    width: i32,
    height: i32,
    title: Option<&str>,
    scale: Option<u32>,
    fps: Option<u32>,
    quit_key: Option<pyxel::Key>,
) {
    set_instance(Pyxel::new(
        i32_to_u32(width),
        i32_to_u32(height),
        title,
        scale,
        fps,
        quit_key,
    ));
}

#[pyfunction]
fn title(title: &str) {
    instance().title(title);
}

#[pyfunction]
fn fullscreen() {
    instance().fullscreen();
}

#[pyfunction]
fn run(update: &PyAny, draw: &PyAny) {
    struct PythonCallback<'a> {
        update: &'a PyAny,
        draw: &'a PyAny,
    }

    impl<'a> PyxelCallback for PythonCallback<'a> {
        fn update(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.update.call0() {
                panic!("{}", err);
            }
        }

        fn draw(&mut self, _pyxel: &mut Pyxel) {
            if let Err(err) = self.draw.call0() {
                panic!("{}", err);
            }
        }
    }

    instance().run(&mut PythonCallback {
        update: update,
        draw: draw,
    });
}

#[pyfunction]
fn show() {
    instance().show();
}

#[pyfunction]
fn flip() {
    instance().flip();
}

#[pyfunction]
fn quit() {
    instance().quit();
}

pub fn add_system_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(init, m)?)?;
    m.add_function(wrap_pyfunction!(title, m)?)?;
    m.add_function(wrap_pyfunction!(fullscreen, m)?)?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(show, m)?)?;
    m.add_function(wrap_pyfunction!(flip, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;

    Ok(())
}
