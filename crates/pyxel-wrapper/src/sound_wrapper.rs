use pyo3::prelude::*;

wrap_as_python_list!(
    Notes,
    pyxel::Note,
    pyxel::SharedSound,
    (|inner: &pyxel::SharedSound| inner.lock().notes.len()),
    (|inner: &pyxel::SharedSound, index| inner.lock().notes[index]),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().notes[index] = value),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().notes.insert(index, value)),
    (|inner: &pyxel::SharedSound, index| inner.lock().notes.remove(index))
);

wrap_as_python_list!(
    Tones,
    pyxel::Tone,
    pyxel::SharedSound,
    (|inner: &pyxel::SharedSound| inner.lock().tones.len()),
    (|inner: &pyxel::SharedSound, index| inner.lock().tones[index]),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().tones[index] = value),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().tones.insert(index, value)),
    (|inner: &pyxel::SharedSound, index| inner.lock().tones.remove(index))
);

wrap_as_python_list!(
    Volumes,
    pyxel::Volume,
    pyxel::SharedSound,
    (|inner: &pyxel::SharedSound| inner.lock().volumes.len()),
    (|inner: &pyxel::SharedSound, index| inner.lock().volumes[index]),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().volumes[index] = value),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().volumes.insert(index, value)),
    (|inner: &pyxel::SharedSound, index| inner.lock().volumes.remove(index))
);

wrap_as_python_list!(
    Effects,
    pyxel::Effect,
    pyxel::SharedSound,
    (|inner: &pyxel::SharedSound| inner.lock().effects.len()),
    (|inner: &pyxel::SharedSound, index| inner.lock().effects[index]),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().effects[index] = value),
    (|inner: &pyxel::SharedSound, index, value| inner.lock().effects.insert(index, value)),
    (|inner: &pyxel::SharedSound, index| inner.lock().effects.remove(index))
);

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub(crate) inner: pyxel::SharedSound,
}

impl Sound {
    pub fn from(inner: pyxel::SharedSound) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Self {
        Self {
            inner: pyxel::Sound::new(),
        }
    }

    #[getter]
    pub fn notes(&self) -> Notes {
        Notes {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn effects(&self) -> Effects {
        Effects {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn get_speed(&self) -> pyxel::Speed {
        self.inner.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: pyxel::Speed) {
        self.inner.lock().speed = speed;
    }

    pub fn set(&self, notes: &str, tones: &str, volumes: &str, effects: &str, speed: pyxel::Speed) {
        self.inner.lock().set(notes, tones, volumes, effects, speed);
    }

    pub fn set_notes(&self, notes: &str) {
        self.inner.lock().set_notes(notes);
    }

    pub fn set_tones(&self, tones: &str) {
        self.inner.lock().set_tones(tones);
    }

    pub fn set_volumes(&self, volumes: &str) {
        self.inner.lock().set_volumes(volumes);
    }

    pub fn set_effects(&self, effects: &str) {
        self.inner.lock().set_effects(effects);
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
