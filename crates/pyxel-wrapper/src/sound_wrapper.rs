use pyo3::prelude::*;
use pyxel_engine as pyxel;

macro_rules! define_private_methods_for_list {
    ($type: ty, $elems: ident) => {
        fn new(pyxel_sound: pyxel::SharedSound) -> Self {
            Self { pyxel_sound }
        }

        fn list(&self) -> &[$type] {
            unsafe { &*(&self.pyxel_sound.lock().$elems as *const Vec<$type>) }
        }

        fn list_mut(&mut self) -> &mut Vec<$type> {
            unsafe { &mut *(&mut self.pyxel_sound.lock().$elems as *mut Vec<$type>) }
        }
    };
}

#[pyclass]
#[derive(Clone)]
pub struct Notes {
    pyxel_sound: pyxel::SharedSound,
}

impl Notes {
    define_private_methods_for_list!(pyxel::Note, notes);
}

#[pymethods]
impl Notes {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<pyxel::Note> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: pyxel::Note) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<pyxel::Note>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<pyxel::Note>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: pyxel::SharedSound,
}

impl Tones {
    define_private_methods_for_list!(pyxel::Tone, tones);
}

#[pymethods]
impl Tones {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<pyxel::Tone> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: pyxel::Tone) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<pyxel::Tone>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<pyxel::Tone>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: pyxel::SharedSound,
}

impl Volumes {
    define_private_methods_for_list!(pyxel::Volume, volumes);
}

#[pymethods]
impl Volumes {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<pyxel::Volume> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: pyxel::Volume) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<pyxel::Volume>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<pyxel::Volume>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: pyxel::SharedSound,
}

impl Effects {
    define_private_methods_for_list!(pyxel::Effect, effects);
}

#[pymethods]
impl Effects {
    fn __len__(&self) -> PyResult<usize> {
        impl_len_method_for_list!(self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<pyxel::Effect> {
        impl_getitem_method_for_list!(self, index)
    }

    fn __setitem__(&mut self, index: isize, value: pyxel::Effect) -> PyResult<()> {
        impl_setitem_method_for_list!(self, index, value)
    }

    pub fn from_list(&mut self, lst: Vec<pyxel::Effect>) -> PyResult<()> {
        impl_from_list_method_for_list!(self, lst)
    }

    pub fn to_list(&self) -> PyResult<Vec<pyxel::Effect>> {
        impl_to_list_method_for_list!(self)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub pyxel_sound: pyxel::SharedSound,
}

pub const fn wrap_pyxel_sound(pyxel_sound: pyxel::SharedSound) -> Sound {
    Sound { pyxel_sound }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Self {
        wrap_pyxel_sound(pyxel::Sound::new())
    }

    #[getter]
    pub fn notes(&self) -> Notes {
        Notes::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn effects(&self) -> Effects {
        Effects::new(self.pyxel_sound.clone())
    }

    #[getter]
    pub fn get_speed(&self) -> pyxel::Speed {
        self.pyxel_sound.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: pyxel::Speed) {
        self.pyxel_sound.lock().speed = speed;
    }

    pub fn set(&self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: pyxel::Speed) {
        self.pyxel_sound
            .lock()
            .set(notes, tones, volumes, effects, speed);
    }

    pub fn set_notes(&self, notes: &str) {
        self.pyxel_sound.lock().set_notes(notes);
    }

    pub fn set_tones(&self, tones: &str) {
        self.pyxel_sound.lock().set_tones(tones);
    }

    pub fn set_volumes(&self, volumes: &str) {
        self.pyxel_sound.lock().set_volumes(volumes);
    }

    pub fn set_effects(&self, effects: &str) {
        self.pyxel_sound.lock().set_effects(effects);
    }
}

pub fn add_sound_class(m: &PyModule) -> PyResult<()> {
    m.add_class::<Notes>()?;
    m.add_class::<Tones>()?;
    m.add_class::<Volumes>()?;
    m.add_class::<Effects>()?;
    m.add_class::<Sound>()?;
    Ok(())
}
