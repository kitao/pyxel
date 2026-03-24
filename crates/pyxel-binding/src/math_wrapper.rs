use std::cmp::Ordering;

use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyInt};
use pyxel::Pyxel;

#[pyfunction]
fn ceil(x: f32) -> i32 {
    Pyxel::ceil(x)
}

#[pyfunction]
fn floor(x: f32) -> i32 {
    Pyxel::floor(x)
}

/// Returns the integer type for int inputs, float type for float inputs.
#[pyfunction]
fn clamp(
    x: Bound<'_, PyAny>,
    lower: Bound<'_, PyAny>,
    upper: Bound<'_, PyAny>,
) -> PyResult<Py<PyAny>> {
    let py = x.py();

    // Try integer path first to preserve Python int type
    if let (Ok(xi), Ok(li), Ok(ui)) = (
        x.extract::<i64>(),
        lower.extract::<i64>(),
        upper.extract::<i64>(),
    ) {
        let (lo, hi) = if li < ui { (li, ui) } else { (ui, li) };
        let v = xi.clamp(lo, hi);
        return Ok(PyInt::new(py, v).into_any().unbind());
    }

    let xf = x.extract::<f64>()?;
    let lf = lower.extract::<f64>()?;
    let uf = upper.extract::<f64>()?;
    let (lo, hi) = if lf < uf { (lf, uf) } else { (uf, lf) };
    Ok(PyFloat::new(py, xf.clamp(lo, hi)).into_any().unbind())
}

/// Returns the integer type for int inputs, float type for float inputs.
#[pyfunction]
fn sgn(x: Bound<'_, PyAny>) -> PyResult<Py<PyAny>> {
    let py = x.py();

    // Try integer path first to preserve Python int type
    if let Ok(xi) = x.extract::<i64>() {
        let v = match xi.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Less => -1,
            Ordering::Equal => 0,
        };
        return Ok(PyInt::new(py, v).into_any().unbind());
    }

    let xf = x.extract::<f64>()?;
    let v = match xf.partial_cmp(&0.0) {
        Some(Ordering::Greater) => 1.0,
        Some(Ordering::Less) => -1.0,
        _ => 0.0,
    };
    Ok(PyFloat::new(py, v).into_any().unbind())
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
    Pyxel::random_seed(seed);
}

#[pyfunction]
fn rndi(a: i32, b: i32) -> i32 {
    Pyxel::random_int(a, b)
}

#[pyfunction]
fn rndf(a: f32, b: f32) -> f32 {
    Pyxel::random_float(a, b)
}

#[pyfunction]
fn nseed(seed: u32) {
    Pyxel::noise_seed(seed);
}

#[pyfunction]
#[pyo3(signature = (x, y=None, z=None))]
fn noise(x: f32, y: Option<f32>, z: Option<f32>) -> f32 {
    Pyxel::noise(x, y.unwrap_or(0.0), z.unwrap_or(0.0))
}

pub fn add_math_functions(m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ceil, m)?)?;
    m.add_function(wrap_pyfunction!(floor, m)?)?;
    m.add_function(wrap_pyfunction!(clamp, m)?)?;
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
