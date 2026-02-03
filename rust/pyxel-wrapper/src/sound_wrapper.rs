use std::sync::Once;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

static OLD_MML_ONCE: Once = Once::new();

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
                .collect::<Vec<$value_type>>())
        );
    };
}

wrap_sound_as_python_list!(Notes, pyxel::SoundNote, notes);
wrap_sound_as_python_list!(Tones, pyxel::SoundTone, tones);
wrap_sound_as_python_list!(Volumes, pyxel::SoundVolume, volumes);
wrap_sound_as_python_list!(Effects, pyxel::SoundEffect, effects);

#[pyclass(from_py_object)]
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
    pub fn get_speed(&self) -> pyxel::SoundSpeed {
        self.inner.lock().speed
    }

    #[setter]
    pub fn set_speed(&self, speed: pyxel::SoundSpeed) {
        self.inner.lock().speed = speed;
    }

    pub fn set(
        &self,
        notes: &str,
        tones: &str,
        volumes: &str,
        effects: &str,
        speed: pyxel::SoundSpeed,
    ) -> PyResult<()> {
        self.inner
            .lock()
            .set(notes, tones, volumes, effects, speed)
            .map_err(PyException::new_err)
    }

    pub fn set_notes(&self, notes: &str) -> PyResult<()> {
        self.inner
            .lock()
            .set_notes(notes)
            .map_err(PyException::new_err)
    }

    pub fn set_tones(&self, tones: &str) -> PyResult<()> {
        self.inner
            .lock()
            .set_tones(tones)
            .map_err(PyException::new_err)
    }

    pub fn set_volumes(&self, volumes: &str) -> PyResult<()> {
        self.inner
            .lock()
            .set_volumes(volumes)
            .map_err(PyException::new_err)
    }

    pub fn set_effects(&self, effects: &str) -> PyResult<()> {
        self.inner
            .lock()
            .set_effects(effects)
            .map_err(PyException::new_err)
    }

    #[pyo3(signature = (code=None))]
    pub fn mml(&self, code: Option<&str>) -> PyResult<()> {
        if let Some(code) = code {
            if code.contains('x') || code.contains('X') || code.contains('~') {
                OLD_MML_ONCE.call_once(|| {
                    println!("Old MML syntax is deprecated. Use new syntax instead.");
                });

                return self
                    .inner
                    .lock()
                    .old_mml(code)
                    .map_err(PyException::new_err);
            }

            self.inner.lock().mml(code).map_err(PyException::new_err)
        } else {
            self.inner.lock().mml0();
            Ok(())
        }
    }

    #[pyo3(signature = (code=None))]
    pub fn old_mml(&self, code: Option<&str>) -> PyResult<()> {
        OLD_MML_ONCE.call_once(|| {
            println!("Sound.old_mml(code) is deprecated. Use Sound.mml(code) instead.");
        });

        if let Some(code) = code {
            self.inner
                .lock()
                .old_mml(code)
                .map_err(PyException::new_err)
        } else {
            self.inner.lock().mml0();
            Ok(())
        }
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    pub fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        self.inner
            .lock()
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    pub fn total_sec(&self) -> Option<f32> {
        self.inner.lock().total_sec()
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
