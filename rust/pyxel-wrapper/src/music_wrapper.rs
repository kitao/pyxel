use std::sync::Once;

use pyo3::prelude::*;
use pyo3::types::{PyList, PyTuple};

static SNDS_LIST_ONCE: Once = Once::new();

wrap_as_python_list!(
    Seq,
    pyxel::SharedSeq,
    (|inner: &pyxel::SharedSeq| inner.lock().len()),
    u32,
    (|inner: &pyxel::SharedSeq, index| inner.lock()[index]),
    u32,
    (|inner: &pyxel::SharedSeq, index, value| inner.lock()[index] = value),
    Vec<u32>,
    (|inner: &pyxel::SharedSeq, list| *inner.lock() = list),
    (|inner: &pyxel::SharedSeq| inner.lock().clone())
);

wrap_as_python_list!(
    Seqs,
    pyxel::SharedMusic,
    (|inner: &pyxel::SharedMusic| inner.lock().seqs.len()),
    Seq,
    (|inner: &pyxel::SharedMusic, index: usize| Seq::wrap(inner.lock().seqs[index].clone())),
    Vec<u32>,
    (|inner: &pyxel::SharedMusic, index: usize, value: Vec<u32>| *inner.lock().seqs[index]
        .lock() = value),
    Vec<Vec<u32>>,
    (|inner: &pyxel::SharedMusic, list: Vec<Vec<u32>>| inner.lock().set(&list)),
    (|inner: &pyxel::SharedMusic| inner
        .lock()
        .seqs
        .iter()
        .map(|seq| seq.lock().clone())
        .collect())
);

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pub(crate) inner: pyxel::SharedMusic,
}

impl Music {
    pub fn wrap(inner: pyxel::SharedMusic) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Music::new())
    }

    #[getter]
    pub fn seqs(&self) -> Seqs {
        Seqs::wrap(self.inner.clone())
    }

    #[pyo3(signature = (*seqs))]
    pub fn set(&self, seqs: &Bound<'_, PyTuple>) {
        let mut rust_seqs: Vec<Vec<u32>> = Vec::new();
        for i in 0..seqs.len() {
            let rust_seq = seqs
                .get_item(i)
                .unwrap()
                .downcast::<PyList>()
                .unwrap()
                .extract::<Vec<u32>>()
                .unwrap();
            rust_seqs.push(rust_seq);
        }
        self.inner.lock().set(&rust_seqs);
    }

    #[getter]
    pub fn snds_list(&self) -> Seqs {
        SNDS_LIST_ONCE.call_once(|| {
            println!("Music.snds_list[ch] is deprecated, use Music.seqs[ch] instead.",);
        });
        Seqs::wrap(self.inner.clone())
    }
}

pub fn add_music_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Seqs>()?;
    m.add_class::<Music>()?;
    Ok(())
}
