use pyo3::prelude::*;
use pyxel::cube::shading::LEVEL_COUNT;

use super::vec3::Vec3;

define_wrapper!(Shading, pyxel::cube::Shading);

#[pymethods]
impl Shading {
    #[new]
    fn new(colors: Vec<u32>) -> Self {
        let palette: Vec<pyxel::Rgb24> = colors.into_iter().map(|c| c as pyxel::Rgb24).collect();
        Self::wrap(pyxel::cube::Shading::new(&palette))
    }

    #[getter]
    fn direction(&self) -> Vec3 {
        Vec3::wrap(self.inner_ref().direction.clone())
    }

    #[setter]
    fn set_direction(&self, v: PyRef<'_, Vec3>) {
        self.inner_mut().direction = v.inner.clone();
    }

    fn __getitem__(&self, key: (usize, usize)) -> PyResult<(i32, i32)> {
        let (col, level) = key;
        let r = self.inner_ref();
        if col >= r.palette_size() || level >= LEVEL_COUNT {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Shading index out of range",
            ));
        }
        Ok(r.get(col, level))
    }

    fn __setitem__(&self, key: (usize, usize), value: (i32, i32)) -> PyResult<()> {
        let (col, level) = key;
        let n = self.inner_ref().palette_size();
        if col >= n || level >= LEVEL_COUNT {
            return Err(pyo3::exceptions::PyIndexError::new_err(
                "Shading index out of range",
            ));
        }
        self.inner_mut().set(col, level, value);
        Ok(())
    }

    fn build(&self, colors: Vec<u32>) {
        let palette: Vec<pyxel::Rgb24> = colors.into_iter().map(|c| c as pyxel::Rgb24).collect();
        self.inner_mut().build(&palette);
    }

    fn __repr__(&self) -> String {
        let r = self.inner_ref();
        format!("Shading({} × {})", r.palette_size(), LEVEL_COUNT)
    }
}

pub fn add_shading_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Shading>()?;
    Ok(())
}
