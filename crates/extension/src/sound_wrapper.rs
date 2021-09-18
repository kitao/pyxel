use pyo3::class::PySequenceProtocol;
use pyo3::prelude::*;
use pyxel::SharedSound as PyxelSharedSound;
use pyxel::Sound as PyxelSound;
use pyxel::{Effect, Note, Speed, Tone, Volume};

#[pyclass]
#[derive(Clone)]
pub struct Notes {
    pyxel_sound: PyxelSharedSound,
}

impl Notes {
    fn new(pyxel_sound: PyxelSharedSound) -> Self {
        Self { pyxel_sound }
    }

    fn list(&self) -> &[Note] {
        unsafe { &*(&self.pyxel_sound.lock().notes as *const Vec<Note>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Note> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().notes as *mut Vec<Note>) }
    }
}

#[pymethods]
impl Notes {
    define_list_edit_methods!(Note);
}

#[pyproto]
impl PySequenceProtocol for Notes {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Note> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Note) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(Self::list_mut, self, index)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: PyxelSharedSound,
}

impl Tones {
    fn new(pyxel_sound: PyxelSharedSound) -> Self {
        Self { pyxel_sound }
    }

    fn list(&self) -> &[Tone] {
        unsafe { &*(&self.pyxel_sound.lock().tones as *const Vec<Tone>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Tone> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().tones as *mut Vec<Tone>) }
    }
}

#[pymethods]
impl Tones {
    define_list_edit_methods!(Tone);
}

#[pyproto]
impl PySequenceProtocol for Tones {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Tone> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Tone) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(Self::list_mut, self, index)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: PyxelSharedSound,
}

impl Volumes {
    fn new(pyxel_sound: PyxelSharedSound) -> Self {
        Self { pyxel_sound }
    }

    fn list(&self) -> &[Volume] {
        unsafe { &*(&self.pyxel_sound.lock().volumes as *const Vec<Volume>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Volume> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().volumes as *mut Vec<Volume>) }
    }
}

#[pymethods]
impl Volumes {
    define_list_edit_methods!(Volume);
}

#[pyproto]
impl PySequenceProtocol for Volumes {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Volume> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Volume) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(Self::list_mut, self, index)
    }
}

#[pyproto]
impl PySequenceProtocol for Effects {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(Self::list, self)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Effect> {
        define_list_get_operator!(Self::list, self, index)
    }

    fn __setitem__(&mut self, index: isize, value: Effect) -> PyResult<()> {
        define_list_set_operator!(Self::list_mut, self, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(Self::list_mut, self, index)
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: PyxelSharedSound,
}

impl Effects {
    fn new(pyxel_sound: PyxelSharedSound) -> Self {
        Self { pyxel_sound }
    }

    fn list(&self) -> &[Effect] {
        unsafe { &*(&self.pyxel_sound.lock().effects as *const Vec<Effect>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Effect> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().effects as *mut Vec<Effect>) }
    }
}

#[pymethods]
impl Effects {
    define_list_edit_methods!(Effect);
}

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub pyxel_sound: PyxelSharedSound,
}

pub fn wrap_pyxel_sound(pyxel_sound: PyxelSharedSound) -> Sound {
    Sound { pyxel_sound }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Self {
        wrap_pyxel_sound(PyxelSound::new())
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
    pub fn get_speed(&self) -> Speed {
        self.pyxel_sound.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: Speed) {
        self.pyxel_sound.lock().speed = speed;
    }

    pub fn set(&self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: Speed) {
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
