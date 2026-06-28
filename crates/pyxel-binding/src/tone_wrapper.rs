use pyo3::prelude::*;

// Python sequence wrapper for the mutable wavetable

wrap_as_python_primitive_sequence!(
    Wavetable,
    pyxel::RcTone,
    (|inner: &pyxel::RcTone| rc_ref!(inner).wavetable.len()),
    pyxel::ToneSample,
    (|inner: &pyxel::RcTone, index| rc_ref!(inner).wavetable[index]),
    pyxel::ToneSample,
    (|inner: &pyxel::RcTone, index, value| rc_mut!(inner).wavetable[index] = value),
    (|inner: &pyxel::RcTone| -> &mut Vec<pyxel::ToneSample> { &mut rc_mut!(inner).wavetable }),
    Vec<pyxel::ToneSample>,
    (|inner: &pyxel::RcTone, list| rc_mut!(inner).wavetable = list),
    (|inner: &pyxel::RcTone| rc_ref!(inner).wavetable.clone())
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
        Wavetable::wrap(self.inner.clone())
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
        Wavetable::wrap(self.inner.clone())
    }
}

// Module registration

pub fn add_tone_class(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Wavetable>()?;
    m.add_class::<Tone>()?;
    Ok(())
}
