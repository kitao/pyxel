use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

wrap_as_python_list!(
    Seq,
    u32,
    (pyxel::SharedMusic, usize),
    (|inner: &(pyxel::SharedMusic, usize)| inner.0.lock().seqs[inner.1].len()),
    (|inner: &(pyxel::SharedMusic, usize), index: usize| inner.0.lock().seqs[inner.1][index]
        .clone()),
    (|inner: &(pyxel::SharedMusic, usize), index, value| inner.0.lock().seqs[inner.1][index] =
        value),
    (|inner: &(pyxel::SharedMusic, usize), index, value| inner.0.lock().seqs[inner.1]
        .insert(index, value)),
    (|inner: &(pyxel::SharedMusic, usize), index| inner.0.lock().seqs[inner.1].remove(index))
);

wrap_as_python_list!(
    Seqs,
    Seq,
    pyxel::SharedMusic,
    (|inner: &pyxel::SharedMusic| inner.lock().seqs.len()),
    (|inner: &pyxel::SharedMusic, index: usize| Seq {
        inner: (inner.clone(), index)
    }),
    (|inner: &pyxel::SharedMusic, index, value: Seq| {
        let value = &value.inner.0.lock().seqs[value.inner.1];
        inner.lock().seqs[index] = value.clone();
    }),
    (|inner: &pyxel::SharedMusic, index, value: Seq| {
        let value = &value.inner.0.lock().seqs[value.inner.1];
        inner.lock().seqs.insert(index, value.clone());
    }),
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
