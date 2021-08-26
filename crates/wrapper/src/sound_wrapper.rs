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
    fn __len__(&self) -> usize {
        self.pyxel_sound.lock().notes.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Note> {
        Ok(self.pyxel_sound.lock().notes[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, note: Note) {
        self.pyxel_sound.lock().notes[idx as usize] = note;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Tones {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Tones {
    fn __len__(&self) -> usize {
        self.pyxel_sound.lock().tones.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Tone> {
        Ok(self.pyxel_sound.lock().tones[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, tone: Tone) {
        self.pyxel_sound.lock().tones[idx as usize] = tone;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Volumes {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Volumes {
    fn __len__(&self) -> usize {
        self.pyxel_sound.lock().volumes.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Volume> {
        Ok(self.pyxel_sound.lock().volumes[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, volume: Volume) {
        self.pyxel_sound.lock().volumes[idx as usize] = volume;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Effects {
    pyxel_sound: PyxelSharedSound,
}

#[pyproto]
impl PySequenceProtocol for Effects {
    fn __len__(&self) -> usize {
        self.pyxel_sound.lock().effects.len()
    }

    fn __getitem__(&self, idx: isize) -> PyResult<Effect> {
        Ok(self.pyxel_sound.lock().effects[idx as usize])
    }

    fn __setitem__(&mut self, idx: isize, effect: Tone) {
        self.pyxel_sound.lock().effects[idx as usize] = effect;
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub pyxel_sound: PyxelSharedSound,
}

pub fn wrap_pyxel_sound(pyxel_sound: PyxelSharedSound) -> Sound {
    Sound {
        pyxel_sound: pyxel_sound,
    }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Sound {
        wrap_pyxel_sound(PyxelSound::new())
    }

    #[getter]
    pub fn notes(&self) -> Notes {
        Notes {
            pyxel_sound: self.pyxel_sound.clone(),
        }
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones {
            pyxel_sound: self.pyxel_sound.clone(),
        }
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes {
            pyxel_sound: self.pyxel_sound.clone(),
        }
    }

    #[getter]
    pub fn effect(&self) -> Effects {
        Effects {
            pyxel_sound: self.pyxel_sound.clone(),
        }
    }

    #[getter]
    pub fn get_speed(&self) -> Speed {
        self.pyxel_sound.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: Speed) {
        self.pyxel_sound.lock().speed = speed;
    }

    pub fn set(
        &self,
        note_str: &str,
        tone_str: &str,
        volume_str: &str,
        effect_str: &str,
        speed: Speed,
    ) {
        self.pyxel_sound
            .lock()
            .set(note_str, tone_str, volume_str, effect_str, speed);
    }

    pub fn set_note(&self, note_str: &str) {
        self.pyxel_sound.lock().set_note(note_str);
    }

    pub fn set_tone(&self, tone_str: &str) {
        self.pyxel_sound.lock().set_tone(tone_str);
    }

    pub fn set_volume(&self, volume_str: &str) {
        self.pyxel_sound.lock().set_volume(volume_str);
    }

    pub fn set_effect(&self, effect_str: &str) {
        self.pyxel_sound.lock().set_effect(effect_str);
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
