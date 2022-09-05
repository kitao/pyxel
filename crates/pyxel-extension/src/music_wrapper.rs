use std::ptr;

use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;
use pyxel::{Music as PyxelMusic, SharedMusic as PyxelSharedMusic};

#[pyclass]
#[derive(Clone)]
pub struct Sounds {
    pyxel_music: PyxelSharedMusic,
    channel_no: u32,
}

impl Sounds {
    fn new(pyxel_music: PyxelSharedMusic, channel_no: u32) -> Self {
        Self {
            pyxel_music,
            channel_no,
        }
    }

    fn list(&self) -> &Vec<u32> {
        unsafe { &*ptr::addr_of!(self.pyxel_music.lock().sounds_list[self.channel_no as usize]) }
    }

    fn list_mut(&mut self) -> &mut Vec<u32> {
        unsafe {
            &mut *ptr::addr_of_mut!(self.pyxel_music.lock().sounds_list[self.channel_no as usize])
        }
    }
}

#[pymethods]
impl Sounds {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<u32> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: u32) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<u32>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<u32>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct SoundsList {
    pyxel_music: PyxelSharedMusic,
}

impl SoundsList {
    fn new(pyxel_music: PyxelSharedMusic) -> Self {
        Self { pyxel_music }
    }
}

#[pymethods]
impl SoundsList {
    fn __len__(&self) -> usize {
        self.pyxel_music.lock().sounds_list.len()
    }

    fn __getitem__(&self, index: isize) -> PyResult<Sounds> {
        if index < self.__len__() as isize {
            Ok(Sounds::new(self.pyxel_music.clone(), index as u32))
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

pub const fn wrap_pyxel_music(pyxel_music: PyxelSharedMusic) -> Music {
    Music { pyxel_music }
}

#[pymethods]
impl Music {
    #[new]
    pub fn new() -> Self {
        wrap_pyxel_music(PyxelMusic::new())
    }

    pub fn set(&self, snds0: Vec<u32>, snds1: Vec<u32>, snds2: Vec<u32>, snds3: Vec<u32>) {
        self.pyxel_music.lock().set(&snds0, &snds1, &snds2, &snds3);
    }

    #[getter]
    pub fn snds_list(&self) -> SoundsList {
        SoundsList::new(self.pyxel_music.clone())
    }
}

pub fn add_music_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Sounds>()?;
    m.add_class::<SoundsList>()?;
    m.add_class::<Music>()?;
    Ok(())
}
