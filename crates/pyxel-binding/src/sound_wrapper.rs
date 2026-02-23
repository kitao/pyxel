use std::sync::Once;

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

static OLD_MML_ONCE: Once = Once::new();

macro_rules! wrap_sound_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $field_name:ident) => {
        wrap_as_python_sequence!(
            $wrapper_name,
            *mut pyxel::Sound,
            (|inner: &*mut pyxel::Sound| unsafe { &**inner }.$field_name.len()),
            $value_type,
            (|inner: &*mut pyxel::Sound, index| unsafe { &**inner }.$field_name[index]),
            $value_type,
            (|inner: &*mut pyxel::Sound, index, value| unsafe { &mut **inner }.$field_name
                [index] = value),
            Vec<$value_type>,
            (|inner: &*mut pyxel::Sound, list| unsafe { &mut **inner }.$field_name = list),
            (|inner: &*mut pyxel::Sound| unsafe { &**inner }
                .$field_name
                .iter()
                .copied()
                .collect::<Vec<$value_type>>())
        );
    };
}

wrap_sound_as_python_list!(Notes, pyxel::SoundNote, notes);
wrap_sound_as_python_list!(Tones, pyxel::SoundTone, tones);
wrap_sound_as_python_list!(Volumes, pyxel::SoundVolume, volumes);
wrap_sound_as_python_list!(Effects, pyxel::SoundEffect, effects);

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Sound {
    pub(crate) inner: *mut pyxel::Sound,
}

unsafe impl Send for Sound {}
unsafe impl Sync for Sound {}

impl Sound {
    pub fn wrap(inner: *mut pyxel::Sound) -> Self {
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
        Notes::wrap(self.inner)
    }

    #[getter]
    pub fn tones(&self) -> Tones {
        Tones::wrap(self.inner)
    }

    #[getter]
    pub fn volumes(&self) -> Volumes {
        Volumes::wrap(self.inner)
    }

    #[getter]
    pub fn effects(&self) -> Effects {
        Effects::wrap(self.inner)
    }

    #[getter]
    pub fn get_speed(&self) -> pyxel::SoundSpeed {
        unsafe { &*self.inner }.speed
    }

    #[setter]
    pub fn set_speed(&self, speed: pyxel::SoundSpeed) {
        unsafe { &mut *self.inner }.speed = speed;
    }

    pub fn set(
        &self,
        notes: &str,
        tones: &str,
        volumes: &str,
        effects: &str,
        speed: pyxel::SoundSpeed,
    ) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .set(notes, tones, volumes, effects, speed)
            .map_err(PyException::new_err)
    }

    pub fn set_notes(&self, notes: &str) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .set_notes(notes)
            .map_err(PyException::new_err)
    }

    pub fn set_tones(&self, tones: &str) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .set_tones(tones)
            .map_err(PyException::new_err)
    }

    pub fn set_volumes(&self, volumes: &str) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .set_volumes(volumes)
            .map_err(PyException::new_err)
    }

    pub fn set_effects(&self, effects: &str) -> PyResult<()> {
        unsafe { &mut *self.inner }
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

                return unsafe { &mut *self.inner }
                    .old_mml(code)
                    .map_err(PyException::new_err);
            }

            unsafe { &mut *self.inner }
                .set_mml(code)
                .map_err(PyException::new_err)
        } else {
            unsafe { &mut *self.inner }.clear_mml();
            Ok(())
        }
    }

    #[pyo3(signature = (code=None))]
    pub fn old_mml(&self, code: Option<&str>) -> PyResult<()> {
        OLD_MML_ONCE.call_once(|| {
            println!("Sound.old_mml(code) is deprecated. Use Sound.mml(code) instead.");
        });

        if let Some(code) = code {
            unsafe { &mut *self.inner }
                .old_mml(code)
                .map_err(PyException::new_err)
        } else {
            unsafe { &mut *self.inner }.clear_mml();
            Ok(())
        }
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    pub fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        unsafe { &mut *self.inner }
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    #[pyo3(signature = (filename=None))]
    pub fn pcm(&self, filename: Option<&str>) -> PyResult<()> {
        if let Some(filename) = filename {
            unsafe { &mut *self.inner }
                .load_pcm(filename)
                .map_err(PyException::new_err)
        } else {
            unsafe { &mut *self.inner }.clear_pcm();
            Ok(())
        }
    }

    pub fn total_sec(&self) -> Option<f32> {
        unsafe { &*self.inner }.total_seconds()
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
