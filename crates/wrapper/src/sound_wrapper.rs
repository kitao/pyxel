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
    fn new(pyxel_sound: PyxelSharedSound) -> Notes {
        Notes { pyxel_sound }
    }

    fn list(&self) -> &[Note] {
        unsafe { &*(&self.pyxel_sound.lock().notes as *const Vec<Note>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Note> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().notes as *mut Vec<Note>) }
    }

    define_list_index_method!();
}

#[pyproto]
impl PySequenceProtocol for Notes {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(self, Notes::list)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Note> {
        define_list_get_operator!(self, Notes::list, index)
    }

    fn __setitem__(&mut self, index: isize, value: Note) -> PyResult<()> {
        define_list_set_operator!(self, Notes::list_mut, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(self, Notes::list_mut, index)
    }
}

#[pymethods]
impl Notes {
    define_list_edit_methods!(Note);
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: PyxelSharedSound,
}

impl Tones {
    fn new(pyxel_sound: PyxelSharedSound) -> Tones {
        Tones { pyxel_sound }
    }

    fn list(&self) -> &[Tone] {
        unsafe { &*(&self.pyxel_sound.lock().tones as *const Vec<Tone>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Tone> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().tones as *mut Vec<Tone>) }
    }

    define_list_index_method!();
}

#[pyproto]
impl PySequenceProtocol for Tones {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(self, Tones::list)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Tone> {
        define_list_get_operator!(self, Tones::list, index)
    }

    fn __setitem__(&mut self, index: isize, value: Tone) -> PyResult<()> {
        define_list_set_operator!(self, Tones::list_mut, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(self, Tones::list_mut, index)
    }
}

#[pymethods]
impl Tones {
    define_list_edit_methods!(Tone);
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: PyxelSharedSound,
}

impl Volumes {
    fn new(pyxel_sound: PyxelSharedSound) -> Volumes {
        Volumes { pyxel_sound }
    }

    fn list(&self) -> &[Volume] {
        unsafe { &*(&self.pyxel_sound.lock().volumes as *const Vec<Volume>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Volume> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().volumes as *mut Vec<Volume>) }
    }

    define_list_index_method!();
}

#[pyproto]
impl PySequenceProtocol for Volumes {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(self, Volumes::list)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Volume> {
        define_list_get_operator!(self, Volumes::list, index)
    }

    fn __setitem__(&mut self, index: isize, value: Volume) -> PyResult<()> {
        define_list_set_operator!(self, Volumes::list_mut, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(self, Volumes::list_mut, index)
    }
}

#[pymethods]
impl Volumes {
    define_list_edit_methods!(Volume);
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: PyxelSharedSound,
}

impl Effects {
    fn new(pyxel_sound: PyxelSharedSound) -> Effects {
        Effects { pyxel_sound }
    }

    fn list(&self) -> &[Effect] {
        unsafe { &*(&self.pyxel_sound.lock().effects as *const Vec<Effect>) }
    }

    fn list_mut(&mut self) -> &mut Vec<Effect> {
        unsafe { &mut *(&mut self.pyxel_sound.lock().effects as *mut Vec<Effect>) }
    }

    define_list_index_method!();
}

#[pyproto]
impl PySequenceProtocol for Effects {
    fn __len__(&self) -> PyResult<usize> {
        define_list_len_operator!(self, Effects::list)
    }

    fn __getitem__(&self, index: isize) -> PyResult<Effect> {
        define_list_get_operator!(self, Effects::list, index)
    }

    fn __setitem__(&mut self, index: isize, value: Effect) -> PyResult<()> {
        define_list_set_operator!(self, Effects::list_mut, index, value)
    }

    fn __delitem__(&mut self, index: isize) -> PyResult<()> {
        define_list_del_operator!(self, Effects::list_mut, index)
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
    pub fn new() -> PyResult<Sound> {
        Ok(wrap_pyxel_sound(PyxelSound::new()))
    }

    #[getter]
    pub fn notes(&self) -> PyResult<Notes> {
        Ok(Notes::new(self.pyxel_sound.clone()))
    }

    #[getter]
    pub fn tones(&self) -> PyResult<Tones> {
        Ok(Tones::new(self.pyxel_sound.clone()))
    }

    #[getter]
    pub fn volumes(&self) -> PyResult<Volumes> {
        Ok(Volumes::new(self.pyxel_sound.clone()))
    }

    #[getter]
    pub fn effects(&self) -> PyResult<Effects> {
        Ok(Effects::new(self.pyxel_sound.clone()))
    }

    #[getter]
    pub fn get_speed(&self) -> PyResult<Speed> {
        Ok(self.pyxel_sound.lock().speed)
    }

    #[setter]
    pub fn set_speed(&self, speed: Speed) -> PyResult<()> {
        self.pyxel_sound.lock().speed = speed;

        Ok(())
    }

    pub fn set(
        &self,
        notes: &str,
        tones: &str,
        volumes: &str,
        effects: &str,
        speed: Speed,
    ) -> PyResult<()> {
        self.pyxel_sound
            .lock()
            .set(notes, tones, volumes, effects, speed);

        Ok(())
    }

    pub fn set_notes(&self, notes: &str) -> PyResult<()> {
        self.pyxel_sound.lock().set_notes(notes);

        Ok(())
    }

    pub fn set_tones(&self, tones: &str) -> PyResult<()> {
        self.pyxel_sound.lock().set_tones(tones);

        Ok(())
    }

    pub fn set_volumes(&self, volumes: &str) -> PyResult<()> {
        self.pyxel_sound.lock().set_volumes(volumes);

        Ok(())
    }

    pub fn set_effects(&self, effects: &str) -> PyResult<()> {
        self.pyxel_sound.lock().set_effects(effects);

        Ok(())
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
