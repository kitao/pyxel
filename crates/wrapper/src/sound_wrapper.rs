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

#[pyproto]
impl PySequenceProtocol for Notes {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.pyxel_sound.lock().notes.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Note> {
        Ok(self.pyxel_sound.lock().notes[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, note: Note) -> PyResult<()> {
        self.pyxel_sound.lock().notes[idx as usize] = note;

        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Tones {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.pyxel_sound.lock().tones.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Tone> {
        Ok(self.pyxel_sound.lock().tones[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, tone: Tone) -> PyResult<()> {
        self.pyxel_sound.lock().tones[idx as usize] = tone;

        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Volumes {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.pyxel_sound.lock().volumes.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Volume> {
        Ok(self.pyxel_sound.lock().volumes[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, volume: Volume) -> PyResult<()> {
        self.pyxel_sound.lock().volumes[idx as usize] = volume;

        Ok(())
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Effects {
    fn __len__(&self) -> PyResult<usize> {
        Ok(self.pyxel_sound.lock().effects.len())
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Effect> {
        Ok(self.pyxel_sound.lock().effects[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, effect: Tone) -> PyResult<()> {
        self.pyxel_sound.lock().effects[idx as usize] = effect;

        Ok(())
    }
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
        Ok(Notes {
            pyxel_sound: self.pyxel_sound.clone(),
        })
    }

    #[getter]
    pub fn tones(&self) -> PyResult<Tones> {
        Ok(Tones {
            pyxel_sound: self.pyxel_sound.clone(),
        })
    }

    #[getter]
    pub fn volumes(&self) -> PyResult<Volumes> {
        Ok(Volumes {
            pyxel_sound: self.pyxel_sound.clone(),
        })
    }

    #[getter]
    pub fn effects(&self) -> PyResult<Effects> {
        Ok(Effects {
            pyxel_sound: self.pyxel_sound.clone(),
        })
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
