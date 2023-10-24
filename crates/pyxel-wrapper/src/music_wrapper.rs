use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

wrap_as_python_list!(
    Seqs,
    Vec<u32>,
    pyxel::SharedMusic,
    (|inner: &pyxel::SharedMusic| inner.lock().seqs.len()),
    (|inner: &pyxel::SharedMusic, index: usize| inner.lock().seqs[index].clone()),
    (|inner: &pyxel::SharedMusic, index, value| inner.lock().seqs[index] = value),
    (|inner: &pyxel::SharedMusic, index, value| inner.lock().seqs.insert(index, value)),
    (|inner: &pyxel::SharedMusic, index| inner.lock().seqs.remove(index))
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

    #[getter]
    pub fn seqs(&self) -> Seqs {
        Seqs {
            inner: self.inner.clone(),
        }
    }

    #[pyo3(signature = (*seqs))]
    pub fn set(&self, seqs: &PyTuple) {
        let mut rust_seqs: Vec<Vec<u32>> = Vec::new();
        for i in 0..seqs.len() {
            let seq = seqs.get_item(i).unwrap().downcast::<PyList>().unwrap();
            let rust_seq = seq.extract::<Vec<u32>>().unwrap();
            rust_seqs.push(rust_seq);
        }
        self.inner.lock().set(&rust_seqs);
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
