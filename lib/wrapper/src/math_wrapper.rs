use pyo3::prelude::*;

use crate::instance;

#[pyfunction]
fn sgn(x: f64) -> f64 {
    instance().sgn(x)
}

#[pyfunction]
fn sqrt(x: f64) -> f64 {
    instance().sqrt(x)
}

#[pyfunction]
fn sin(deg: f64) -> f64 {
    instance().sin(deg)
}

#[pyfunction]
fn cos(deg: f64) -> f64 {
    instance().cos(deg)
}

#[pyfunction]
fn atan2(y: f64, x: f64) -> f64 {
    instance().atan2(y, x)
}

#[pyfunction]
fn srand(seed: u32) {
    instance().srand(seed);
}

#[pyfunction]
fn rndi(a: i32, b: i32) -> i32 {
    instance().rndi(a, b)
}

#[pyfunction]
fn rndf(a: f64, b: f64) -> f64 {
    instance().rndf(a, b)
}

#[pyfunction]
fn nseed(seed: u32) {
    instance().nseed(seed);
}

#[pyfunction]
fn noise(x: f64, y: Option<f64>, z: Option<f64>) -> f64 {
    let y = y.unwrap_or(0.0);
    let z = z.unwrap_or(0.0);
    instance().noise(x, y, z)
}

pub fn add_math_functions(m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sgn, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(atan2, m)?)?;
    m.add_function(wrap_pyfunction!(srand, m)?)?;
    m.add_function(wrap_pyfunction!(rndi, m)?)?;
    m.add_function(wrap_pyfunction!(rndf, m)?)?;
    m.add_function(wrap_pyfunction!(nseed, m)?)?;
    m.add_function(wrap_pyfunction!(noise, m)?)?;
    Ok(())
}
