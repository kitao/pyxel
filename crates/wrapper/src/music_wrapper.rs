use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyxel::Music as PyxelMusic;
use pyxel::SharedMusic as PyxelSharedMusic;
use pyxel::NUM_CHANNELS;

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

#[pymethods]
impl Sequence {
    define_list_edit_methods!(u32);
}

#[pyproto]
impl PySequenceProtocol for Sequence {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<u32> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: u32) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(Self::list_mut, self, index)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sequences {
    sequences: Vec<Sequence>,
}

impl Sequences {
    fn new(pyxel_music: PyxelSharedMusic) -> Self {
        let sequences = (0..NUM_CHANNELS)
            .map(|channel_no| Sequence::new(pyxel_music.clone(), channel_no as u32))
            .collect();
        Self { sequences }
    }

    fn list(&self) -> &Vec<Sequence> {
        unsafe { &*(&self.sequences as *const Vec<Sequence>) }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Music {
    pyxel_music: PyxelSharedMusic,
}

#[pyproto]
impl PySequenceProtocol for Sequences {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Sequence> {
        define_list_get_operator!(Self::list, self, index)
    }
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
