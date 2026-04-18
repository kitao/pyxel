use pyo3::exceptions::PyException;
use pyo3::prelude::*;

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

define_wrapper!(Sound, pyxel::Sound);

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
        Notes::wrap(self.inner)
    }

    #[getter]
    fn tones(&self) -> Tones {
        Tones::wrap(self.inner)
    }

    #[getter]
    fn volumes(&self) -> Volumes {
        Volumes::wrap(self.inner)
    }

    #[getter]
    fn effects(&self) -> Effects {
        Effects::wrap(self.inner)
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

        // Detect old MML syntax by presence of 'x'/'X' or '~'
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

    // File operations

    #[pyo3(signature = (filename, sec, ffmpeg=None))]
    fn save(&self, filename: &str, sec: f32, ffmpeg: Option<bool>) -> PyResult<()> {
        self.inner_mut()
            .save(filename, sec, ffmpeg)
            .map_err(PyException::new_err)
    }

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

    fn total_sec(&self) -> Option<f32> {
        self.inner_ref().total_seconds()
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
