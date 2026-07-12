use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use crate::utils::MutexFieldMut;

fn notes_mut(inner: &pyxel::RcSound) -> MutexFieldMut<'_, pyxel::Sound, Vec<pyxel::SoundNote>> {
    MutexFieldMut::new(
        audio_mut!(inner),
        |sound| &sound.notes,
        |sound| &mut sound.notes,
    )
}

fn tones_mut(inner: &pyxel::RcSound) -> MutexFieldMut<'_, pyxel::Sound, Vec<pyxel::SoundTone>> {
    MutexFieldMut::new(
        audio_mut!(inner),
        |sound| &sound.tones,
        |sound| &mut sound.tones,
    )
}

fn volumes_mut(inner: &pyxel::RcSound) -> MutexFieldMut<'_, pyxel::Sound, Vec<pyxel::SoundVolume>> {
    MutexFieldMut::new(
        audio_mut!(inner),
        |sound| &sound.volumes,
        |sound| &mut sound.volumes,
    )
}

fn effects_mut(inner: &pyxel::RcSound) -> MutexFieldMut<'_, pyxel::Sound, Vec<pyxel::SoundEffect>> {
    MutexFieldMut::new(
        audio_mut!(inner),
        |sound| &sound.effects,
        |sound| &mut sound.effects,
    )
}

// Python sequence wrappers for mutable sound component lists

macro_rules! wrap_sound_as_python_list {
    ($wrapper_name:ident, $value_type:ty, $field_name:ident, $field_mut:ident) => {
        wrap_as_python_primitive_sequence!(
            $wrapper_name,
            pyxel::RcSound,
            (|inner: &pyxel::RcSound| audio_ref!(inner).$field_name.len()),
            $value_type,
            (|inner: &pyxel::RcSound, index| audio_ref!(inner).$field_name[index]),
            $value_type,
            (|inner: &pyxel::RcSound, index, value| audio_mut!(inner).$field_name[index] = value),
            $field_mut,
            Vec<$value_type>,
            (|inner: &pyxel::RcSound, list| audio_mut!(inner).$field_name = list),
            (|inner: &pyxel::RcSound| audio_ref!(inner).$field_name.clone())
        );
    };
}

wrap_sound_as_python_list!(Notes, pyxel::SoundNote, notes, notes_mut);
wrap_sound_as_python_list!(Tones, pyxel::SoundTone, tones, tones_mut);
wrap_sound_as_python_list!(Volumes, pyxel::SoundVolume, volumes, volumes_mut);
wrap_sound_as_python_list!(Effects, pyxel::SoundEffect, effects, effects_mut);

define_audio_wrapper!(Sound, pyxel::Sound, pyxel::RcSound);

#[pymethods]
impl Sound {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::Sound::new())
    }

    // Sequence properties

    #[getter]
    fn notes(&self) -> Notes {
        Notes::wrap(self.inner.clone())
    }

    #[getter]
    fn tones(&self) -> Tones {
        Tones::wrap(self.inner.clone())
    }

    #[getter]
    fn volumes(&self) -> Volumes {
        Volumes::wrap(self.inner.clone())
    }

    #[getter]
    fn effects(&self) -> Effects {
        Effects::wrap(self.inner.clone())
    }

    #[getter]
    fn speed(&self) -> pyxel::SoundSpeed {
        self.inner_ref().speed
    }

    #[setter]
    fn set_speed(&self, speed: pyxel::SoundSpeed) {
        self.inner_mut().speed = speed;
    }

    // Data operations

    fn set(
        &self,
        notes: &str,
        tones: &str,
        volumes: &str,
        effects: &str,
        speed: pyxel::SoundSpeed,
    ) -> PyResult<()> {
        self.inner_mut()
            .set(notes, tones, volumes, effects, speed)
            .map_err(PyException::new_err)
    }

    fn set_notes(&self, notes: &str) -> PyResult<()> {
        self.inner_mut()
            .set_notes(notes)
            .map_err(PyException::new_err)
    }

    fn set_tones(&self, tones: &str) -> PyResult<()> {
        self.inner_mut()
            .set_tones(tones)
            .map_err(PyException::new_err)
    }

    fn set_volumes(&self, volumes: &str) -> PyResult<()> {
        self.inner_mut()
            .set_volumes(volumes)
            .map_err(PyException::new_err)
    }

    fn set_effects(&self, effects: &str) -> PyResult<()> {
        self.inner_mut()
            .set_effects(effects)
            .map_err(PyException::new_err)
    }

    // MML

    #[pyo3(signature = (code=None))]
    fn mml(&self, code: Option<&str>) -> PyResult<()> {
        let Some(code) = code else {
            self.inner_mut().clear_mml();
            return Ok(());
        };

        // Detect old MML syntax by the presence of 'x'/'X' or '~'.
        if code.contains('x') || code.contains('X') || code.contains('~') {
            deprecation_warning!(
                OLD_MML_ONCE,
                "Old MML syntax is deprecated. Use new syntax instead."
            );
            return self.inner_mut().old_mml(code).map_err(PyException::new_err);
        }

        self.inner_mut().set_mml(code).map_err(PyException::new_err)
    }

    #[pyo3(signature = (code=None))]
    fn old_mml(&self, code: Option<&str>) -> PyResult<()> {
        deprecation_warning!(
            OLD_MML_FUNC_ONCE,
            "Sound.old_mml(code) is deprecated. Use Sound.mml(code) instead."
        );

        let Some(code) = code else {
            self.inner_mut().clear_mml();
            return Ok(());
        };

        self.inner_mut().old_mml(code).map_err(PyException::new_err)
    }

    // PCM file operations

    #[pyo3(signature = (filename=None))]
    fn pcm(&self, filename: Option<&str>) -> PyResult<()> {
        let Some(filename) = filename else {
            self.inner_mut().clear_pcm();
            return Ok(());
        };

        self.inner_mut()
            .load_pcm(filename)
            .map_err(PyException::new_err)
    }

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        let sound = self.inner_ref().clone();
        sound
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

    // Playback duration

    fn total_sec(&self) -> Option<f32> {
        self.inner_ref().total_seconds()
    }
}

// Module registration

pub fn add_sound_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Notes>()?;
    m.add_class::<Tones>()?;
    m.add_class::<Volumes>()?;
    m.add_class::<Effects>()?;
    m.add_class::<Sound>()?;
    Ok(())
}
