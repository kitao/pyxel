use pyo3::class::PySequenceProtocol;
use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyxel::Music as PyxelMusic;
use pyxel::SharedMusic as PyxelSharedMusic;

#[pyclass]
#[derive(Clone)]
pub struct Sequence {
    pyxel_music: PyxelSharedMusic,
    channel_no: u32,
}

impl Sequence {
    fn new(pyxel_music: PyxelSharedMusic, channel_no: u32) -> Self {
        Self {
            pyxel_music,
            channel_no,
        }
    }

    fn list(&self) -> &Vec<u32> {
        unsafe {
            &*(&self.pyxel_music.lock().sequences[self.channel_no as usize] as *const Vec<u32>)
        }
    }

    fn list_mut(&mut self) -> &mut Vec<u32> {
        unsafe {
            &mut *(&mut self.pyxel_music.lock().sequences[self.channel_no as usize]
                as *mut Vec<u32>)
        }
    }
}

#[pyproto]
impl PySequenceProtocol for Sequence {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<u32> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: u32) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }
}

#[pymethods]
impl Sequence {
    pub fn from_list(&mut self, list: Vec<u32>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, list)
    }

    pub fn to_list(&self) -> PyResult<Vec<u32>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sequences {
    pyxel_music: PyxelSharedMusic,
}

impl Sequences {
    fn new(pyxel_music: PyxelSharedMusic) -> Self {
        Self { pyxel_music }
    }
}

#[pyproto]
impl PySequenceProtocol for Sequences {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.pyxel_music.lock().sequences.len())
    }

    fn __getitem__(&self, index: isize) -> PyResult<Sequence> {
        if index < self.__len__().unwrap() as isize {
            Ok(Sequence::new(self.pyxel_music.clone(), index as u32))
        } else {
            Err(PyIndexError::new_err("list index out of range"))
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pyxel_music: PyxelSharedMusic,
}

pub fn wrap_pyxel_music(pyxel_music: PyxelSharedMusic) -> Music {
    Music { pyxel_music }
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Self {
        wrap_pyxel_music(PyxelMusic::new())
    }

    pub fn set(&self, seq0: Vec<u32>, seq1: Vec<u32>, seq2: Vec<u32>, seq3: Vec<u32>) {
        self.pyxel_music.lock().set(&seq0, &seq1, &seq2, &seq3);
    }

    #[getter]
    pub fn sequences(&self) -> Sequences {
        Sequences::new(self.pyxel_music.clone())
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Sequence>()?;
    m.add_class::<Sequences>()?;
    m.add_class::<Music>()?;
    Ok(())
}
