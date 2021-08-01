use pyo3::prelude::*;

use pyxel as engine;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn pyxel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::py_run;
    use pyo3::types::PyList;

    #[test]
    fn test() {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let list = PyList::new(py, &[1, 2, 3]);
        py_run!(py, list, "assert list == [1, 2, 3]");
    }
}
