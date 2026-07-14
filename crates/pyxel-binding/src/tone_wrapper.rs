use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::utils::MutexFieldMut;

fn wavetable_mut(inner: &pyxel::RcTone) -> MutexFieldMut<'_, pyxel::Tone, Vec<pyxel::ToneSample>> {
    MutexFieldMut::new(
        audio_mut!(inner),
        |tone| &tone.wavetable,
        |tone| &mut tone.wavetable,
    )
}

// Python sequence wrapper for the mutable wavetable

wrap_as_python_primitive_sequence!(
    Wavetable,
    pyxel::RcTone,
    (|inner: &pyxel::RcTone| audio_ref!(inner).wavetable.len()),
    pyxel::ToneSample,
    (|inner: &pyxel::RcTone, index| audio_ref!(inner).wavetable[index]),
    pyxel::ToneSample,
    (|inner: &pyxel::RcTone, index, value| audio_mut!(inner).wavetable[index] = value),
    wavetable_mut,
    Vec<pyxel::ToneSample>,
    (|inner: &pyxel::RcTone, list| audio_mut!(inner).wavetable = list),
    (|inner: &pyxel::RcTone| audio_ref!(inner).wavetable.clone())
);

define_audio_wrapper!(Tone, pyxel::Tone, pyxel::RcTone);

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
    fn set_sample_bits(&self, sample_bits: u32) -> PyResult<()> {
        if !(1..=pyxel::AUDIO_SAMPLE_BITS).contains(&sample_bits) {
            return Err(PyValueError::new_err(format!(
                "sample_bits must be between 1 and {}",
                pyxel::AUDIO_SAMPLE_BITS
            )));
        }
        self.inner_mut().sample_bits = sample_bits;
        Ok(())
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
