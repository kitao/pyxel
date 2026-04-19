use pyo3::prelude::*;

wrap_as_python_sequence!(
    Wavetable,
    *mut pyxel::Tone,
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.len()),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index| unsafe { &**inner }.wavetable[index]),
    pyxel::ToneSample,
    (|inner: &*mut pyxel::Tone, index, value| unsafe { &mut **inner }.wavetable[index] = value),
    (|inner: &*mut pyxel::Tone| -> &mut Vec<pyxel::ToneSample> {
        &mut unsafe { &mut **inner }.wavetable
    }),
    Vec<pyxel::ToneSample>,
    (|inner: &*mut pyxel::Tone, list| unsafe { &mut **inner }.wavetable = list),
    (|inner: &*mut pyxel::Tone| unsafe { &**inner }.wavetable.clone())
);

define_wrapper!(Tone, pyxel::Tone);

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
    fn sample_bits(&self) -> u32 {
        self.inner_ref().sample_bits
    }

    #[setter]
    fn set_sample_bits(&self, sample_bits: u32) {
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
