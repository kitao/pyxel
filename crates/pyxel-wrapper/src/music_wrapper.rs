use pyo3::prelude::*;

wrap_as_python_list!(
    Chain,
    u32,
    pyxel::SharedChain,
    (|inner: &pyxel::SharedChain| inner.lock().len()),
    (|inner: &pyxel::SharedChain, index| inner.lock()[index]),
    (|inner: &pyxel::SharedChain, index, value| inner.lock()[index] = value),
    (|inner: &pyxel::SharedChain, index, value| inner.lock().insert(index, value)),
    (|inner: &pyxel::SharedChain, index| inner.lock().remove(index))
);

wrap_as_python_list!(
    Chains,
    Chain,
    pyxel::SharedMusic,
    (|inner: &pyxel::SharedMusic| inner.lock().chains.len()),
    (|inner: &pyxel::SharedMusic, index: usize| Chain {
        inner: inner.lock().chains[index].clone()
    }),
    (|inner: &pyxel::SharedMusic, index, value: Chain| inner.lock().chains[index] = value.inner),
    (|inner: &pyxel::SharedMusic, index, value: Chain| inner
        .lock()
        .chains
        .insert(index, value.inner)),
    (|inner: &pyxel::SharedMusic, index| inner.lock().chains.remove(index))
);

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pub(crate) inner: pyxel::SharedMusic,
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: pyxel::Music::new(),
        }
    }

    pub fn set(&self, chain0: Vec<u32>, chain1: Vec<u32>, chain2: Vec<u32>, chain3: Vec<u32>) {
        self.inner.lock().set(&chain0, &chain1, &chain2, &chain3);
    }

    #[getter]
    pub fn chains(&self) -> Chains {
        Chains {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn snds_list(&self) -> Chains {
        Chains {
            inner: self.inner.clone(),
        }
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Chain>()?;
    m.add_class::<Chains>()?;
    m.add_class::<Music>()?;
    Ok(())
}
