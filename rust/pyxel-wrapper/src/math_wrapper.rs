use pyo3::prelude::*;
use pyxel::Pyxel;

#[pyfunction]
fn ceil(x: f32) -> i32 {
    Pyxel::ceil(x)
}

#[pyfunction]
fn floor(x: f32) -> i32 {
    Pyxel::floor(x)
}

#[pyfunction]
fn sgn(x: f32) -> i32 {
    Pyxel::sgn(x)
}

#[pyfunction]
fn sqrt(x: f32) -> f32 {
    Pyxel::sqrt(x)
}

#[pyfunction]
fn sin(deg: f32) -> f32 {
    Pyxel::sin(deg)
}

#[pyfunction]
fn cos(deg: f32) -> f32 {
    Pyxel::cos(deg)
}

#[pyfunction]
fn atan2(y: f32, x: f32) -> f32 {
    Pyxel::atan2(y, x)
}

#[pyfunction]
fn rseed(seed: u32) {
    Pyxel::rseed(seed);
}

#[pyfunction]
fn rndi(a: i32, b: i32) -> i32 {
    Pyxel::rndi(a, b)
}

#[pyfunction]
fn rndf(a: f32, b: f32) -> f32 {
    Pyxel::rndf(a, b)
}

#[pyfunction]
fn nseed(seed: u32) {
    Pyxel::nseed(seed);
}

#[pyfunction]
#[pyo3(signature = (x, y=None, z=None))]
fn noise(x: f32, y: Option<f32>, z: Option<f32>) -> f32 {
    let y = y.unwrap_or(0.0);
    let z = z.unwrap_or(0.0);
    Pyxel::noise(x, y, z)
}

pub fn add_math_functions(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ceil, m)?)?;
    m.add_function(wrap_pyfunction!(floor, m)?)?;
    m.add_function(wrap_pyfunction!(sgn, m)?)?;
    m.add_function(wrap_pyfunction!(sqrt, m)?)?;
    m.add_function(wrap_pyfunction!(sin, m)?)?;
    m.add_function(wrap_pyfunction!(cos, m)?)?;
    m.add_function(wrap_pyfunction!(atan2, m)?)?;
    m.add_function(wrap_pyfunction!(rseed, m)?)?;
    m.add_function(wrap_pyfunction!(rndi, m)?)?;
    m.add_function(wrap_pyfunction!(rndf, m)?)?;
    m.add_function(wrap_pyfunction!(nseed, m)?)?;
    m.add_function(wrap_pyfunction!(noise, m)?)?;
    Ok(())
}
