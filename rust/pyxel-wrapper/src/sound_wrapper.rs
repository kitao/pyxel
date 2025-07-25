use std::sync::Once;

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
    ) {
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

    #[pyo3(signature = (code=None))]
    pub fn mml(&self, code: Option<&str>) {
        if let Some(code) = code {
            if code.contains('x') || code.contains('X') {
                OLD_MML_ONCE.call_once(|| {
                    println!("Old MML syntax is deprecated. Use new syntax instead.");
                });

                self.inner.lock().old_mml(code);
                return;
            }

            self.inner.lock().mml(code);
        } else {
            self.inner.lock().mml0();
        }
    }

    #[pyo3(signature = (code=None))]
    pub fn old_mml(&self, code: Option<&str>) {
        OLD_MML_ONCE.call_once(|| {
            println!("Sound.old_mml(code) is deprecated. Use Sound.mml(code) instead.");
        });

        if let Some(code) = code {
            self.inner.lock().old_mml(code);
        } else {
            self.inner.lock().mml0();
        }
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    pub fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) {
        self.inner.lock().save(filename, sec, ffmpeg);
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
