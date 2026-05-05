use pyo3::prelude::*;

use pyxel::cube::ramp::LEVEL_COUNT;

define_wrapper!(Ramp, pyxel::cube::Ramp);

#[pymethods]
impl Ramp {
    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::cube::Ramp::new())
    }

    fn __getitem__(&self, key: (usize, usize)) -> PyResult<i32> {
        let (col, level) = key;
        let r = self.inner_ref();
        if col >= r.palette_size() || level >= LEVEL_COUNT {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Ramp index out of range",
            ));
        }
        Ok(r.get(col, level))
    }

    fn __setitem__(&self, key: (usize, usize), value: i32) -> PyResult<()> {
        let (col, level) = key;
        let n = self.inner_ref().palette_size();
        if col >= n || level >= LEVEL_COUNT {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Ramp index out of range",
            ));
        }
        self.inner_mut().set(col, level, value);
        Ok(())
    }

    fn build(&self) {
        self.inner_mut().build();
    }

    fn __repr__(&self) -> String {
        let r = self.inner_ref();
        format!("Ramp({} × {})", r.palette_size(), LEVEL_COUNT)
    }
}

pub fn add_ramp_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Ramp>()?;
    Ok(())
}
