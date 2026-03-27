use pyo3::prelude::*;

wrap_as_python_sequence!(
    Wavetable,
    *mut pyxel::Tone,
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.len()),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index| unsafe { &**inner }.wavetable[index]),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index, value| unsafe { &mut **inner }.wavetable[index] = value),
    Vec<pyxel::ToneSample>,
    (|inner: &*mut pyxel::Tone, list| unsafe { &mut **inner }.wavetable = list),
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.clone())
);

#[pyclass(from_py_object)]
#[derive(Clone, Copy)]
pub struct Tone {
    pub(crate) inner: *mut pyxel::Tone,
}

unsafe impl Send for Tone {}
unsafe impl Sync for Tone {}

impl Tone {
    pub fn wrap(inner: *mut pyxel::Tone) -> Self {
        Self { inner }
    }

    fn inner_ref(&self) -> &pyxel::Tone {
        unsafe { &*self.inner }
    }

    #[allow(clippy::mut_from_ref)]
    fn inner_mut(&self) -> &mut pyxel::Tone {
        unsafe { &mut *self.inner }
    }
}

#[pymethods]
impl Tone {
    // Constructor

    #[new]
    fn new() -> Self {
        Self::wrap(pyxel::Tone::new())
    }

    // Properties

    #[getter]
    fn mode(&self) -> u32 {
        self.inner_ref().mode.into()
    }

    #[setter]
    fn set_mode(&self, mode: u32) {
        self.inner_mut().mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    fn sample_bits(&self) -> pyxel::ToneSample {
        self.inner_ref().sample_bits
    }

    #[setter]
    fn set_sample_bits(&self, sample_bits: pyxel::ToneSample) {
        self.inner_mut().sample_bits = sample_bits;
    }

    #[getter]
    fn wavetable(&self) -> Wavetable {
        Wavetable::wrap(self.inner)
    }

    #[getter]
    fn gain(&self) -> pyxel::ToneGain {
        self.inner_ref().gain
    }

    #[setter]
    fn set_gain(&self, gain: pyxel::ToneGain) {
        self.inner_mut().gain = gain;
    }

    // Deprecated properties

    #[getter]
    fn noise(&self) -> u32 {
        deprecation_warning!(
            NOISE_ONCE,
            "Tone.noise is deprecated. Use Tone.mode instead."
        );
        self.inner_ref().mode.into()
    }

    #[setter]
    fn set_noise(&self, mode: u32) {
        deprecation_warning!(
            SET_NOISE_ONCE,
            "Tone.noise is deprecated. Use Tone.mode instead."
        );
        self.inner_mut().mode = pyxel::ToneMode::from(mode);
    }

    #[getter]
    fn waveform(&self) -> Wavetable {
        deprecation_warning!(
            WAVEFORM_ONCE,
            "Tone.waveform is deprecated. Use Tone.wavetable instead."
        );
        Wavetable::wrap(self.inner)
    }
}

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Tone>()?;
    Ok(())
}
