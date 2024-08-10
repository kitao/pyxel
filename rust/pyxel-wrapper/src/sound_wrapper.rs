use pyo3::prelude::*;

macro_rules! wrap_sound_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $field_name:ident) => {
        wrap_as_python_list!(
            $wrapper_name,
            pyxel::SharedSound,
            (|inner: &pyxel::SharedSound| inner.lock().$field_name.len()),
            $value_type,
            (|inner: &pyxel::SharedSound, index| inner.lock().$field_name[index]),
            $value_type,
            (|inner: &pyxel::SharedSound, index, value| inner.lock().$field_name[index] = value),
            Vec<$value_type>,
            (|inner: &pyxel::SharedSound, list| inner.lock().$field_name = list),
            (|inner: &pyxel::SharedSound| inner
                .lock()
                .$field_name
                .iter()
                .map(|value| *value)
                .collect())
        );
    };
}

wrap_sound_as_python_list!(Notes, pyxel::Note, notes);
wrap_sound_as_python_list!(Tones, u32, tones);
wrap_sound_as_python_list!(Volumes, pyxel::Volume, volumes);
wrap_sound_as_python_list!(Effects, pyxel::Effect, effects);

#[pyclass]
#[derive(Clone)]
pub struct Sound {
    pub(crate) inner: pyxel::SharedSound,
}

impl Sound {
    pub fn wrap(inner: pyxel::SharedSound) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl Sound {
    #[new]
    pub fn new() -> Self {
        Self::wrap(pyxel::Sound::new())
    }

    #[getter]
    pub fn notes(&self) -> Notes {
        Notes::wrap(self.inner.clone())
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones::wrap(self.inner.clone())
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes::wrap(self.inner.clone())
    }

    #[getter]
    pub fn effects(&self) -> Effects {
        Effects::wrap(self.inner.clone())
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

pub fn add_sound_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Notes>()?;
    m.add_class::<Tones>()?;
    m.add_class::<Volumes>()?;
    m.add_class::<Effects>()?;
    m.add_class::<Sound>()?;
    Ok(())
}
